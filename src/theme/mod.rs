use std::fs;
use std::path::Path;
use std::io::Write;

pub struct Theme {
  name: String,
  css: String
}

impl Theme {
  pub fn default() -> Result<Theme, Box<dyn std::error::Error>> {
    Ok(Theme { 
      name: String::from("default"),
      css: String::from("body { font-size: 11px; }")
    })
  }

  pub fn save_to_disk(&self, project_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let theme_dir = project_dir.join("themes/").join(&self.name);
    let directories = [
      theme_dir.to_path_buf(),
      theme_dir.join("static"),
      theme_dir.join("styles"),
      theme_dir.join("images"),
      theme_dir.join("partials")
    ];

    for dir in directories.iter() {
      fs::create_dir(dir)?;
    }

    let mut css = fs::File::create(theme_dir.join("styles/theme.css"))?;
    css.write(self.css.as_bytes())?;

    Ok(())
  }
}