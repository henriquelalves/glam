use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    /// Verbose (output subshell commands)
    #[clap(short, long, takes_value = false)]
    verbose: bool,
}

// TODO: COMMANDS BROKE
// TODO: ADD https://github.com/termapps/enquirer https://docs.rs/dialoguer/latest/dialoguer/
// TODO: ADD MULTIPLE TARGET / SOURCE FOLDERS PER REPOSITORY

#[derive(Subcommand)]
enum Commands {
    /// Initialize Godot project for GLAM
    Init {},

    /// Add new repository
    Add {
        /// Package project git
        git_repo: String,
    },

    /// Install all packages on .glam file
    Install {
        /// Don't copy to target folder
        #[clap(short, long, required = false, takes_value = false)]
        no_copy: bool,
    },

    /// Update a single repository. If no repository name is provided, update all repositories
    Update {
        /// Name of the package to update (default is all packages)
        package_name: String,
        /// Don't copy to target folder
        #[clap(short, long, required = false, takes_value = false)]
        no_copy: bool,
    },

    /// Apply changes to a repository
    Apply {
        /// Names of the package to apply changes to
        package_names: Vec<String>,
        /// Create new package from the specified addon folder (will create a git repo)
        #[clap(short, long, required = false, default_value = "")]
        create_from_addon: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {} => {
            let root = commands::search_project_root();
            commands::initialize_glam_files(&root);
            commands::initialize(&root);
        }

        Commands::Add {
            git_repo,
        } => {
            let root = commands::search_project_root();
            if commands::check_initialization(&root) {
                commands::add_repository(&root, git_repo, cli.verbose);
            }
        }

        Commands::Install { no_copy } => {
            let root = commands::search_project_root();
            if commands::check_initialization(&root) {
                commands::install_repositories(&root, cli.verbose, !*no_copy);
            }
        }

        Commands::Update {
            no_copy,
            package_name,
        } => {
            let root = commands::search_project_root();
            if commands::check_initialization(&root) {
                commands::update_repositories(&root, cli.verbose, !*no_copy);
            }
        }

        Commands::Apply {
            package_names,
            create_from_addon,
        } => {
            let root = commands::search_project_root();
            if commands::check_initialization(&root) {
                for package_name in package_names {
                    commands::apply_changes(&root, &package_name, &create_from_addon, cli.verbose);
                }
            }
        }
    }
}
