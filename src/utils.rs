pub fn get_repo_name(repo: &str) -> String {
		let mut chars = repo.chars().rev();
		let length = repo.chars().count();
		let mut last_i = 0;
		let mut first_i = 0;

		let mut i = length;
		println!("{}", i);

		while i > 0 {
				match chars.next() {
						Some('.') => {
								last_i = i-1;
						}
						Some('/') => {
								first_i = i;
								break;
						}
						_ => {}
				}
				i -= 1;
		}

		if last_i == 0 {
				last_i = length;
		}

		println!("{} {}", first_i, last_i);
		let name = &repo[first_i..last_i];
		println!("{}", name);
		// TODO: Return a Result (may be error)
		return name.to_string();
}

pub fn run_shell_command(command : &str, folder : Option<&str>) -> bool {
		let status = std::process::Command::
    new("sh")
				.current_dir(folder.unwrap_or("./"))
        .arg("-c")
        .arg(command)
        .status()
        .expect("Error running command.");
		return status.success();
}
