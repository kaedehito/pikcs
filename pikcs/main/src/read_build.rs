use std::fs;

use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Build {
    pub build: Vec<String>,
    pub install: Vec<String>,
    pub remove: Vec<String>,
}

pub fn read_to_build() -> Build {
    let home = dirs::home_dir().unwrap();

    let mut path = format!("{}/.pikcs/build/.build", home.display());
    if cfg!(target_os = "windows") {
        path = format!("{}\\.pikcs\\build\\.build", home.display());
    }

    let build = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("{}: Failed to read build script", "Error".red().bold());
        eprintln!("{e}");
        std::process::exit(1);
    });

    let json: Build = serde_json::from_str(&build).unwrap_or_else(|e| {
        eprintln!("{}: Failed to parse json", "Error".red().bold());
        eprintln!("{e}");
        std::process::exit(1);
    });

    json
}
