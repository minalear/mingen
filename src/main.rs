use std::fs::File;
use std::io::{ Read, Write };
use minmarkdown;

fn main() {
  // reading test input file
  let mut file = File::open("input-short.md").expect("Unable to open file.");
  let mut input = String::new();
  file.read_to_string(&mut input).expect("Unable to read input file.");
  let input = input; // make immutable

  let output = minmarkdown::to_html(&input);
  let mut output_file = File::create("output.html").expect("Unable to create output file.");
  output_file.write(output.as_bytes()).expect("Unable to write to output file.");

  println!("done");
}