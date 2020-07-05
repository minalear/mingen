#[macro_use] extern crate lazy_static;

use std::fs::File;
use std::io::{ Read, Write };
use regex::Regex;

fn main() {
  // reading test input file
  let mut file = File::open("input-short.md").expect("Unable to open file.");
  let mut input = String::new();
  file.read_to_string(&mut input).expect("Unable to read input file.");
  let input = input; // make immutable

  let mut output = String::new();
  for line in input.lines() {
    match line.len() {
      0 => continue,
      _ => {
        output.push_str(&format_line(line));
        output.push_str("\n");
      }
    }
  }

  println!("{}", output);
}

fn format_line(line: &str) -> std::borrow::Cow<str> {
  // only compile the regex once
  lazy_static! {
    static ref RE: Regex = Regex::new(r"\*\*(.*?)*\*\*").unwrap();
  }

  let line = RE.replace_all(line.trim(), "<strong>$1</strong>");


  /*if line.contains("# ") {
    return format!("<h1>{}</h1>\n", line.replace("# ", ""));
  } else if line.contains("## ") {
    return format!("<h2>{}</h2>\n", line.replace("## ", ""));
  } else if line.contains("### ") {
    return format!("<h3>{}</h3>\n", line.replace("### ", ""));
  }

  format!("<p>{}</p>\n", line)*/

  line
}