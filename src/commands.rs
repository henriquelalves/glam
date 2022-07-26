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

pub fn install_all_packages(root : &str, verbose : bool) {
		let glam_file_path = format!("{}/.glam", root);

		// Find glam object or create one with default configuration
		let glam_object = read_glam_file(&glam_file_path);
		let mut glam_packages = glam_object.packages;

		for package in glam_packages.iter_mut() {
				clone_or_fetch_package(root, &package, verbose);
				let commit = package.commit.to_string();
				install_glam_package(root, &commit, package, false, true, verbose);
		}
}

pub fn install_package(root : &str, git_repo : &str, commit : &str, copy_files : bool, verbose : bool) {

		let glam_file_path = format!("{}/.glam", root);

		// Find glam object or create one with default configuration
		let mut glam_object = read_glam_file(&glam_file_path);
		let mut glam_packages = glam_object.packages;

		let name = utils::get_repo_name(git_repo);
		let (_, target_package) = find_package(
				&mut glam_packages,
				&name,
				Some(GlamPackage{
						git_repo : git_repo.to_string(),
						name : name.to_string(),
						commit : "".to_string(),
				}));

		clone_or_fetch_package(root, &target_package, verbose);

		// Update package folder to commit hash
		install_glam_package(root, commit, target_package, false, copy_files, verbose);

		// Write .glam file
		glam_object.packages = glam_packages;
		write_glam_file(&glam_file_path, &glam_object);
}

pub fn update_all_packages(root : &str, verbose : bool) {
		let glam_file_path = format!("{}/.glam", root);

		// Find glam object or create one with default configuration
		let mut glam_object = read_glam_file(&glam_file_path);
		let mut glam_packages = glam_object.packages;

		for package in glam_packages.iter_mut() {
				clone_or_fetch_package(root, &package, verbose);
				install_glam_package(root, "", package, true, true, verbose);
		}

		glam_object.packages = glam_packages;
		write_glam_file(&glam_file_path, &glam_object);
}

pub fn update_package(root : &str, package_name : &str, verbose : bool) {
		let glam_file_path = format!("{}/.glam", root);

		// Find package to update

		let mut glam_object = read_glam_file(&glam_file_path);
		let mut glam_packages = glam_object.packages;

		let (_, mut target_package) = find_package(&mut glam_packages, package_name, None);

		clone_or_fetch_package(root, &target_package, verbose);
		install_glam_package(root, "", &mut target_package, true, true, verbose);

		// Write .glam file
		glam_object.packages = glam_packages;
		write_glam_file(&glam_file_path, &glam_object);
}

pub fn remove_package(root : &str, package_name : &str, verbose : bool) {
		let glam_file_path = format!("{}/.glam", root);

		// Find package to update

		let mut glam_object = read_glam_file(&glam_file_path);
		let mut glam_packages = glam_object.packages;
		let (index, target_package) = find_package(&mut glam_packages, package_name, None);

		remove_glam_package_files(root, &target_package, verbose);

		glam_packages.remove(index);

		// Write .glam file
		glam_object.packages = glam_packages;
		write_glam_file(&glam_file_path, &glam_object);
}

pub fn apply_changes(root : &str, package_name : &str, verbose : bool) {
		let glam_file_path = format!("{}/.glam", root);

		// Find package to update

		let glam_object = read_glam_file(&glam_file_path);
		let mut glam_packages = glam_object.packages;
		let (_, target_package) = find_package(&mut glam_packages, package_name, None);

		apply_glam_package_files(root, &target_package, verbose);
}

fn find_package<'a>(packages : &'a mut Vec<GlamPackage>, name : &str, new_package : Option<GlamPackage>) -> (usize, &'a mut GlamPackage) {
		let mut package_index = 0;
		let mut found_package = false;

		for (i, package) in packages.iter().enumerate() {
				if package.name == name {
						package_index = i;
						found_package = true;
				}
		}

		if !found_package {
				match new_package {
						None => {
								utils::log_error("Package not found!");
								exit(1);
						},
						Some(package) => {
								packages.push(package);
								package_index = packages.len() - 1;
						}
				}
		}

		return (package_index, &mut packages[package_index]);
}

