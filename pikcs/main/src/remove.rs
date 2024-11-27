use crate::git;
use crate::log;
use crate::path;
use colored::*;
use std::env;
use std::fs;
use std::process::{Command, Stdio};

pub fn remove(package: &str, user: &str) {
    let spinner = git::spinner();
    spinner.set_message("Reading log file...");

    log::search_log_file(package);

    let entry = log::LogEntry::new(package.to_string());
    let remove = entry.get_remove();

    for script in remove {
        let sc: Vec<&str> = script.split_whitespace().collect();
        spinner.set_message(format!("removing {}/{}... {}", user, package, script));

        if sc[0] == "cd" {
            env::set_current_dir(sc[1]).unwrap();
            continue;
        }

        let _status = Command::new(sc.first().unwrap())
            .args(&sc[1..])
            .stdout(Stdio::null())
            .status()
            .unwrap_or_else(|e| {
                eprintln!("{}: {script}:", "Error".red().bold());
                eprintln!("{e}");
                std::process::exit(1);
            });
    }

    let path = path::log();
    let path = format!("{}/{}", path, package);
    let path: &std::path::Path = path.as_ref();

    spinner.set_message("Deleting log file...");
    fs::remove_file(path).unwrap_or_else(|e| {
        eprintln!("{}: Failed to remove log file", "Error".red().bold());
        eprintln!("{e}");
        std::process::exit(1);
    });

    let path = path::bin();
    let path = format!("{}/{}", path, package);
    let path: &std::path::Path = path.as_ref();

    spinner.set_message("Deleting executable file...");
    fs::remove_file(path).unwrap_or_else(|e| {
        eprintln!("{}: Failed to remove executable file", "Error".red().bold());
        eprintln!("{e}");
        std::process::exit(1);
    });
    spinner.finish();
}
