use crate::log;
use crate::path::build_path;
use crate::{git, path};
use crate::{read_build::read_to_build, read_package_list::read_to_package};
use colored::*;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, fs};

pub fn install(package: &str) {
    let build_path = build_path();
    let build_path = Path::new(&build_path).to_str().unwrap();
    let s: &Path = build_path.as_ref();

    // remove old build directory
    if s.exists() {
        fs::remove_dir_all(s).unwrap_or_else(|e| {
            eprintln!("{}: Failed to remove build directory", "Error".red().bold());
            eprintln!("{e}");
            std::process::exit(1);
        });
    }

    let data = read_to_package();

    // search the package
    let pkg = data.search_package(package.to_string()).unwrap_or_else(|| {
        eprintln!("Package not found: {}", package);
        std::process::exit(1);
    });

    //  search dependencies
    for depen in pkg.dependencies.clone() {
        if cfg!(target_os = "windows") {
            let exit = Command::new("where")
                .arg(depen)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .unwrap_or_else(|e| {
                    eprintln!("{}: {e}", "Error".red().bold());
                    std::process::exit(1);
                });
            if !exit.success() {
                eprintln!(
                    "{}: This repository requires: {:?}",
                    "Error".red().bold(),
                    pkg.dependencies
                );
                eprintln!("Please install these and try again");
                std::process::exit(1);
            }
        } else {
            Command::new("which")
                .arg(depen)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .unwrap_or_else(|e| {
                    eprintln!("{}: {e}", "Error".red().bold());
                    std::process::exit(1);
                });
        }
    }

    // clone repository
    git::clone(&pkg.url, build_path).unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });

    // generate spinner
    let spin = git::spinner();

    spin.set_message(format!("building {}...", pkg.name));

    let scripts = read_to_build();

    //  for build, set current dir
    env::set_current_dir(build_path).unwrap();

    // run the build script
    for script in scripts.build {
        let sc: Vec<&str> = script.split_whitespace().collect();
        spin.set_message(format!("building {}... {}", pkg.name, script));

        if sc[0] == "cd" {
            env::set_current_dir(sc[1]).unwrap();
            continue;
        }

        let _status = Command::new(sc.first().unwrap())
            .args(&sc[1..])
            .stdout(Stdio::null())
            .status()
            .unwrap_or_else(|e| {
                eprintln!("{script}:");
                eprintln!("{}: {e}", "Error".red().bold());
                std::process::exit(1);
            });
    }

    // msg: installing {}...
    spin.set_message(format!("installing {}...", pkg.name));

    // set a current dir
    env::set_current_dir(build_path).unwrap();

    // run the install script
    for script in scripts.install {
        let sc: Vec<&str> = script.split_whitespace().collect();
        spin.set_message(format!(
            "installing {}... {}",
            pkg.name,
            sc.first().unwrap()
        ));

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

    // set a installing message
    spin.set_message("installing...");

    // vars
    let exec = format!("{}/{}", build_path, pkg.name);
    let bin = path::bin();
    let bin = format!("{bin}/{}", pkg.name);
    let exec: &std::path::Path = exec.as_ref();
    let bin: &std::path::Path = bin.as_ref();

    // move to bin directory
    fs::rename(exec, bin).unwrap_or_else(|e| {
        spin.finish();
        eprintln!("{}: Failed to move executable file", "Error".red().bold());
        eprintln!("{e}");
        std::process::exit(1);
    });

    log::LogEntry::write(pkg.name.clone(), pkg.version.clone(), scripts.remove).unwrap_or_else(
        |e| {
            eprintln!("src/install.rs 154:9 {}: {}", "Error".red().bold(), e);
            std::process::exit(1);
        },
    );

    spin.set_message("Cleaning...");

    // remove build directory
    if s.exists() {
        fs::remove_dir_all(s).unwrap_or_else(|e| {
            eprintln!("{}: Failed to remove build directory", "Error".red().bold());
            eprintln!("{e}");
            std::process::exit(1);
        });
    }
    // finish
    spin.finish();
}
