use std::{env, sync::OnceLock};
use config::{Config, File};
use log::error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub cmd: String,
    pub query: String,
    pub raft: String
}

impl Settings {
    pub fn new() -> Self {
        let run_mode = env::var("RUN_MODE").unwrap_or_default();
        
        let config = Config::builder()
            .add_source(File::with_name("conf/default"))
            .add_source(File::with_name(&format!("conf/{run_mode}")).required(false))
            .build();

        let config = match config {
            Ok(c) => c,
            Err(e) => {
                error!("[BUG] Failed to build configuration file: {}", e);
                std::process::exit(-1);
            }
        };

        match config.try_deserialize() {
            Ok(s) => s,
            Err(e) => {
                error!("[BUG] Failed to create Settings from config: {}", e);
                std::process::exit(-1);
            }
        }
    }
}

pub fn settings() -> &'static Settings {
    static SETTINGS: OnceLock<Settings> = OnceLock::new();
    SETTINGS.get_or_init(|| {
        Settings::new()
    })
}