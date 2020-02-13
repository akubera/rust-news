//
// build.rs
//

use std::{env, fs, path::Path, fs::File, io::Write};

use slideby::{SlideShow, Slide};
use failure::Error;


fn main() -> Result<(), Error>
{
  build_slides()?;

  Ok(())
}

fn build_slides() -> Result<(), Error>
{
  let out_dir = env::var("OUT_DIR").unwrap();

  let mut slideshows = vec![];

  for entry in fs::read_dir("data").unwrap() {
    let next_file = entry.unwrap().path();
    let filename: String = next_file.as_os_str().to_str().unwrap().into();
    println!("cargo:rerun-if-changed={}", filename);

    if filename.ends_with(".yaml") {

      let slideshow = SlideShow::from_yaml_path(next_file);

      slideshows.push((filename, slideshow));
    }
  }

  let dest_path = Path::new(&out_dir).join("precompiled_slides.rs");
  let mut f = File::create(&dest_path).unwrap();

  for (filename, slideshow) in slideshows {
    // f.write
  }

  // f.write_all(b"
  //     pub fn message() -> &'static str {
  //         \"Hello, World!\"
  //     }
  // ").unwrap();
  Ok(())
}
