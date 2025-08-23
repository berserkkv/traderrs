use crate::models::bot::Bot;
use crate::models::models::{BotStatistic, Container, Order, SharedVec, Statistic, StatisticResult, SystemInfo, TimeRange};
use crate::tools::sort_bot_statistics;
use crate::{api, tools};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::routing::{get, put};
use axum::{Extension, Json, Router};
use chrono::{DateTime, FixedOffset};
use mime_guess::MimeGuess;
use rust_embed::RustEmbed;
use std::collections::HashMap;
use std::sync::Arc;
use sysinfo::System;
use tower_http::cors::{Any, CorsLayer};

pub fn get_router(bots: Arc<SharedVec<Bot>>, container: Arc<Container>) -> Router {
    let started_time = tools::get_date(3);

    let assets_router = Assets::router();

    let fallback = get(|| async {
        if let Some((content, mime)) = Assets::get_file("index.html") {
            (StatusCode::OK, [("Content-Type", mime)], content)
        } else {
            (
                StatusCode::NOT_FOUND,
                [("Content-Type", "text/plain".to_string())],
                "index.html not found".into(),
            )
        }
    });

    let cors = CorsLayer::new()
      .allow_origin(Any)
      .allow_methods(Any)
      .allow_headers(Any);
    assets_router
      .route("/api/v1/bots", get(get_all_bot))
      .route("/api/v1/bots/{id}/orders", get(get_orders_by_id))
      .route("/api/v1/bots/reset", put(reset_bots))
      .route("/api/v1/system", get(api::get_system_usage))
      .route("/api/v1/bots/statistics", get(get_all_bot_statistics))
      .route("/api/v1/bots/{bot_name}/statistics", get(get_bot_statistics))
      .route("/api/v1/bots/{bot_name}/statistics/range", get(get_statistic_in_range))
      .layer(Extension(bots))
      .layer(Extension(started_time))
      .layer(Extension(container))
      .layer(cors)
      .fallback(fallback)
}

pub async fn get_system_usage(Extension(started_time): Extension<DateTime<FixedOffset>>) -> Json<SystemInfo> {
    let mut sys = System::new_all();

    tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;

    sys.refresh_cpu_all();


    let total = sys.total_memory();
    let used_memory = sys.used_memory();
    let memory_usage = used_memory * 100 / total;
    Json(SystemInfo {
        cpu_usage: sys.global_cpu_usage(),
        memory_usage,
        started_time,
    })
}

pub async fn get_all_bot(Extension(bots): Extension<Arc<SharedVec<Bot>>>) -> Json<Vec<Bot>> {
    unsafe {
        let bots = &mut *bots.0.get();
        let mut v = Vec::with_capacity(bots.len());
        for b in bots.iter() {
            v.push(b.clone());
        }

        tools::sort_bots(&mut v);
        Json(v)
    }
}

pub async fn get_orders_by_id(Path(id): Path<String>, Extension(container) : Extension<Arc<Container>>) -> Json<Vec<Order>> {
    let orders = container.repository.get_order_by_bot_name(id).unwrap();
    // orders.reverse();

    // let mut orders = Vec::new();
    // for i in 0..5 {
    //     orders.push(Order::dummy());
    // }
    // orders.get_mut(0).unwrap().pnl = 0.0;
    // orders.get_mut(1).unwrap().pnl = 3.0;
    // orders.get_mut(2).unwrap().pnl = -10.5;
    // orders.get_mut(3).unwrap().pnl = 20.5;
    // orders.get_mut(4).unwrap().pnl = -2.0;
    // orders.get_mut(0).unwrap().created_at = DateTime::parse_from_rfc3339("2025-08-21T10:00:00+03:00").unwrap();
    // orders.get_mut(1).unwrap().created_at = DateTime::parse_from_rfc3339("2025-08-21T10:10:00+03:00").unwrap();
    // orders.get_mut(2).unwrap().created_at = DateTime::parse_from_rfc3339("2025-08-21T10:20:00+03:00").unwrap();
    // orders.get_mut(3).unwrap().created_at = DateTime::parse_from_rfc3339("2025-08-21T10:30:00+03:00").unwrap();
    // orders.get_mut(4).unwrap().created_at = DateTime::parse_from_rfc3339("2025-08-21T10:40:00+03:00").unwrap();
    //


    Json(orders)
}