fn install_glam_package(root : &str, commit : &str, package : &mut GlamPackage, update_package : bool, copy_files : bool, verbose : bool) {
		// Update package folder to commit hash
		if update_package {
				package.commit = "".to_string();
		}

		if commit != "" {
				package.commit = commit.to_string();
		}

		if package.commit == "" {
				let res = utils::run_shell_command(
						&format!("cd .glam.d/{} && git rev-parse HEAD",
										 package.name),
						&root,
						verbose).unwrap();
				package.commit = res.trim().to_string();
		} else {
				utils::log_info("Git checkout to package commit");
				let res = utils::run_shell_command(
						&format!("cd .glam.d/{} && git checkout {}",
										 package.name,
										 package.commit),
						&root,
						verbose);

				utils::assert_res(&res, "Couldn't checkout repository!");
		}

		if copy_files {
				// If project addon folder doesn't exist, create it
				let res = utils::run_shell_command(
						&format!("mkdir -p {}/addons/{}", root, package.name),
						&root,
						verbose
				);

				utils::assert_res(&res, "Couldn't create addons folder!");

				// TODO: use source_folder to copy files from (default: /addons/)
				// TODO: use target_folder to copy files to (defautl: (root)/)
				// Copy addon repository content to target folder
				let res = utils::run_shell_command(
						&format!("cp -rf .glam.d/{}/addons/* -t {}/addons/", package.name, root),
						&root,
						verbose
				);

				utils::assert_res(&res, "Couldn't copy files to addons!");
		}
}

fn apply_glam_package_files(root : &str, package : &GlamPackage, verbose : bool) {
		// TODO: use source_folder to copy files from (default: /addons/)
		// TODO: use target_folder to copy files to (defautl: (root)/)
		// Copy addon repository content to target folder
		let res = utils::run_shell_command(
				&format!("for f in $(ls ./.glam.d/{}/addons/); do cp -rf ./addons/$f ./.glam.d/{}/addons/; done", package.name, package.name),
				&root,
				verbose
		);

		utils::assert_res(&res, "Couldn't copy files to repository!");
}

fn remove_glam_package_files(root : &str, package : &GlamPackage, verbose : bool) {
		// TODO: use source_folder to copy files from (default: /addons/)
		// TODO: use target_folder to copy files to (defautl: (root)/)
		// Copy addon repository content to target folder
		let res = utils::run_shell_command(
				&format!("for f in $(ls ./.glam.d/{}/addons/); do rm -rf ./addons/$f ; done", package.name),
				&root,
				verbose
		);

		utils::assert_res(&res, "Couldn't remove files from root addons!");

		let res = utils::run_shell_command(
				&format!("rm -rf ./.glam.d/{}", package.name),
				&root,
				verbose
		);

		utils::assert_res(&res, "Couldn't copy files to addons!");
}

pub fn initialize_glam_files(root : &str) {
		// Create glam.d/ folder if it doesn't exist
		if !Path::new(&format!("{}/.glam.d/", root)).exists() {
				let res = utils::run_shell_command(
						&format!("mkdir -p {}/.glam.d/", root),
						&root,
						false
				);

				utils::assert_res(&res, "Couldn't create .glam.d/ folder!");

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
				let res = utils::run_shell_command(
						&format!("cd .glam.d/ && git clone {} {} --progress", package.git_repo, package.name),
						&root,
						verbose
				);

				utils::assert_res(&res, "Couldn't clone repository!");

				utils::log_check("Created package folder on .glam.d");
		} else {
				let res = utils::run_shell_command(
						&format!("cd .glam.d/{} && git fetch origin && git pull", package.name),
						&root,
						verbose
				);
				utils::assert_res(&res, "Couldn't fetch package repository updates!");

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
