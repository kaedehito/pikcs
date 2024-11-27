use crate::path;
use colored::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::Write;

#[allow(unused)]
#[derive(Deserialize, Serialize)]
pub struct LogEntry {
    pub log: Log,
    pub remove: Vec<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
pub struct Log {
    pub package: String,
    pub version: String,
}

#[allow(dead_code)]
impl LogEntry {
    pub fn new(package: String) -> Self {
        let log_path = path::log();
        let log_path = format!("{}/{}", log_path, package);
        let log_path: &std::path::Path = log_path.as_ref();

        let content = fs::read_to_string(log_path).unwrap_or_else(|e| {
            eprintln!("{}: Failed to read log file", "Error".red().bold());
            eprintln!("{e}");
            std::process::exit(1);
        });

        let json: LogEntry = serde_json::from_str(&content).unwrap_or_else(|e| {
            eprintln!("{}: Failed to parse log file", "Error".red().bold());
            eprintln!("{e}");
            std::process::exit(1);
        });

        json
    }

    pub fn write(
        package: String,
        version: String,
        remove: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let log_file = path::log();
        let log_file = format!("{}/{}", log_file, package);
        let log_file: &std::path::Path = log_file.as_ref();
        let mut file = fs::File::create(log_file)?;

        writeln!(
            file,
            r#"{{
    "log": {{
        "package": "{}",
        "version": "{}"
    }},
    "remove": {:?}
}}
            "#,
            package, version, remove
        )?;

        Ok(())
    }
    pub fn get_remove(&self) -> Vec<String> {
        self.remove.clone()
    }
}

pub fn search_log_file(package: &str) {
    let log_path = path::log();
    let log_path = format!("{}/{}", log_path, package);

    let log_path: &std::path::Path = log_path.as_ref();

    if !log_path.exists() {
        eprintln!("package not found: {}", package);
        std::process::exit(1);
    }
}
