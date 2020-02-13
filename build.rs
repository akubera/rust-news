
// extern crate maud;
// extern crate maud;

// use maud;

// // use std::fs::{File, ReadDir};
// use std::io::prelude::*;

// use maud;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use yaml_rust::yaml;
use failure::Error;

// struct Error;


fn main() -> Result<(), Error>
{
  build_slides()?;

  Ok(())
}

fn build_slides() -> Result<(), Error>
{
  let out_dir = env::var("OUT_DIR").unwrap();

  let foobar = SlideShow::from_yaml_path("data/2016-12-08.yaml");

  // let dest_path = Path::new(&out_dir).join("hello.rs");
  // let mut f = File::create(&dest_path).unwrap();

  // f.write_all(b"
  //     pub fn message() -> &'static str {
  //         \"Hello, World!\"
  //     }
  // ").unwrap();
  Ok(())
}

struct SlideShow {
  title: String,
  author: String,
  date: String,

  slides: Vec<Slide>
}

impl SlideShow {

  fn from_yaml_path<P: AsRef<Path>>(yaml_path: P) -> Option<SlideShow>
  {
    let mut file = File::open(yaml_path).ok()?;

    let mut yaml_src = String::new();
    file.read_to_string(&mut yaml_src).ok()?;

    let docs = yaml::YamlLoader::load_from_str(&yaml_src).ok()?;
    let doc = &docs[0];

    let title = doc["title"].as_str().unwrap_or("");

    Some(SlideShow {
      title: title.into()

    })
  }

  fn to_html(&self) -> String
  {
    use horrorshow::prelude::*;
    use horrorshow::html;
    use horrorshow::helper::doctype;

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


struct Slide {
  title: Option<String>,
  bullets: Vec<Bullet>,
}

struct Bullet {
  text: String,
  bullets: Vec<Bullet>,
}


impl Slide {
  fn from_yaml() -> Vec<Slide>
  {
    vec![]
  }
}
