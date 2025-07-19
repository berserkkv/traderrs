use axum::{
    extract::Path,
    http::StatusCode,
    routing::get,
    Router,
};
use rust_embed::RustEmbed;
use mime_guess::MimeGuess;
use tokio::net::TcpListener;

#[derive(RustEmbed)]
#[folder = "ui/build"]
struct Assets;

impl Assets {
    fn get_file(path: &str) -> Option<(Vec<u8>, String)> {
        Assets::get(path).map(|file| {
            let mime = MimeGuess::from_path(path)
                .first_or_octet_stream()
                .as_ref()
                .to_owned();
            (file.data.into_owned(), mime)
        })
    }

    fn router() -> Router {
        let mut router = Router::new();

        for file_path in Assets::iter() {
            let path_string = file_path.as_ref().to_string();

            // prepend `/` because Axum expects routes to start with '/'
            let route_path = format!("/{}", path_string);

            // clone for closure
            let path_for_closure = path_string.clone();

            let handler = get(move || {
                let path_for_closure = path_for_closure.clone();
                async move {
                    if let Some((content, mime)) = Assets::get_file(&path_for_closure) {
                        (StatusCode::OK, [("Content-Type", mime)], content)
                    } else {
                        (StatusCode::NOT_FOUND, [("Content-Type", "text/plain".to_string())], Vec::new())
                    }
                }
            });

            router = router.route(&route_path, handler.clone());

            // Handle /somepath/ route if the file is /somepath/index.html
            if path_string.ends_with("/index.html") {
                let prefix = path_string.strip_suffix("index.html").unwrap();
                let prefix_route = format!("/{}", prefix);
                router = router.route(&prefix_route, handler.clone());
            }
        }

        router
    }

}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Static assets router
    let assets_router = Assets::router();

    // Fallback route: serve index.html for any other path (for SPA)
    let fallback = get(|| async {
        if let Some((content, mime)) = Assets::get_file("index.html") {
            (StatusCode::OK, [("Content-Type", mime)], content)
        } else {
            (StatusCode::NOT_FOUND, [("Content-Type", "text/plain".to_string())], "index.html not found".into())
        }
    });


    // Combine router with fallback
    let app = assets_router.fallback(fallback);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
