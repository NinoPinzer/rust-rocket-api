use rocket::fairing::AdHoc;
use tracing::subscriber::set_global_default;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("Logger", |rocket| async {
        let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        let subscriber = fmt()
            .with_env_filter(EnvFilter::new(log_level))
            .with_target(false)
            .finish();

        if set_global_default(subscriber).is_err() {
            eprintln!("Global logger already set, skipping initialization.");
        }

        rocket
    })
}