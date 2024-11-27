use std::time::Duration;
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};
use git2::{RemoteCallbacks, FetchOptions};
use colored::Colorize;

pub fn clone(url: &str, into: &str) -> Result<(), Box<dyn std::error::Error>> {
    let prg = spinner();

    prg.set_message(format!("Cloning {}...", url));

    let mut callbacks = RemoteCallbacks::new();

    callbacks.transfer_progress(|progress| {
        let received = progress.received_objects();
        let total = progress.total_objects();

        if total > 0 {
            let percentage = (received * 100) / total;
            let received_mb = progress.received_bytes() as f64 / (1024.0 * 1024.0);
            prg.set_message(format!(
                "Cloning... {}/{} objects ({:>3}%) | {:.2} MB",
                received, total, percentage, received_mb
            ));
        }
        true
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    let result = builder.clone(url, Path::new(into));

    result.map_err(|e| {
        prg.finish();
        eprintln!("{}: Failed to clone repository", "Error".red().bold());
        eprintln!("{}", e);
        e
    })?;
    Ok(())
}


pub fn spinner() -> ProgressBar{
    let prg = ProgressBar::new_spinner();

    prg.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"
            ])
            .template("{spinner:.white} {msg}")
            .expect("Failed to set progress bar style")
    );

    prg.enable_steady_tick(Duration::from_millis(100));
    prg
}