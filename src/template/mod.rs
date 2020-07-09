use crate::site::{ Post, Website };
use regex::{ Regex, Captures};

#[derive(Debug)]
struct Loop {
  header: String,
  body: String,
}

pub fn parse(post: &Post, site: &Website, template: &str) -> Result<String, Box<dyn std::error::Error>> {
  lazy_static! {
    static ref LOOP_BEG_RE: Regex = Regex::new(r"\{\{.?#each (.+)?.?\}\}").unwrap();
    static ref LOOP_END_RE: Regex = Regex::new(r"\{\{.?/each.?\}\}").unwrap();
  }

  // first unroll all loops
  let mut first_pass = String::new();
  let mut _loop = Loop {
    header: String::new(),
    body: String::new()
  };
  for line in template.lines() {
    // check for matching first loop
    match LOOP_BEG_RE.captures(line) {
      Some(cap) => {
        // if we're already in a loop, error out (no nested loops yet)
        if _loop.header.len() > 0 {
          return Err("Nested loops are not supported yet in templates.")?;
        }

        _loop.header.push_str(&cap[1]);
        continue;
      },
      None => { }
    }
    match LOOP_END_RE.captures(line) {
      Some(_) => {
        // ensure we have a header before we process
        if _loop.header.len() == 0 {
          return Err("No opening loop tag for closing loop tag.")?;
        }

        // unroll the loop into our template
        first_pass.push_str(&_loop.execute(&site)?);
        first_pass.push_str("\n");

        // clear _loop
        _loop.header.clear();
        _loop.body.clear();
        continue;
      },
      None => { }
    }

    if _loop.header.len() > 0 {
      // construct the loop body
      _loop.body.push_str(line);
    } else {
      // push formatted text to the first_pass document
      // first_pass.push_str(line);
      first_pass.push_str(&format_line(&line, &post, &site)?);
      first_pass.push_str("\n");
    }
  }

  Ok(first_pass)
}

impl Loop {
  fn execute(&self, site: &Website) -> Result<String, Box<dyn std::error::Error>> {
    if self.header.len() == 0 {
      Err("Invalid loop header.")?
    }

    // split header based on whitespace
    let tokens: Vec<&str> = self.header.split(' ').collect();
    // println!("tokens: {:#?}", tokens);
    
    // figure out the number of posts to loop through
    let count: usize = 
      if tokens[0] == "site.posts" {
        match tokens.len() {
          1 => site.posts.len(),
          3 => {
            // ensure we return the smaller of the two; lim or post size
            let lim = tokens[2].parse::<usize>().unwrap(); 
            if lim > site.posts.len() { site.posts.len() } else { lim }
          },
          _ => return Err("Invalid number of loop header elements.")?
        }
      } else {
        return Err("We only support looping through posts right now.")?;
      };

    // unroll loop
    let mut html = String::new();
    for i in 0..count {
      html.push_str(&format_line(&self.body, &site.posts[i], &site)?);
      html.push_str("\n");
    }

    Ok(html)
  }
}

fn format_line(line: &str, post: &Post, site: &Website) -> Result<String, Box<dyn std::error::Error>> {
  lazy_static! {
    // site data
    static ref SITE_TITLE:   Regex = Regex::new(r"\{\{.?site.title?.?\}\}").unwrap();
    static ref SITE_THEME:   Regex = Regex::new(r"\{\{.?site.theme?.?\}\}").unwrap();
    static ref SITE_AUTHOR:  Regex = Regex::new(r"\{\{.?site.author?.?\}\}").unwrap();

    static ref POST_TITLE:   Regex = Regex::new(r"\{\{.?this.title?.?\}\}").unwrap();
    static ref POST_SLUG:    Regex = Regex::new(r"\{\{.?this.slug?.?\}\}").unwrap();
    static ref POST_AUTHOR:  Regex = Regex::new(r"\{\{.?this.author?.?\}\}").unwrap();
    static ref POST_PUBDATE: Regex = Regex::new(r"\{\{.?this.pubdate?.?\}\}").unwrap();
    static ref POST_DRAFT:   Regex = Regex::new(r"\{\{.?this.draft?.?\}\}").unwrap();
    static ref POST_CONTENT: Regex = Regex::new(r"\{\{.?this.content?.?\}\}").unwrap();
  }

  let format = SITE_TITLE.replace_all(line, |_: &Captures| { &site.name });
  let format = SITE_THEME.replace_all(&format, |_: &Captures| { &site.theme });
  let format = SITE_AUTHOR.replace_all(&format, |_: &Captures| { &site.author });

  let format = POST_TITLE.replace_all(&format, |_: &Captures| { &post.title });
  let format = POST_SLUG.replace_all(&format, |_: &Captures| { &post.slug });
  let format = POST_AUTHOR.replace_all(&format, |_: &Captures| { &post.author });
  let format = POST_PUBDATE.replace_all(&format, |_: &Captures| { &post.pubdate });
  let format = POST_DRAFT.replace_all(&format, |_: &Captures| { 
    if post.draft { "true" } else { "false" }
  });
  let format = POST_CONTENT.replace_all(&format, |_: &Captures| { &post.content });

  let format = String::from(format);

  Ok(format)
}