use colored::Colorize;
use std::io::Write;

pub fn log_warning(msg: &str) {
		let msg = format!("⚠️ {}", msg).yellow();
		println!("{}", msg);
}

pub fn log_error(msg: &str) {
		let msg = format!("❌ {}", msg).red();
		println!("{}", msg);
}

pub fn log_info(msg: &str) {
		let msg = format!("ℹ️ {}", msg).blue();
		println!("{}", msg);
}

pub fn log_check(msg: &str) {
		let msg = format!("✅ {}", msg).green();
		println!("{}", msg);
}

pub fn get_repo_name(repo: &str) -> String {
		let mut chars = repo.chars().rev();
		let length = repo.chars().count();
		let mut last_i = 0;
		let mut first_i = 0;

		let mut i = length;

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

		let name = &repo[first_i..last_i];
		// TODO: Return a Result (may be error)
		return name.to_string();
}

pub fn run_shell_command(command : &str, folder : &str, verbose : bool) -> Result<String, String> {
		let output = std::process::Command::
    new("sh")
				.current_dir(folder)
				.stdin(std::process::Stdio::inherit())
        .arg("-c")
        .arg(command)
        .output()
        .expect("Error running command.");

		if verbose {
				std::io::stdout().write_all(&output.stdout).unwrap();
				std::io::stderr().write_all(&output.stderr).unwrap();
		}

		match output.status.success() {
				true => {
						let stdout_str = String::from_utf8(output.stdout).unwrap();
						return Ok(stdout_str);
				}

				false => {
						let stderr_str = String::from_utf8(output.stderr).unwrap();
						return Err(stderr_str);
				}
		}
}
