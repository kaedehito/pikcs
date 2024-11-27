use crate::read_package_list::read_to_package;

pub fn search_package(package: String) {
    let pkg = read_to_package();

    let depend = pkg.search_package(package.clone()).unwrap_or_else(|| {
        eprintln!("repository not found: {}", package);
        std::process::exit(1);
    });

    println!("{}/{} {}",pkg.user ,depend.name, depend.version);
    println!("{}", depend.desc);
}