pub async fn reset_bots(Extension(bots): Extension<Arc<SharedVec<Bot>>>) {
    unsafe {
        let bots = &mut *bots.0.get();
        for b in bots.iter_mut() {
            b.reset();
        }
    }
}

pub async fn get_all_bot_statistics(Extension(c): Extension<Arc<Container>>) -> Json<Statistic> {
    let bots = c.repository.get_all_bots().unwrap();
    let mut hm = HashMap::new();

    for b in bots.into_iter() {
        hm.entry(b.name.clone())
          .or_insert(Vec::new())
          .push(b)
    }
    let mut bot_statistics = Vec::with_capacity(hm.len());
    for (key, value) in hm {
        let (win_days, lose_days, capital) = get_win_loss_capital(&value).await;

        bot_statistics.push(BotStatistic {
            bot_name: key,
            win_days,
            lose_days,
            capital,

            results: value,
        })
    }
    sort_bot_statistics(&mut bot_statistics);

    Json(Statistic {
        bot_statistics,
    })
}

pub async fn get_bot_statistics(Path(bot_name): Path<String>, Extension(c): Extension<Arc<Container>>) -> Json<Statistic> {
    let vec = c.repository.get_bot(bot_name).unwrap();
    let mut bot_statistics = Vec::with_capacity(vec.len());
    if vec.is_empty() {
        return Json(Statistic { bot_statistics });
    }

    let (win_days, lose_days, capital) = get_win_loss_capital(&vec).await;

    bot_statistics.push(BotStatistic {
        bot_name: vec[0].name.clone(),
        win_days,
        lose_days,
        capital,

        results: vec,
    });

    sort_bot_statistics(&mut bot_statistics);

    Json(Statistic {
        bot_statistics,
    })
}

pub async fn get_statistic_in_range(Path(bot_name): Path<String>, Query(range): Query<TimeRange>, Extension(c): Extension<Arc<Container>>) -> Json<Vec<Order>> {
    let start = tools::parse_time(&range.start_time);
    let end = tools::parse_time(&range.end_time);
    let vec = c.repository.get_orders_in_range(bot_name, start, end).unwrap();

    Json(vec)
}




async fn get_win_loss_capital(statistics: &Vec<StatisticResult>) -> (u16, u16, f64) {
    let mut win_days = 0;
    let mut lose_days = 0;
    let mut capital = 0.0;
    for res in statistics.iter() {
        if res.capital > 100.0 {
            win_days += 1;
        } else if res.capital < 100.0 {
            lose_days += 1;
        }

        capital += res.capital - 100.0;
    }
    (win_days, lose_days, capital)
}


#[derive(RustEmbed)]
#[folder = "ui/build"]
pub struct Assets;

impl Assets {
    pub fn get_file(path: &str) -> Option<(Vec<u8>, String)> {
        Assets::get(path).map(|file| {
            let mime = MimeGuess::from_path(path)
              .first_or_octet_stream()
              .as_ref()
              .to_owned();
            (file.data.into_owned(), mime)
        })
    }

    pub fn router() -> Router {
        let mut router = Router::new();

        for file_path in Assets::iter() {
            let path_string = file_path.as_ref().to_string();
            let route_path = format!("/{}", path_string);

            let path_for_closure = path_string.clone();

            let handler = get(move || {
                let path_for_closure = path_for_closure.clone();
                async move {
                    if let Some((content, mime)) = Assets::get_file(&path_for_closure) {
                        (StatusCode::OK, [("Content-Type", mime)], content)
                    } else {
                        (
                            StatusCode::NOT_FOUND,
                            [("Content-Type", "text/plain".to_string())],
                            Vec::new(),
                        )
                    }
                }
            });

            router = router.route(&route_path, handler.clone());

            // Handle /somePath/ route if the file is /somePath/index.html
            if path_string.ends_with("/index.html") {
                let prefix = path_string.strip_suffix("index.html").unwrap();
                let prefix_route = format!("/{}", prefix);
                router = router.route(&prefix_route, handler.clone());
            }
        }

        router
    }
}