use std::{fs, io, time::Duration};

use crate::read_package_list::read_to_package;
use colored::*;
use git2::{FetchOptions, Progress, RemoteCallbacks};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

pub fn update(user: Option<String>) {
    #[allow(unused)]
    let mut username = String::new();

    // ここなんかもっとよくできそう
    // MARK
    if user.is_none() {
        username = read_to_package().user;
    } else {
        username = user.unwrap();
    }

    let url = format!("https://github.com/{}/packagelist", username);

    #[allow(unused)]
    let mut into = String::new();

    let s = dirs::home_dir().unwrap();

    if cfg!(target_os = "windows") {
        into = format!("{}\\.pikcs\\packagelist", &s.clone().display());
    } else {
        into = format!("{}/.pikcs/packagelist", &s.clone().display());
    }

    let mut temp = into.clone();

    fs::remove_dir_all(temp).unwrap_or_else(|e| {
        if e.kind() != io::ErrorKind::NotFound {
            eprintln!("{}: Failed to remove package list", "Error".red().bold());
            eprintln!("{e}");
            std::process::exit(1);
        }
    });

    let prg = ProgressBar::new_spinner();

    prg.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.white} {msg}")
            .unwrap(),
    );

    prg.enable_steady_tick(Duration::from_millis(100));
    prg.set_message(format!(
        "Clone package list: https://github.com/{}/packagelist",
        username
    ));

    let mut callbacks = RemoteCallbacks::new();

    callbacks.transfer_progress(|progress: Progress| {
        let received = progress.received_objects();
        let total = progress.total_objects();

        if total > 0 {
            let percentage = (received * 100) / total;
            prg.set_message(format!(
                "Cloning... {}/{} objects ({:>3}%) | {:.2} MB",
                received,
                total,
                percentage,
                progress.received_bytes() as f64 / 1024.0 / 1024.0
            ));
        }
        true
    });

    let mut fetch_options = FetchOptions::new();

    fetch_options.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    builder.clone(&url, Path::new(&into)).unwrap_or_else(|e| {
        eprintln!("{}: Failed to clone repository", "Error".red().bold());
        eprintln!("{}", e);
        std::process::exit(1);
    });

    prg.set_message("Clone completed");

    if cfg!(target_os = "windows") {
        temp = format!("{}\\.pikcs\\packagelist\\.git", &s.clone().display());
    } else {
        temp = format!("{}/.pikcs/packagelist/.git", &s.clone().display());
    }

    #[allow(unused)]
    let mut readme = String::new();

    if cfg!(target_os = "windows") {
        readme = format!("{}\\.pikcs\\packagelist\\README.md", &s.clone().display());
    } else {
        readme = format!("{}/.pikcs/packagelist/README.md", &s.clone().display());
    }

    fs::remove_dir_all(temp).unwrap_or_else(|e| {
        eprintln!("{}: Failed to remove .git directory", "Error".red().bold());
        eprintln!("{e}");
        std::process::exit(1);
    });

    fs::remove_file(readme).unwrap_or_else(|e| {
        if e.kind() == io::ErrorKind::NotFound {
        } else {
            eprintln!("{}: Failed to remove README.md", "Error".red().bold());
            eprintln!("{e}");
            std::process::exit(1);
        }
    });

    prg.finish_with_message("Update completed");
}
