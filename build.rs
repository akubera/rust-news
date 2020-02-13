//
// build.rs
//

use std::{env, fs, path::Path, fs::File, io::Write};

use slideby::SlideShow;
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
      if let Some(slideshow) = SlideShow::from_yaml_path(next_file) {
        slideshows.push((filename, slideshow));
      } else {
        let err = failure::err_msg(format!("Reading yaml file {:?} failed", filename));
        return Err(err);
      }
    }
  }

  let dest_path = Path::new(&out_dir).join("precompiled_slides.rs");
  let mut f = File::create(&dest_path).unwrap();

  writeln!(&mut f, "pub fn load_slideshows() -> Vec<SlideShow> {{").unwrap();
  writeln!(&mut f, "  vec![").unwrap();
  for (_filename, slideshow) in slideshows {
    // writeln!(&mut f, "   rmp_serde::from_slice(&{:?}).unwrap(),", rmp_serde::to_vec(&slideshow).unwrap()).unwrap();
    writeln!(&mut f, "   serde_json::from_str({:?}).unwrap(),", serde_json::to_string(&slideshow).unwrap()).unwrap();
  }
  writeln!(&mut f, "]}} ").unwrap();

  Ok(())
}
