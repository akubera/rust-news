
pub use yaml_rust::yaml;

use serde::{Serialize, Deserialize};
use std::{
    path::Path,
    fs::File,
    io::prelude::*,
};


#[derive(Serialize, Deserialize, Debug,)]
pub struct SlideShow {
  pub title: String,
  pub author: String,
  pub date: String,

  pub slides: Vec<String>,
}

impl SlideShow {

  pub fn from_yaml_path<P: AsRef<Path>>(yaml_path: P) -> Option<SlideShow>
  {
    let mut file = File::open(yaml_path).ok()?;

    let mut yaml_src = String::new();
    file.read_to_string(&mut yaml_src).ok()?;

    let docs = yaml::YamlLoader::load_from_str(&yaml_src).ok()?;
    let doc = &docs[0];

    let title = doc["title"].as_str().unwrap_or("");
    let author = doc["author"].as_str().unwrap_or("");
    let date = doc["date"].as_str().unwrap_or("");

    Some(SlideShow {
      title: title.into(),
      author: author.into(),
      date: date.into(),
      slides: vec![],
    })
  }

  pub fn to_html(&self) -> String
  {
    use horrorshow::{
        prelude::*,
          html,
          helper::doctype};

    let template = html! {
      : doctype::HTML;
      html {
        head {
          title: "RUST NEWS"
        }
        body {
          h1: "Rust News"
        }
      }
    };

    template.into_string().unwrap()
  }
}


pub struct Slide {
  pub title: Option<String>,
  pub bullets: Vec<Bullet>,
}


impl Slide {
  pub fn from_yaml() -> Vec<Slide>
  {
    vec![]
  }
}

/// An individual bullet, with text and maybe sub-bullets
pub struct Bullet {
  pub text: String,
  pub bullets: Vec<Bullet>,
}
