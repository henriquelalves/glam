pub fn create_gitignore_file() -> String {
		return
r#"# Godot 4+ ignores
.godot/

# Godot 3 ignores
.import/

# Imported translations (automatically generated from CSV files)
*.translation

# Mono-specific ignores
.mono/
data_*/

# Glam-specific ignores
.glam.d/"#.to_string();
}

pub fn create_gdignore_file() -> String {
		return
r#"# Hide this folder from Godot editor"#.to_string();
}

pub fn create_glam_file() -> String {
		return
r#"{
    "packages" : [
    ]
}
"#.to_string();
}
