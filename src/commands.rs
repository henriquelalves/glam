use std::path::{Path, PathBuf};
use std::fs;
use std::fs::write;
use std::process::exit;
use serde::{Serialize, Deserialize};

#[path = "utils.rs"] mod utils;
#[path = "content.rs"] mod content;

#[derive(Serialize, Deserialize)]
struct GlamObject {
		packages: Vec<GlamPackage>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GlamPackage {
		name : String,
		git_repo : String,
		commit : String,
}

pub fn initialize(root : &str) {
		let git_ignore = &format!("{}/.gitignore", &root);
		if !Path::new(git_ignore).exists() {
				match write(git_ignore, content::create_gitignore_file()) {
						Ok(_v) => (),
						Err(_e) => {
								utils::log_error("There was a problem creating the .gitignore file!");
								exit(1);
						}
				}
		}

		let gd_ignore = &format!("{}/.gdignore", &root);
		if !Path::new(gd_ignore).exists() {
				match write(gd_ignore, content::create_gdignore_file()) {
						Ok(_v) => (),
						Err(_e) => {
								utils::log_error("There was a problem creating the .gdignore file!");
								exit(1);
						}
				}
		}
}

pub fn check_ignores(root : &str) {
		let git_ignore = &format!("{}/.gitignore", &root);
		if !Path::new(git_ignore).exists() {
				utils::log_warning(".gitignore file does not exist!");
		}

		let gd_ignore = &format!("{}/.gdignore", &root);
		if !Path::new(gd_ignore).exists() {
				utils::log_warning(".gdignore file does not exist!");
		}
}

pub fn install(root : &str, verbose : bool) {
		let glam_file_path = format!("{}/.glam", root);

		// Find glam object or create one with default configuration
		let glam_object = read_glam_file(&glam_file_path);
		let glam_packages = glam_object.packages;

		for package in glam_packages.iter() {
				install_package(root, &package.git_repo, "", verbose);
		}
}

pub fn install_package(root : &str, git_repo : &str, commit : &str, verbose : bool) {

		let glam_file_path = format!("{}/.glam", root);

		// Find glam object or create one with default configuration
		let mut glam_object = read_glam_file(&glam_file_path);
		let mut glam_packages = glam_object.packages;
		let mut package_index = 0;
		let mut found_package = false;

		let name = utils::get_repo_name(git_repo);

		for (i, package) in glam_packages.iter().enumerate() {
				if package.name == name {
						package_index = i;
						found_package = true;
				}
		}

		if !found_package {
				let package = GlamPackage{
						git_repo : git_repo.to_string(),
						name : name.to_string(),
						commit : "".to_string(),
				};

				glam_packages.push(package);
				package_index = glam_packages.len() - 1;
		}

		let target_package = &mut glam_packages[package_index];

		clone_or_fetch_package(root, &target_package, verbose);

		// Update package folder to commit hash
		if commit != "" {
				target_package.commit = commit.to_string();
		}

		if target_package.commit == "" {
				let res = utils::run_shell_command(
						&format!("cd .glam.d/{} && git rev-parse HEAD",
										 target_package.name),
						&root,
						verbose).unwrap();
				target_package.commit = res.trim().to_string();
		} else {
				utils::log_info("Git checkout to package commit");
				utils::run_shell_command(
						&format!("cd .glam.d/{} && git checkout {}",
										 target_package.name,
										 target_package.commit),
						&root,
						verbose);
		}

		// If project addon folder doesn't exist, create it
		utils::run_shell_command(
				&format!("mkdir -p {}/addons/{}", root, name),
				&root,
				verbose
		);

		// TODO: use source_folder to copy files from (default: /addons/)
		// TODO: use target_folder to copy files to (defautl: (root)/)
		// Copy addon repository content to target folder
		utils::run_shell_command(
				&format!("cp -f -r .glam.d/{}/addons/* -t {}/addons/", name, root),
				&root,
				verbose
		);

		// Write .glam file
		glam_object.packages = glam_packages;
		write_glam_file(&glam_file_path, &glam_object);
}

pub fn update_all_packages(root : &str, verbose : bool) {
		let glam_file_path = format!("{}/.glam", root);

		// Find glam object or create one with default configuration
		let glam_object = read_glam_file(&glam_file_path);
		let glam_packages = glam_object.packages;

		for package in glam_packages.iter() {
				install_package(root, &package.git_repo, "", verbose);
		}
}

pub fn update_package(root : &str, package_name : &str, verbose : bool) {
		let glam_file_path = format!("{}/.glam", root);

		// Find package to update

		let mut glam_object = read_glam_file(&glam_file_path);
		let mut glam_packages = glam_object.packages;
		let mut package_index = 0;
		let mut found_package = false;

		for (i, package) in glam_packages.iter().enumerate() {
				if package.name == package_name {
						package_index = i;
						found_package = true;
				}
		}

		if !found_package {
				utils::log_error("Package not found!");
				exit(1);
		}

		let target_package = &mut glam_packages[package_index];

		clone_or_fetch_package(root, &target_package, verbose);

		let res = utils::run_shell_command(
				&format!("cd .glam.d/{} && git rev-parse HEAD",
								 target_package.name),
				&root,
				verbose).unwrap();
		target_package.commit = res.trim().to_string();

		// If project addon folder doesn't exist, create it
		utils::run_shell_command(
				&format!("mkdir -p {}/addons/{}", root, target_package.name),
				&root,
				verbose
		);

		// TODO: use source_folder to copy files from (default: /addons/)
		// TODO: use target_folder to copy files to (defautl: (root)/)
		// Copy addon repository content to target folder
		utils::run_shell_command(
				&format!("cp -f -r .glam.d/{}/addons/* -t {}/addons/", target_package.name, root),
				&root,
				verbose
		);

		// Write .glam file
		glam_object.packages = glam_packages;
		write_glam_file(&glam_file_path, &glam_object);
}

pub fn remove_package(root : &str, package : &str, verbose : bool) {
}

pub fn apply_changes(root : &str, package : &str, verbose : bool) {

}

pub fn initialize_glam_files(root : &str) {
		// Create glam.d/ folder if it doesn't exist
		if !Path::new(&format!("{}/.glam.d/", root)).exists() {
				utils::run_shell_command(
						&format!("mkdir -p {}/.glam.d/", root),
						&root,
						false
				);
				utils::log_info("Created .glam.d/ folder");
		}

		// Create .glam file if it doesn't exist
		if !Path::new(&format!("{}/.glam", root)).exists() {
				fs::write(&format!("{}/.glam",root),
									content::create_glam_file())
						.expect("Couldn't create .glam file!");
				utils::log_info("Created .glam file");
		}
}

fn clone_or_fetch_package(root : &str, package : &GlamPackage, verbose : bool) {
		// If glam package folder doesn't exist, clone project
		if !Path::new(&format!("{}/.glam.d/{}", root, package.name)).exists() {
				utils::run_shell_command(
						&format!("cd .glam.d/ && git clone {} {} --progress", package.git_repo, package.name),
						&root,
						verbose
				);
				utils::log_check("Created package folder on .glam.d");
		} else {
				utils::run_shell_command(
						&format!("cd .glam.d/{} && git fetch origin && git pull", package.name),
						&root,
						verbose
				);

				utils::log_info("Glam package folder already exists, fetched and pulled latest changes");
		}
}

fn read_glam_file(file_path : &str) -> GlamObject {
		if !Path::new(file_path).exists() {
				fs::write("./.glam", content::create_glam_file()).expect("Couldn't create .glam file!");
		}

		let glam_content = fs::read_to_string("./.glam").expect("Couldn't read .glam file!");
		let glam_obj : GlamObject = serde_json::from_str(&glam_content).unwrap();

		return glam_obj;
}

fn write_glam_file(file_path : &str, glam_object : &GlamObject) {
		let json_string = serde_json::to_string_pretty(glam_object).unwrap();
		fs::write(file_path, json_string).expect("Couldn't create .glam file!");
}

pub fn search_project_root() -> String{
		let path = PathBuf::from("./");
		let mut dir = path.canonicalize().unwrap();

		loop {
				let dir_path = dir.to_str().unwrap();
				let proj_path = format!("{}/project.godot", dir_path);

				let godot_project = Path::new(&proj_path);

				if godot_project.exists() {
						break;
				}
				else {
						let parent = dir.parent();
						if parent.is_none() {
								utils::log_error("Godot project not found!");
								exit(1);
						}
						dir = dir.parent().unwrap().to_path_buf();
				}
		}

		let root = dir.to_str().unwrap().to_string();
		utils::log_check(&format!("Found root project in: {}", root));
		return root;
}
