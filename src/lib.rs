use std::fs;
use std::fs::File;
use std::io::{ Write };
use std::path::Path;
use serde::{ Serialize, Deserialize };

/// Generates a new project with necessary file structure and config files
pub fn gen_new_site(project_dir: &Path) {
  fs::create_dir(project_dir).unwrap();
  create_config_file(project_dir);
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectConfig {
  name: String,
  author: String,
}

fn create_config_file(project_dir: &Path) {
  let default_config = ProjectConfig {
    name: String::from(project_dir.to_str().unwrap()),
    author: String::from("Author")
  };

  let toml = toml::to_string(&default_config).unwrap();
  let mut config_file = File::create(project_dir.join("config.toml")).unwrap();
  config_file.write(toml.as_bytes()).unwrap();
}