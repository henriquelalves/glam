pub fn create_gitignore_file() -> String {
		return
r#"# Godot-specific ignores
.import/
export.cfg
export_presets.cfg

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
r#"# Glam-specific ignores
.glam.d/"#.to_string();
}

pub fn create_glam_file() -> String {
		return
r#"{
    "packages" : {
    }
}
"#.to_string();
}
