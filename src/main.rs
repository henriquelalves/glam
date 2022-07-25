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
				#[clap(short, long, required=false)]
				commit: String,
		},

		/// Install packages on .glam file
		Install {
				/// Verbose (output subshell commands)
				#[clap(short, long, takes_value = false)]
				verbose: bool
		},

		UpdatePackage {
				/// Name of the package to update (default is all packages)
				#[clap(short, long)]
				package_name: String,
		},

		/// Update all GLAM packages
		Update {
		},

		/// Remove a GLAM package
		RemovePackage {
				/// Name of the package to remove
				package_name: String,
		},

		/// Apply changes to a package
		Apply {
				/// Name of the package to apply changes to
				package_name: String,
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

				Commands::InstallPackage {git_repo, commit} => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						commands::install_package(&root, git_repo, commit, cli.verbose);
				},

				Commands::Install { verbose } => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						commands::install(&root, *verbose);
				},

				Commands::UpdatePackage { package_name } => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						commands::update_package(&root, &package_name, cli.verbose);
				},

				Commands::Update {} => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						commands::update_all_packages(&root, cli.verbose);
				},

				Commands::RemovePackage {package_name} => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						commands::remove_package(&root, &package_name, cli.verbose);
				},

				Commands::Apply {package_name} => {
						let root = commands::search_project_root();
						commands::check_ignores(&root);
						commands::initialize_glam_files(&root);
						commands::apply_changes(&root, &package_name, cli.verbose);
				},
		}
}
