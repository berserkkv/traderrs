use env_logger::fmt::style::{Color, RgbColor};
use env_logger::Builder;
use std::env;
use std::io::Write;
use std::path::PathBuf;

pub fn init_logger() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let project_root = find_project_root(&current_dir).unwrap_or_else(|| current_dir.clone());

    let mut builder = Builder::new();

    let level = if cfg!(debug_assertions) {
        env::var("RUST_LOG").unwrap_or("info".to_string())
    } else {
        env::var("RUST_LOG").unwrap_or("off".to_string())
    };

    builder.parse_filters(&level);

    builder
        .format(move |buf, record| {
            let s: &str;
            match record.level() {
                log::Level::Error => s = "E",
                log::Level::Warn => s = "W",
                log::Level::Info => s = "I",
                log::Level::Debug => s = "D",
                log::Level::Trace => s = "T",
            }
            let level_style = buf.default_level_style(record.level());
            let file = record.file().unwrap_or("<unknown>");
            let line = record.line().unwrap_or(0);

            let mut abs_path = project_root.clone();
            abs_path.push(file);

            let file_url = format!("file://{}:{}", abs_path.display(), line);

            // OSC 8 hyperlink format: \x1b]8;;<link>\x1b\\<text>\x1b]8;;\x1b\\
            let link_start = format!("\x1b]8;;{}\x1b\\", file_url);
            let link_end = "\x1b]8;;\x1b\\";

            write!(buf, "{}[{}]", level_style, s.to_string()).expect("TODO: panic message");
            let level_style = level_style.fg_color(Some(Color::from(RgbColor {
                0: 210,
                1: 210,
                2: 210,
            })));
            write!(buf, " {}{} ", level_style, record.args()).expect("TODO: panic message");

            let level_style = level_style.fg_color(Some(Color::from(RgbColor {
                0: 45,
                1: 151,
                2: 227,
            })));
            write!(
                buf,
                "{}[{}{}:{}{}]",
                level_style, link_start, file, line, link_end
            )
            .expect("TODO: panic message");
            writeln!(buf)
        })
        .init();
}

fn find_project_root(start_dir: &PathBuf) -> Option<PathBuf> {
    let mut dir = start_dir.clone();
    loop {
        if dir.join("Cargo.toml").exists() {
            return Some(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    None
}
