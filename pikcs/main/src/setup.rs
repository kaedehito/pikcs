use std::fs;
use std::path::Path;

use dirs::home_dir;

pub fn setup() {
    #[allow(unused_assignments)]
    let mut path = String::new();

    let temp = &[".pikcs/", ".pikcs/bin", ".pikcs/log"];

    for t in temp {
        path = format!("{}/{t}", home_dir().unwrap().display());
        let path = Path::new(&path);
        if !path.exists() {
            fs::create_dir(path).unwrap_or_else(|e| {
                eprintln!("{}", e);
            });
        }
    }
}
