#[macro_use]
mod debug;
mod consts;
mod git;
mod install;
mod log;
mod path;
mod read_build;
mod read_package_list;
mod remove;
mod search;
mod setup;
mod update;
mod upgrade;
mod version;
use clap::{Parser, Subcommand};
//use debug as gdb;
//use user;

#[derive(Parser)]
#[command(name = "pikcs")]
#[command(author = "ogasawara futo")]
#[command(version = "1.0")]
#[command(about = "package manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/*
#[derive(Subcommand)]
enum User {
    /// Chenge the packagelist of github user
    Chenge {
        #[arg(required = true)]
        user: String,
    },

    /// Add the github user repositorys
    Add {
        #[arg(required = true)]
        user: String,
    },

    /// Remove the github user repositorys
    Remove {
        #[arg(required = true)]
        user: String,
    },
}
*/

#[derive(Subcommand)]
enum Commands {
    /// Search for a package
    Search {
        /// Package name to search
        #[arg(required = true)]
        package: String,
    },
    /// Update package list
    Update {
        /// Specific package to update
        #[arg(required = false)]
        user: Option<String>,
    },
    /// Install a package
    Install {
        /// Package name to install
        #[arg(required = true)]
        package: String,
    },
    /// Remove a package
    Remove {
        /// Package name to remove
        #[arg(required = true)]
        package: String,
    },
    /*
    /// Upgrade packages
    Upgrade {
        /// upgrade the package
        #[arg(required = false)]
        package: Option<String>,
    },
    
    /// Options of github users
    User {
        #[command(subcommand)]
        user: User,
    },
    */
}

fn main() {
    setup::setup();

    let cli = Cli::parse();

    match cli.command {
        Commands::Search { package } => {
            search::search_package(package);
        }
        Commands::Update { user } => {
            update::update(user);
        }
        Commands::Install { package } => {
            install::install(&package);
        }
        #[allow(unused)]
        Commands::Remove { package } => {
            let user = read_package_list::read_to_package();
            remove::remove(&package, &user.user);
        }
        /*
        Commands::Upgrade { package } => {
            upgrade::upgrade(package);
        }
        #[allow(unused)]
        Commands::User { user } => match user {
            User::Add { user } => {
                user::add::add(user);
            }
            User::Chenge { user } => {
                todo!("User.Chenge is not implmented!");
            }
            User::Remove { user } => {
                todo!("User.Remove is not implmented!");
            }
        },
        */
    }
}
