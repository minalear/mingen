use std::fs;
use std::path::Path;
use std::io::{ Read, Write };
use serde::{ Serialize, Deserialize };

#[derive(Debug)]
pub struct Website {
  name: String,
  theme: String,
  posts: Vec<Post>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  site_name: String,
  site_theme: String,
  site_author: String,
}

#[derive(Debug)]
pub struct Post {
  author: String,
  pubdate: String,
  title: String,
  content: String,
  draft: bool
}

impl Website {
  pub fn new(project_dir: &Path) -> Result<Website, Box<dyn std::error::Error>> {
    // paths for new project
    let directories = [
      project_dir.to_path_buf(),
      project_dir.join("content"),
      project_dir.join("partials"),
      project_dir.join("static"),
      project_dir.join("static/styles"),
      project_dir.join("static/images"),
      project_dir.join("themes")
    ];

    for dir in directories.iter() {
      // println!("creating directory {:?}...", dir);
      fs::create_dir(dir)?;
    }

    let config = Config {
      site_name: String::from("Website Name"),
      site_theme: String::from("default"),
      site_author: String::from("First Last")
    };
    config.save_to_disk(project_dir)?;
    
    Ok(Website {
      name: config.site_name.clone(),
      theme: config.site_theme.clone(),
      posts: vec![]
    })
  }

  pub fn from_project(project_dir: &Path) -> Result<Website, Box<dyn std::error::Error>> {
    let mut config_file = fs::File::open(project_dir.join("config.toml"))?;
    let mut config = String::new();
    config_file.read_to_string(&mut config)?;
    
    let config: Config = toml::from_str(&config)?;

    // TODO: Find all posts and add them to vector
    Ok(Website{
      name: config.site_name,
      theme: config.site_theme,
      posts: vec![]
    })
  }

  pub fn generate(&self, project_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = project_dir.join("output/");
    
    // delete any previous exports
    if output_dir.exists() {
      fs::remove_dir_all(output_dir)?;
    }

    

    Ok(())
  }
}

impl Config {
  pub fn save_to_disk(&self, project_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let toml = toml::to_string(&self)?;
    let mut file = fs::File::create(project_dir.join("config.toml"))?;
    file.write(toml.as_bytes())?;
    
    Ok(())
  }
}