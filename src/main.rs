use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
struct Cli {
		#[clap(subcommand)]
		command: Commands,
		/// Verbose (output subshell commands)
		#[clap(short, long, takes_value = false)]
		verbose: bool
}

#[derive(Subcommand)]
enum Commands {
		/// Initialize Godot project for GLAM
		Init {
		},

		/// Install new GLAM package
		InstallPackage {
				/// Package project git
				git_repo: String,
				/// Commit to checkout (default is latest)
				#[clap(short, long, required = false, default_value = "")]
				commit: String,
				/// Don't copy to target folder
				#[clap(short, long, required = false, takes_value = false)]
				no_copy: bool,
		},

		/// Install all packages on .glam file
		Install {
				/// Don't copy to target folder
				#[clap(short, long, required = false, takes_value = false)]
				no_copy: bool,
		},

		/// Update a single GLAM package
		UpdatePackage {
				/// Name of the package to update (default is all packages)
				package_name: String,
				/// Don't copy to target folder
				#[clap(short, long, required = false, takes_value = false)]
				no_copy: bool,
		},

		/// Update all GLAM packages
		Update {
				/// Don't copy to target folder
				#[clap(short, long, required = false, takes_value = false)]
				no_copy: bool,
		},

		/// Remove a GLAM package
		RemovePackage {
				/// Name of the package to remove
				package_name: String,
		},

		/// Apply changes to a package
		Apply {
				/// Names of the package to apply changes to
				package_names: Vec<String>,
				/// Create new package from the specified addon folder (will create a git repo)
				#[clap(short, long, required = false, default_value = "")]
				create_from_addon: String,
		}
}

fn main() {
		let cli = Cli::parse();

		match &cli.command {
				Commands::Init {} => {
						let root = commands::search_project_root();
						commands::initialize_glam_files(&root);
						commands::initialize(&root);
				},

				Commands::InstallPackage {git_repo, commit, no_copy} => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						commands::install_package(&root, git_repo, commit, !*no_copy, cli.verbose);
				},

				Commands::Install { no_copy } => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						commands::install_all_packages(&root, cli.verbose, !*no_copy);
				},

				Commands::UpdatePackage { package_name, no_copy } => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						commands::update_package(&root, &package_name, cli.verbose, !*no_copy);
				},

				Commands::Update { no_copy } => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						commands::update_all_packages(&root, cli.verbose, !*no_copy);
				},

				Commands::RemovePackage {package_name} => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						commands::remove_package(&root, &package_name, cli.verbose);
				},

				Commands::Apply {package_names, create_from_addon} => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						for package_name in package_names {
								commands::apply_changes(&root, &package_name, &create_from_addon, cli.verbose);
						}

				},
		}
}
