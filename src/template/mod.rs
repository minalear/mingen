use crate::site::{ Post, Website };
use regex::Regex;

#[derive(Debug)]
struct _Loop {
  this: Post,
  loop_body: String,
}

pub fn parse(post: &Post, _site: &Website, _template: &str) {
  lazy_static! {
    static ref LOOP_BEG_RE: Regex = Regex::new(r"{{.?#each (.+)?.?}}").unwrap();
    static ref LOOP_END_RE: Regex = Regex::new(r"{{.?\/each.?}}").unwrap();
  }

  let mut _loops: Vec<_Loop>;
  let mut _buffer = String::new();
  for line in post.content.lines() {
    match LOOP_BEG_RE.captures(line) {
      Some(cap) => {
        println!("{:?}", cap);
      },
      None => { }
    }
  }
}