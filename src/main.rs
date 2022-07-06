use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::fs::write;
use std::process::exit;
use std::fs;
use json;

mod content;
mod utils;

#[derive(Parser)]
struct Cli {
		#[clap(subcommand)]
		command: Commands,
}

#[derive(Subcommand)]
enum Commands {
		Init {
		},

		/// Install new addon
		Install {
				/// Addon project git
				git_repo: String
		},

		Update {
		},

		Remove {
		},
}

#[derive(Clone)]
#[derive(Debug)]
struct GlamObject {
		name : String,
		git_repo : String
}

fn main() {
		let cli = Cli::parse();

		match &cli.command {
				Commands::Init {} => {
						initialize();
				},

				Commands::Install {git_repo: git_address} => {
						check_initialization();
						install_addon(git_address);
				},

				Commands::Update {} => {
						check_initialization();
						update_addon();
				},

				Commands::Remove {} => {
						check_initialization();
						remove_addon();
				},
		}
}

fn initialize() {
		if !Path::new("./.gitignore").exists() {
				match write("./.gitignore", content::create_gitignore_file()) {
						Ok(_v) => (),
						Err(_e) => {
								println!("There was a problem creating the .gitignore file!");
								exit(1);
						}
				}
		}

		if !Path::new("./.gdignore").exists() {
				match write("./.gdignore", content::create_gdignore_file()) {
						Ok(_v) => (),
						Err(_e) => {
								println!("There was a problem creating the .gdignore file!");
								exit(1);
						}
				}
		}
}

fn check_initialization() {
		if !Path::new("./.gitignore").exists() {
				println!(".gitignore file does not exist!");
		}
		if !Path::new("./.gdignore").exists() {
				println!(".gdignore file does not exist!");
		}
}

fn install_addon(git_repo : &str) {

		// Search for project root folder
		let root = search_project_root();
		println!("Found root project in: {}", root);

		// Create glam.d/ folder if it doesn't exist
		if !Path::new(&format!("{}/.glam.d/", root)).exists() {
				utils::run_shell_command(
						&format!("mkdir -p {}/.glam.d/", root),
						None
				);
		}

		// Create .glam file if it doesn't exist
		if !Path::new(&format!("{}/.glam", root)).exists() {
				fs::write(&format!("{}/.glam",root),
									content::create_glam_file())
						.expect("Couldn't create .glam file!");
		}

		// Find glam object or create one with default configuration
		let mut glam_objects = read_glam_file();
		let mut glam_obj : Option<GlamObject> = None;
		let name = utils::get_repo_name(git_repo);

		for obj in glam_objects.iter() {
				if obj.name == name {
						glam_obj = Some(obj.clone());
				}
		}

		match glam_obj {
				None => {
						let obj = GlamObject{
								git_repo : git_repo.to_string(),
								name : name.to_string()
						};

						glam_obj = Some(obj.clone());
						glam_objects.push(obj);
				}

				_ => {}
		}

		let target_obj = glam_obj.unwrap();

		// If glam addon folder doesn't exist, clone project
		if !Path::new(&format!("{}/.glam.d/{}", root, target_obj.name)).exists() {
				utils::run_shell_command(
						&format!("cd {}/.glam.d/ && git clone {} {}", root, target_obj.git_repo, target_obj.name),
						None
				);
				println!("Created addon folder on .glam.d!");
		} else {
				println!("Not Created!");
		}

		// If project addon folder doesn't exist, create it
		utils::run_shell_command(
				&format!("mkdir -p {}/addons/{}", root, name),
				None
		);

		// TODO: use source_folder to copy files from (default: /addons/)
		// TODO: use target_folder to copy files to (defautl: (root)/)
		// Copy addon repository content to target folder
		utils::run_shell_command(
				&format!("cp -f -r {}/.glam.d/{}/addons/* -t {}/addons/",root, name, root),
				None
		);

		// Write .glam file
		write_glam_file(&glam_objects);
}

// TODO: Use root folder
fn read_glam_file() -> Vec<GlamObject> {
		if !Path::new("./.glam").exists() {
				fs::write("./.glam", content::create_glam_file()).expect("Couldn't create .glam file!");
		}

		let glam_content = fs::read_to_string("./.glam").expect("Couldn't read .glam file!");
		let glam_obj = json::parse(&glam_content).expect("Couldn't parse .glam file!");

		let packages = &glam_obj["packages"];

		let mut glam_objects = Vec::new();

		println!("{}", packages);
		for i in packages.entries() {
				println!("{}", i.1);
				glam_objects.push(
						GlamObject{
								name : i.1["name"].to_string(),
								git_repo : i.1["git_repo"].to_string()
						}
				);
		}

		return glam_objects;
}

// TODO: Use root folder
fn write_glam_file(glam_objects : &Vec<GlamObject>) {
		let mut data = json::object!{};
		data["packages"] = json::object!{};

		for glam_obj in glam_objects {
				println!("{:?}", glam_obj);
				data["packages"][&glam_obj.name] = json::object!{};
				data["packages"][&glam_obj.name]["name"] = glam_obj.clone().name.into();
				data["packages"][&glam_obj.name]["git_repo"] = glam_obj.clone().git_repo.into();
		}

		let json_string = json::stringify(data);

		println!("{}", json_string);
		fs::write("./.glam", json_string).expect("Couldn't create .glam file!");
}

fn update_addon() {
}

fn remove_addon() {
}

fn search_project_root() -> String{
		let path = PathBuf::from("./");
		let mut dir = path.canonicalize().unwrap();

		loop {
				let dir_path = dir.to_str().unwrap();
				let proj_path = format!("{}/project.godot", dir_path);
				println!("{}", proj_path);
				let godot_project = Path::new(&proj_path);

				if godot_project.exists() {
						break;
				}
				else {
						let parent = dir.parent();
						if parent.is_none() {
								panic!("Godot project not found!");
						}
						dir = dir.parent().unwrap().to_path_buf();
				}
		}

		return dir.to_str().unwrap().to_string();
}
