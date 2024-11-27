use std::{env, fs};

use colored::Colorize;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Package {
    pub version: u32,
    pub user: String,
    pub list: Vec<Dependency>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub url: String,
    pub license: String,
    pub language: String,
    pub desc: String,
    pub dependencies: Vec<String>,
    pub version: String,
}

#[allow(warnings)]
impl Package {
    pub fn search_package(&self, package: String) -> Option<&Dependency> {
        self.list.iter().find(|s| s.name == package)
    }

    pub fn get_desc(&self, get_package: String) -> Option<&String> {
        for s in &self.list {
            if s.name == get_package {
                return Some(&s.desc);
            }
        }
        None
    }
}

pub fn read_to_package() -> Package {
    let home = dirs::home_dir().unwrap();

    let path = home.to_str().unwrap_or_else(|| {
        eprintln!("{}: Failed to get home directory", "Error".red().bold());
        std::process::exit(1);
    });

    #[allow(unused_assignments)]
    let mut listpath = "Error".to_string();
    if env::consts::OS.to_lowercase() == "windows" {
        listpath = format!("{path}\\.pikcs\\packagelist\\packagelist.json")
    } else {
        listpath = format!("{path}/.pikcs/packagelist/packagelist.json");
    }

    if listpath == "Error" {
        eprintln!("{}", listpath);
    }

    let pkg = fs::read_to_string(listpath).unwrap_or_else(|err| {
        eprintln!("{}: Failed to read package list.", "Error".bold().red());
        eprintln!("{err}");
        std::process::exit(1);
    });

    let package: Package = serde_json::from_str(&pkg).unwrap_or_else(|e| {
        eprintln!(
            "{}: Failed to parse package list json",
            "Error".red().bold()
        );
        eprintln!("{}", e);
        std::process::exit(1);
    });

    if package.version > crate::consts::PIKCS_VERSION {
        eprintln!("{}: pikcs version is old", "Error".red().bold());
        eprintln!("Please upgrade the pikcs");
        std::process::exit(1);
    }

    package
}
