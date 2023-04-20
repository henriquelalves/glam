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

#[derive(Subcommand)]
enum Commands {
    /// Initialize Godot project for GLAM
    Init {},

    /// Add new repository
    Add {
        /// Package project git
        git_repo: String,
    },

    /// Update a repository
    Update {},

    /// Install all addons on glam file
    Install {},
    // Apply changes to a repository
    //Apply {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {} => {
            let root = commands::search_project_root();
            commands::initialize_glam_files(&root);
            commands::initialize(&root);
        }

        Commands::Add { git_repo } => {
            let root = commands::search_project_root();
            if commands::check_initialization(&root) {
                commands::add_repository(&root, git_repo, cli.verbose);
            }
        }

        Commands::Update {} => {
            let root = commands::search_project_root();
            if commands::check_initialization(&root) {
                commands::update_repository(&root, cli.verbose);
            }
        } 
        
        Commands::Install {} => {
            let root = commands::search_project_root();
            if commands::check_initialization(&root) {
                commands::install_repositories(&root, cli.verbose);
            }
        } 
        
        /*Commands::Apply {} => {
              let root = commands::search_project_root();
              if commands::check_initialization(&root) {
                  commands::apply_changes(&root, cli.verbose);
              }
          }*/
    }
}
