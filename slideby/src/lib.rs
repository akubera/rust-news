
pub use yaml_rust::yaml::{self, Yaml};

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

  pub slides: Vec<Slide>,
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

    let slide_srcs = doc["slides"].as_vec().unwrap();

    Some(SlideShow {
      title: title.into(),
      author: author.into(),
      date: date.into(),
      slides: slide_srcs.iter().map(Slide::from_yaml).collect(),
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


#[derive(Serialize, Deserialize, Debug)]
pub struct Slide {
  pub title: Option<String>,
  pub bullets: Vec<Bullet>,
}

impl Slide {
  pub fn from_yaml(y: &yaml::Yaml) -> Slide
  {
    let title_key = Yaml::from_str("title");
    let bullets_key = Yaml::from_str("bullets");

    let slide_data = y.as_hash().unwrap();

    let title = match slide_data.get(&title_key) {
      Some(&Yaml::String(ref s)) => Some(String::from(s.as_str())),
      _ => None,
    };

    let bullets = match slide_data.get(&bullets_key) {
      Some(&Yaml::Array(ref a)) => a.iter().map(Bullet::read_from_yaml).collect(),
      // Bullet::read_vec_from_yaml(a),
      Some(&Yaml::Null) => vec![],
      Some(_) => panic!("bullets not an array"),
      None => vec![],
    };

    Slide {
      title, bullets
    }
  }
}

/// An individual bullet, with text and maybe sub-bullets
#[derive(Serialize, Deserialize, Debug)]
pub enum Bullet {
  Text(String),
  SubBullets(Vec<Bullet>),
}

impl Bullet {
  // fn read_vec_from_yaml(y: &Yaml) -> Vec<Bullet>
  fn read_from_yaml(y: &Yaml) -> Bullet
  {
    match y {
      &Yaml::String(ref s) => Bullet::Text(String::from(s)),
      &Yaml::Array(ref a) => Bullet::SubBullets(a.iter().map(Bullet::read_from_yaml).collect()),
      _ => {
        panic!("Invalid bullet");
      }
    }
  }
}
