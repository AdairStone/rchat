
use serde::Deserialize;
use config::{Config, ConfigError, File};
use std::env;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("Failed to load settings");
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub redis_url: String,
    pub oss_access_key_id: String,
    pub oss_access_key_secret: String,
    pub oss_endpoint: String,
    pub oss_bucket_name: String,
    pub time_zone: i32,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("env").unwrap_or_else(|_| "dev".into());
        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/app.default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("config/app.{}", run_mode))
                    .required(false),
            )
            // .add_source(File::with_name("config/local").required(false))
            // .add_source(Environment::with_prefix("app"))
            .build()?;
        // Now that we're done, let's access our configuration
        tracing::info!("redis_url: {:?}", s.get::<String>("oss_access_key_id"));
        tracing::info!("oss_bucket_name: {:?}", s.get::<String>("oss_bucket_name"));
        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
