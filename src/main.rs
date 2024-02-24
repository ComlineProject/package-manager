// Relative Modules
pub mod client;
pub mod manager;

#[cfg(test)]
pub mod tests;


// Standard Uses
use std::{path::Path, process::ExitCode};

// Crate Uses

// External Uses
use comline_core::package;
use comline_core::package::config::ir::frozen as package_frozen;

use clap::{Parser, Subcommand};
use colored::Colorize;



#[derive(Parser)]
#[command(name = "Comline Package Manager")]
#[command(bin_name = "comlinepm")]
#[command(author = "Comline")]
#[command(version = "0.1.1")]
#[command(
    about = "Package Manager and Publisher tool for Comline (also compiles things)",
    long_about = None)
]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
#[derive(Debug)]
enum Commands {
    /// Create new package
    New {
        name: String,

        #[arg(short, long, default_value="false")]
        force: bool
    },

    /// Add dependency to package
    Add {
        dependency: String,

        #[arg(short, long, default_value="false")]
        local: bool
    },

    /// Build package
    Build,

    /// Registry related operations like authentication, publishing
    #[command(subcommand)]
    Registry(RegistryCommands)
}

#[derive(Subcommand)]
#[derive(Debug)]
enum RegistryCommands {
    /// Login to a package registry
    Login {
        name: String,

        #[arg(short, long)]
        password: Option<String>,

        #[arg(short, long, default_value="ssh")]
        method: String
    },

    /// Logout of a package registry
    Logout {
        name: String
    },

    /// Publish package
    Publish {
        /// The registries on where to publish, divided by space
        #[arg(long)]
        registries: String,
    }
}


pub fn main() -> ExitCode {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name, force } => match_new(name, *force),
        Commands::Add { dependency, local } => match_add(dependency, *local),
        Commands::Build => match_build(),
        Commands::Registry(commands) => match_registry(&commands),
    }
}

fn match_new(name: &String, force: bool) -> ExitCode {
    println!(
        "{} new package named {}",
        "Creating".green(), name.yellow().underline()
    );
    let cur_path = std::env::current_dir().unwrap();

    let new_package_path = cur_path.join(name.to_owned() + "/");
    if let Err(e) = client::create_empty_package_at(&new_package_path, name, force) {
        eprintln!("{}: {}", "Could not create package".red().underline(), e);
        return ExitCode::FAILURE
    };

    let relative_path = new_package_path.strip_prefix(cur_path).unwrap();
    println!("Package created at {}", relative_path.display());

    ExitCode::SUCCESS
}

fn match_add(dependency: &String, local: bool) -> ExitCode {
    println!("{} for dependency...", "Searching".green());
    let package_path = std::env::current_dir().unwrap();

    if local {
        println!("Searching for local package at \"{}\"", dependency);

        let dependency_path = Path::new(dependency);
        if dependency_path.exists() {
            eprintln!("Given package path does not exist")
        }

        if !package::config::is_package_path(dependency_path) {
            eprintln!("Given package path does not seem to be a valid package")
        }

        return ExitCode::FAILURE
    }

    if let Err(e) = client::dependency::add_remote_dependency(
        dependency.clone(), &package_path
    ) {
        eprintln!("Cannot add dependency: \n - {}", e)
    }

    ExitCode::SUCCESS
}

fn match_build() -> ExitCode {
    println!("{} build process...", "Starting".green());
    let package_path = std::env::current_dir().unwrap();

    match package::build::build(&package_path) {
        Ok(ctx) => {
            println!(
                "{} package to latest version ({}) 🐸",
                "Built".green(),
                package::config::ir::frozen::version(&ctx.config_frozen.unwrap()).unwrap()
                    .green().underline()
            )
        }
        Err(e) => {
            eprintln!("Couldn't build package: \n - {}", e);
            return ExitCode::FAILURE
        }
    }

    ExitCode::SUCCESS
}

fn match_registry(commands: &RegistryCommands) -> ExitCode {
    match commands {
        RegistryCommands::Login { name, password, method } => {
            println!(
                "{} at package registry with name '{}' with method '{}'",
                "Logging-in".green(),
                name.yellow().underline(),
                method.yellow().underline(),
            );

            if let Err(e) = client::registry::login(method, name, password.clone()) {
                eprintln!("{}: {}", "Could not login in regisry:".red().underline(), e);
                return ExitCode::FAILURE
            };
        },
        RegistryCommands::Logout { name } => {
            todo!()
        },
        RegistryCommands::Publish { registries } => {
            let parts: Vec<String> = registries.split_whitespace()
                .map(|p| p.to_owned()).collect();
            println!(
                "Starting process to publish into '{}' registries: {}",
                parts.len(), registries
            );
    
            println!(
                "{} package before trying to publish", "Building".green()
            );
            let package_path = std::env::current_dir().unwrap();
            let package_ctx = package::build::build(&package_path)
                .unwrap();
    
            let package_config = package_ctx.config_frozen.as_ref().unwrap();
            let package_name = package_frozen::namespace(package_config).unwrap();
            let package_version = package_frozen::version(package_config).unwrap();
            println!(
                "Package '{}'#'{}' is okay, starting publish in registries",
                package_name, package_version
            );
            if let Err(error) = client::publish::publish_to_registries(
                &package_ctx, parts
            ) {
                eprintln!("{}", error);
                return ExitCode::FAILURE
            }
        }
    }

    ExitCode::SUCCESS
}

