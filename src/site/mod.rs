use std::fs;
use std::path::Path;
use std::io::{ Read, Write };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub site_name: String,
  pub site_theme: String,
  pub site_author: String,
}

#[derive(Debug)]
pub struct Website {
  pub name: String,
  pub theme: String,
  pub posts: Vec<Post>
}

#[derive(Deserialize, Debug)]
pub struct Post {
  pub title: String,
  pub slug: String,
  pub author: String,
  pub pubdate: String,

  #[serde(default)]
  pub draft: bool,

  #[serde(skip_deserializing)]
  pub content: String
}

impl Config {
  pub fn save_to_disk(&self, project_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let toml = toml::to_string(&self)?;
    let mut file = fs::File::create(project_dir.join("config.toml"))?;
    file.write(toml.as_bytes())?;
    
    Ok(())
  }
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

    let mut posts: Vec<Post> = vec![];
    for entry in fs::read_dir(project_dir.join("content/"))? {
      let entry = entry?;
      let path = entry.path();

      if path.is_dir() {
        println!("Skipping {:?}", path);
        continue; // TODO: Implement subdirectory generation
      } else {
        println!("processing {:?} ...", path);
        posts.push(Post::from_file(&path)?);
      }
    }

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

    // process posts
    

    Ok(())
  }
}

impl Post {
  pub fn from_file(path: &Path) -> Result<Post, Box<dyn std::error::Error>> {
    // get the file contents into a string
    let mut file = fs::File::open(path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    let file_content = file_content;

    // scan the document and separate the metadata from the content body
    let mut buffer = String::new();
    let mut metadata = String::new();
    let mut scanning_metadata = false;
    for line in file_content.lines() {
      // metadata is separated by a "---" section at the very top
      if line.contains("---") {
        match scanning_metadata {
          true => {
            metadata = buffer.clone();
            buffer.clear();
          },
          false => scanning_metadata = true
        }
      } else {
        buffer.push_str(line);
        buffer.push_str("\n");
      }
    }

    // ensure we have data to convert
    if metadata.len() == 0 {
      Err("Post is missing metadata.")?
    }

    // deserialize the metadata and clone the buffer into the post's contents
    let mut post: Post = toml::from_str(&metadata)?;
    post.content = buffer.clone();

    Ok(post)
  }
}