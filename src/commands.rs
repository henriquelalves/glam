use inquire::{Select, Text, MultiSelect};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::write;
use std::path::{Path, PathBuf};
use std::process::exit;

#[path = "content.rs"]
mod content;
#[path = "utils.rs"]
mod utils;

#[derive(Serialize, Deserialize)]
struct GlamObject {
    packages: Vec<GlamPackage>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GlamPackage {
    name: String,
    git_repo: String,
    #[serde(default = "default_string")]
    commit: String,
    #[serde(default)]
    links: Vec<Link>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Link {
    target_folder: String,
    source_folder: String,
}


fn default_string() -> String {
    return "".to_string();
}

pub fn search_project_root() -> String {
    let path = PathBuf::from("./");
    let mut dir = path.canonicalize().unwrap();

    loop {
        let dir_path = dir.to_str().unwrap();
        let proj_path = format!("{}/project.godot", dir_path);

        let godot_project = Path::new(&proj_path);

        if godot_project.exists() {
            break;
        } else {
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

pub fn initialize(root: &str) {
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
}

pub fn initialize_glam_files(root: &str) {
    // Create glam.d/ folder if it doesn't exist
    if !Path::new(&format!("{}/.glam.d/", root)).exists() {
        let res = utils::run_shell_command("mkdir -p .glam.d", &root, false);

        utils::assert_result(&res, "Couldn't create .glam.d/ folder!");

        let gd_ignore = &format!("{}/.glam.d/.gdignore", &root);
        if !Path::new(gd_ignore).exists() {
            match write(gd_ignore, content::create_gdignore_file()) {
                Ok(_v) => (),
                Err(_e) => {
                    utils::log_error("There was a problem creating the .gdignore file!");
                    exit(1);
                }
            }
        }

        utils::log_info("Created .glam.d/ folder");
    }

    // Create .glam file if it doesn't exist
    if !Path::new(&format!("{}/.glam", root)).exists() {
        fs::write(&format!("{}/.glam", root), content::create_glam_file())
            .expect("Couldn't create .glam file!");
        utils::log_info("Created .glam file");
    }
}

pub fn check_initialization(root: &str) -> bool {
    let git_ignore = &format!("{}/.gitignore", &root);
    if !Path::new(git_ignore).exists() {
        utils::log_warning(".gitignore file does not exist!");
    }

    let mut ret = true;
    let glam_file = &format!("{}/.glam", &root);
    if !Path::new(glam_file).exists() {
        utils::log_error(".glam file does not exist!");
        ret = false;
    }

    let glam_folder = &format!("{}/.glam.d/", &root);
    if !Path::new(glam_folder).exists() {
        utils::log_error(".glam.d/ folder does not exist!");
        ret = false;
    }
    return ret;
}

pub fn install_repositories(root: &str, verbose: bool) {
    let glam_file_path = format!("{}/.glam", root);
    let mut glam_object = read_glam_file(&glam_file_path);
    let mut glam_packages = glam_object.packages;

    for package in glam_packages.iter_mut() {
        utils::log_info(&format!("Installing {}...", package.name));
        clone_or_fetch_package(root, package, verbose);
        let commit = package.commit.to_string();
        install_glam_package(root, &commit, package, false, true, verbose);
    }

    glam_object.packages = glam_packages;
    write_glam_file(&glam_file_path, &glam_object);
}

pub fn add_repository(root: &str, git_repo: &str, verbose: bool) {
    let glam_file_path = format!("{}/.glam", root);
    let mut glam_object = read_glam_file(&glam_file_path);
    let mut glam_packages = glam_object.packages;

    if find_package_by_repository(&glam_packages, &git_repo).is_some() {
        utils::log_error("Repository already exists!");
        exit(1);
    }

    let default_name: String = utils::get_repo_name(git_repo);
    let inquire_name = "Name of the addon:";
    let name = Text::new(inquire_name)
        .with_default(&default_name)
        .with_placeholder(&default_name)
        .prompt()
        .unwrap();

    if find_package_by_name(&glam_packages, &name).is_some() {
        utils::log_error("Addon name exists!");
        exit(1);
    }

    let default_commit = "latest";
    let inquire_commit = "Commit hash of the repository:";
    let commit = Text::new(inquire_commit)
        .with_default(&default_commit)
        .with_placeholder(&default_commit)
        .prompt()
        .unwrap();

    glam_packages.push(GlamPackage {
        name: name.to_string(),
        git_repo: git_repo.to_string(),
        commit: commit.to_string(),
        links: [].to_vec(),
    });

    let target_package = glam_packages.last_mut().unwrap();

    clone_or_fetch_package(root, target_package, verbose);
    install_glam_package(root, &commit, target_package, false, true, verbose);

    glam_object.packages = glam_packages;
    write_glam_file(&glam_file_path, &glam_object);
}

pub fn create_addon(root: &str, verbose: bool) {
    let glam_file_path = format!("{}/.glam", root);
    let mut glam_object = read_glam_file(&glam_file_path);
    let mut glam_packages = glam_object.packages;

    let folders = list_addons(root, verbose);
    
    let addon_name = Select::new("Which addon you'll create a repository?", folders)
        .prompt()
        .unwrap();

    if find_package_by_link(&glam_packages, &addon_name).is_some() {
        utils::log_error("There is a repository linked to that addon already!");
        exit(1);
    }

    let repo_name = Text::new("Name of the repository:")
        .with_default(&addon_name)
        .with_placeholder(&addon_name)
        .prompt()
        .unwrap();
    
    let res = utils::run_shell_command(
        &format!("mkdir -p .glam.d/{}/addons/{}", repo_name, addon_name),
        &root,
        verbose
    );

    utils::assert_result(&res, "Repository folder failed to be created!");

    let res = utils::run_shell_command(
        &format!("cd .glam.d/{} && git init", repo_name),
        &root,
        verbose,
    );

    utils::assert_result(&res, "Repository failed to be initialized!");
    
    glam_packages.push(GlamPackage {
        name: repo_name.to_string(),
        git_repo: "".to_string(),
        commit: "".to_string(),
        links: [Link{
            target_folder: format!("addons/{}", addon_name),
            source_folder: format!("addons/{}", addon_name),
        }].to_vec(),
    });    
    
    glam_object.packages = glam_packages;
    write_glam_file(&glam_file_path, &glam_object);

    let target_package = glam_object.packages.last_mut().unwrap();
    
    apply_package_files(&root, &target_package, verbose);
}

pub fn update_repository(root: &str, verbose: bool) {
    let glam_file_path = format!("{}/.glam", root);
    let mut glam_object = read_glam_file(&glam_file_path);
    let mut glam_packages = glam_object.packages;

    if glam_packages.is_empty() {
        utils::log_error("No addons to update!")
    }

    let options = glam_packages
        .iter()
        .map(|x| -> &str { &x.name })
        .collect::<Vec<&str>>();

    if options.len() == 0 {
        utils::log_error("No repository to update!");
        exit(1);
    }

    let ans = Select::new("Which addon you want to update?", options)
        .prompt()
        .unwrap();

    let package_index = find_package_by_name(&glam_packages, ans).unwrap();
    let target_package = &mut glam_packages[package_index];

    utils::log_info(&format!("Updating {}...", target_package.name));
    clone_or_fetch_package(root, target_package, verbose);
    install_glam_package(root, "", target_package, true, true, verbose);

    glam_object.packages = glam_packages;
    write_glam_file(&glam_file_path, &glam_object);
}

pub fn apply_changes(root: &str, verbose: bool) {
    let glam_file_path = format!("{}/.glam", root);
    let mut glam_object = read_glam_file(&glam_file_path);
    let mut glam_packages = glam_object.packages;

    if glam_packages.is_empty() {
        utils::log_error("No addons to apply changes!")
    }

    let options = glam_packages
        .iter()
        .map(|x| -> &str { &x.name })
        .collect::<Vec<&str>>();

    let ans = Select::new("Which addon you want to apply changes?", options)
        .prompt()
        .unwrap();

    let package_index = find_package_by_name(&glam_packages, ans).unwrap();
    let target_package = &mut glam_packages[package_index];

    apply_package_files(root, target_package, verbose);

    glam_object.packages = glam_packages;
    write_glam_file(&glam_file_path, &glam_object);
}

fn find_package_by_link(packages: &Vec<GlamPackage>, addons_folder: &str) -> Option<usize> {
    let mut package_index = 0;
    let mut found_package = false;

    for (i, package) in packages.iter().enumerate() {
        for (_j, link) in package.links.iter().enumerate() {
            if link.target_folder == addons_folder {
                package_index = i;
                found_package = true;
            }
        }
    }

    if found_package {
        return Some(package_index);
    }

    return None;    
}

fn find_package_by_name(packages: &Vec<GlamPackage>, name: &str) -> Option<usize> {
    let mut package_index = 0;
    let mut found_package = false;

    for (i, package) in packages.iter().enumerate() {
        if package.name == name {
            package_index = i;
            found_package = true;
        }
    }

    if found_package {
        return Some(package_index);
    }

    return None;
}

fn find_package_by_repository(packages: &Vec<GlamPackage>, repo: &str) -> Option<usize> {
    let mut package_index = 0;
    let mut found_package = false;

    for (i, package) in packages.iter().enumerate() {
        if package.git_repo == repo {
            package_index = i;
            found_package = true;
        }
    }

    if found_package {
        return Some(package_index);
    }

    return None;
}

fn list_addons(root: &str, verbose: bool) -> Vec<String> {
    let res = utils::run_shell_command(
        &format!("ls addons"),
        &root,
        verbose,
    );

    utils::assert_result(&res, "Addons folder doesn't exist!");

    let addon_folders = res.unwrap().trim().to_string();
    let split = addon_folders.split("\n").collect::<Vec<&str>>();
    
    return split.iter().map(|s| s.to_string()).collect();
}

fn install_glam_package(
    root: &str,
    commit: &str,
    package: &mut GlamPackage,
    update_package: bool,
    copy_files: bool,
    verbose: bool,
) {
    // Update package folder to commit hash
    if update_package {
        package.commit = "latest".to_string();
    }

    if commit != "latest" {
        package.commit = commit.to_string();
    }

    // TODO Get all folders on addon
    let res = utils::run_shell_command(
        &format!("ls .glam.d/{}/addons", package.name),
        &root,
        verbose,
    );

    if res.is_err() {
        utils::log_error("Couldn't get addon name.");
        exit(1);
    }

    if package.links.is_empty() {
        let addon_folders = res.unwrap().trim().to_string();
        let folders = addon_folders.split("\n").collect::<Vec<&str>>();

        if folders.len() == 1 {
            package.links.push(
                Link {
                    target_folder: format!("addons/{}", folders[0]),
                    source_folder: format!("addons/{}", folders[0]),
                }
            );
        } else {
            let ans = MultiSelect::new("Which addons you'd like to import?", folders)
                .prompt()
                .unwrap();

            if ans.len() == 0 {
                utils::log_error("No addon selected!");
                exit(0);
            }

            for folder in ans {
                package.links.push(
                    Link {
                        target_folder: format!("addons/{}", folder),
                        source_folder: format!("addons/{}", folder),
                    }
                );
            }
        }
    }

    if package.commit == "latest" {
        let res = utils::run_shell_command(
            &format!("cd .glam.d/{} && git rev-parse HEAD", package.name),
            &root,
            verbose,
        )
        .unwrap();
        package.commit = res.trim().to_string();
    } else {
        utils::log_info("Git checkout to package commit");
        let res = utils::run_shell_command(
            &format!(
                "cd .glam.d/{} && git reset --hard {}",
                package.name, package.commit
            ),
            &root,
            verbose,
        );

        utils::assert_result(&res, "Couldn't checkout repository!");
    }

    if copy_files {
        for link in &package.links {
            // If project addon folder doesn't exist, create it
            let res = utils::run_shell_command(
                &format!("mkdir -p {}", link.target_folder),
                &root,
                verbose,
            );

            utils::assert_result(&res, "Couldn't create addons folder!");
        }


        // TODO: Why this is throwing Err("cp: -t: No such file or directory\n") on mac?
        // println!(
        //     "cp -rf .glam.d/{}/{}/* -t {}",
        //     package.name, package.source_folder, package.target_folder
        // );

        // Copy addon repository content to target folder
        for link in &package.links {
            let source_folder = &link.source_folder;
            let target_folder = &link.target_folder;

            let res = utils::run_shell_command(
                &format!(
                    "cp -rf .glam.d/{}/{}/* -t {}",
                    package.name, source_folder, target_folder
                ),
                &root,
                verbose,
            );
            utils::assert_result(&res, "Couldn't copy files to addons!");
        }
    }
}

fn apply_package_files(root: &str, package: &GlamPackage, verbose: bool) {
    for link in &package.links {
        // Overwrite source folder with target folder
        let res = utils::run_shell_command(
            &format!(
                "for f in $(ls .glam.d/{}/{}); do rm -rf .glam.d/{}/{}/$f; done",
                package.name, link.source_folder, package.name, link.source_folder
            ),
            &root,
            verbose,
        );
        utils::assert_result(&res, "Couldn't overwrite source folder files!");

        // Copy addon repository content to target folder
        let res = utils::run_shell_command(
            &format!(
                "for f in $(ls ./{}); do cp -rf ./{}/$f ./.glam.d/{}/{}/$f; done",
                link.target_folder, link.target_folder, package.name, link.source_folder
            ),
            &root,
            verbose,
        );

        utils::assert_result(&res, "Couldn't copy files to repository!");
    }
}

fn clone_or_fetch_package(root: &str, package: &mut GlamPackage, verbose: bool) {
    // If glam package folder doesn't exist, clone project
    if !Path::new(&format!("{}/.glam.d/{}", root, package.name)).exists() {
        let res = utils::run_shell_command(
            &format!(
                "cd .glam.d/ && git clone {} {} --progress",
                package.git_repo, package.name
            ),
            &root,
            verbose,
        );

        utils::assert_result(&res, "Couldn't clone repository!");
        utils::log_check("Created package folder on .glam.d");
    } else {
        if package.git_repo == "" {
            let res = utils::run_shell_command(
                &format!("cd .glam.d/{} && git remote get-url origin", package.name),
                &root,
                verbose,
            );

            if res.is_err() {
                utils::log_error("GLAM Package has no origin yet!");
                exit(1);
            }

            package.git_repo = res.unwrap().trim().to_string();
        }

        let res = utils::run_shell_command(
            &format!(
                "cd .glam.d/{} && git fetch origin && git pull",
                package.name
            ),
            &root,
            verbose,
        );
        utils::assert_result(&res, "Couldn't fetch package repository updates!");
        utils::log_info("Glam package folder already exists, fetched and pulled latest changes");
    }
}

fn read_glam_file(file_path: &str) -> GlamObject {
    if !Path::new(file_path).exists() {
        fs::write(file_path, content::create_glam_file()).expect("Couldn't create .glam file!");
    }

    let glam_content = fs::read_to_string(file_path).expect("Couldn't read .glam file!");
    let glam_obj: GlamObject = serde_json::from_str(&glam_content).unwrap();

    return glam_obj;
}

fn write_glam_file(file_path: &str, glam_object: &GlamObject) {
    let json_string = serde_json::to_string_pretty(glam_object).unwrap();
    fs::write(file_path, json_string).expect("Couldn't create .glam file!");
}
