use std::{env, sync::OnceLock};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub environment: String,
    pub otel_endpoint: String,
    pub service_name: String,
    pub service_version: String,
}

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

pub fn init_app_config() {
    if APP_CONFIG.get().is_some() {
        return;
    }

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    let environment = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    let otel_endpoint = env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .or_else(|_| env::var("OTEL_ENDPOINT"))
        .unwrap_or_else(|_| "http://localhost:4317".to_string());
    let service_name =
        env::var("SERVICE_NAME").unwrap_or_else(|_| "rust-otel-collector".to_string());
    let service_version = env::var("SERVICE_VERSION").unwrap_or_else(|_| "0.1.0".to_string());

    let app_config = AppConfig {
        port,
        environment,
        otel_endpoint,
        service_name,
        service_version,
    };

    let _ = APP_CONFIG.set(app_config);
}

pub fn get_config() -> &'static AppConfig {
    APP_CONFIG.get().unwrap()
}
