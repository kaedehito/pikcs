#[allow(unused)]
pub fn build_buildscript_path() -> String {
    let home = dirs::home_dir().unwrap();

    let mut path = format!("{}/.pikcs/build/.build", home.display());
    if cfg!(target_os = "windows") {
        path = format!("{}\\.pikcs\\build\\.build", home.display());
    }
    path
}

pub fn bin() -> String {
    let home = dirs::home_dir().unwrap();

    let path = format!("{}/.pikcs/bin", home.display());
    path
}

pub fn log() -> String {
    let home = dirs::home_dir().unwrap();
    let path = format!("{}/.pikcs/log", home.display());
    path
}

pub fn build_path() -> String {
    let home = dirs::home_dir().unwrap();

    let path = format!("{}/.pikcs/build", home.display());
    path
}
