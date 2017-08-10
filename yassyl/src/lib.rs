//! [\file]:# (yash/src/lib.rs)
//!
//! Yet Another Slide Show YAML Library
//!
//! Build simple presentation slides from yaml documents
//!

extern crate yaml_rust;
use yaml_rust::Yaml;


use std::convert;

/// Slideshow object
pub struct Slideshow {
  pub title: String,
  pub author: String,
  pub date: String,
}


impl Slideshow {
  fn get_url(&self) -> String
  {
    format!("/{}.html", self.date)
  }
}

impl<'a> convert::From<&'a Yaml> for Slideshow {
  fn from(yaml: &'a Yaml) -> Self
  {
    let title = yaml["title"].as_str().unwrap().into();
    let author = yaml["author"].as_str().unwrap().into();
    let date = yaml["date"].as_str().unwrap().into();

    Slideshow {
      author: author,
      title: title,
      date: date,
    }
  }
}
