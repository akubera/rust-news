//! [\file]:# (build.rs)
//!
//! Build Script
//!

// #![feature(io)]

extern crate yaml_rust;


fn main()
{
  generate_slides();
}

use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};

extern crate yassyl;
// use yassyl;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;


fn yaml_files_in(dirname: &str) -> Vec<String>
{
  std::fs::read_dir(dirname)
    .unwrap()
    .map(|entry| entry.unwrap().path().into_os_string().into_string())
    .filter(|s| s.is_ok())
    .map(|s| s.unwrap())
    .filter(|s| s.ends_with(".yaml"))
    .collect()
}

fn generate_slides()
{

  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("slides.rs");
  let mut f = File::create(&dest_path).unwrap();

  for yaml_filename in yaml_files_in("data") {
    println!("rerun-if-changed={}", yaml_filename);

    let mut s = String::new();
    File::open(&yaml_filename).unwrap().read_to_string(&mut s).unwrap();

    let docs = YamlLoader::load_from_str(&s).unwrap();
    let doc = &docs[0];

    let slides: yassyl::Slideshow = doc.into();
  }

}
