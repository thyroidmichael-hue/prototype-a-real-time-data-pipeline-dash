// Configuration file for Real-Time Data Pipeline Dashboard Prototype

// Import required crates
extern crate tokio;
extern crate TokioPostgres;
extern crate serde_json;
extern crate actix_web;

// Database connection settings
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "password".to_string(),
            database: "realtime_dashboard".to_string(),
        }
    }
}

// Data pipeline settings
pub struct PipelineConfig {
    pub input_topic: String,
    pub output_topic: String,
    pub kafka_broker: String,
}

impl PipelineConfig {
    pub fn new() -> Self {
        PipelineConfig {
            input_topic: "input_data".to_string(),
            output_topic: "processed_data".to_string(),
            kafka_broker: "localhost:9092".to_string(),
        }
    }
}

// Dashboard settings
pub struct DashboardConfig {
    pub port: u16,
    pub host: String,
}

impl DashboardConfig {
    pub fn new() -> Self {
        DashboardConfig {
            port: 8080,
            host: "localhost".to_string(),
        }
    }
}

// Main configuration
pub struct Config {
    pub database: DatabaseConfig,
    pub pipeline: PipelineConfig,
    pub dashboard: DashboardConfig,
}

impl Config {
    pub fn new() -> Self {
        Config {
            database: DatabaseConfig::new(),
            pipeline: PipelineConfig::new(),
            dashboard: DashboardConfig::new(),
        }
    }
}