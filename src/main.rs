// use std::fs::File;
// use std::io::{ Read, Write };
use std::path::PathBuf;
use clap::Clap;

// use minmarkdown;

// mingen new <content_path> - creates a new content file in contents/<content_path>
// mingen new site <sitename> - creates a new site
// mingen server - launches server that shows live updates

#[derive(Clap, Debug)]
enum Args {
  /// generate new mingen content
  New {
    #[clap(short, long)]
    site: bool,

    /// name of the content
    #[clap(parse(from_os_str))]
    name: PathBuf
  },
  Gen
}

fn main() {
  let args = Args::parse();
  match args {
    Args::New {
      site, name
    } => {
      if site {
        // generate a new site project
        match mingen::gen_new_site(&name) {
          Ok(_) => println!("New site created!"),
          Err(e) => {
            println!("Application error: {}", e);
            std::process::exit(1);
          }
        }
      } else {
        // generate new content
      }
    }, 
    Args::Gen => {
      match mingen::gen_site() {
        _ => ()
      }
    }
  }

  // reading test input file
  /*let mut file = File::open("input-long.md").expect("Unable to open file.");
  let mut input = String::new();
  file.read_to_string(&mut input).expect("Unable to read input file.");
  let input = input; // make immutable

  let output = minmarkdown::to_html(&input);
  let mut output_file = File::create("output.html").expect("Unable to create output file.");
  output_file.write(output.as_bytes()).expect("Unable to write to output file.");

  println!("done");*/
}