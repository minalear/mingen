#[macro_use]
extern crate lazy_static;

mod site;
mod theme;
mod template;

use std::path::Path;

/// Creates a new project with necessary file structure and config files
pub fn gen_new_site(project_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
  let _site = site::Website::new(project_dir)?;
  let theme = theme::Theme::default()?;

  theme.save_to_disk(project_dir)?;

  Ok(())
}

/// Generates the current website
pub fn gen_site() -> Result<(), Box<dyn std::error::Error>> {
  // ensure we are in a project directory
  let project_dir = std::env::current_dir()?;
  if !project_dir.join("config.toml").exists() {
    Err("Invalid project directory.")?
  }

  let site = site::Website::from_project(&project_dir)?;
  site.generate(&project_dir)?;
  
  Ok(())
}

/*fn create_content_file(project_dir: &Path) {
  let mut content_file = File::create(project_dir.join("content/"))
}*/