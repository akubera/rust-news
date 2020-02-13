
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

use slideby::{yaml, SlideShow, Slide};
use failure::Error;


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
