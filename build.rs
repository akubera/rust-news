//! [\file]:# (build.rs)
//!
//! Build Script
//!
#![feature(rustc_private)]
// #![feature(io)]
#![feature(plugin)]
#![plugin(maud_macros)]


extern crate yaml_rust;
extern crate aster;

extern crate maud;
use maud::DOCTYPE;

const foo: &'static str = "hello.";

fn main()
{
  generate_slides();
}

use std::io::Read;

use yaml_rust::{Yaml, YamlLoader};

extern crate slideby_html;
use slideby_html::Slideshow;

use std::env;
use std::fs::{self, File};
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
extern crate syntax;
use syntax::print::pprust;
fn generate_slides()
{
  let OUT_DIR_ENV: String = env::var("OUT_DIR").unwrap();

  let out_dir = Path::new(&OUT_DIR_ENV);
  let dest_path = out_dir.join("slides.rs");
  let mut f = File::create(&dest_path).unwrap();

  let builder = aster::AstBuilder::new();

  let slideshow_index_path = out_dir.join("slideshow-index.rs");

  let mut slideshow_index_file = File::create(&slideshow_index_path).expect("Could not create slideshow-index page");
  // let slide_expr = builder.expr().lit().str("Foobar");
  // let stuff = builder.stmt().let_id("foo").expr().assign().lit().str("Foo");
  // let stuff = builder.item().pub_().const_("haia").expr().lit().str("foo");

  // let mut slide_files = Vec::new();

  let mut slide_links = Vec::new();
  // slide_links.push(("/link", "Foobar"));

  for yaml_filename in yaml_files_in("data") {
    println!("rerun-if-changed={}", yaml_filename);
    let source_path = Path::new(&yaml_filename);
    // let target_file = format!("{}.rs", yaml_filename); // : String::from_str(yaml_filename) + ".rs";
    let target_filename = Path::new(&source_path.file_name().unwrap()).with_extension("rs");
    let target_path = out_dir.join(&target_filename);

    // slide_files.push(target_filename);
    // println!("# {} -> {}", yaml_filename, target_path.to_str().unwrap_or(""));

    // if !should_skip_file_creation(&yaml_filename, &target_path) {
    //   continue;
    // }

    // let ex = builder.expr().lit().str("H\\\"xey");
    // .build_add(builder.expr().u16(1), builder.expr().add().some().u8(3).u8(9));
    //       // let f = builder.expr().
    //       // let ex = builder.expr().neg().add().u32(4).u8(1);

    //       //add_assign().
    //       //.add().sub().u32(3).u32(1).u32(4); // .u32(8);
    //                              //(+ (- 3 1) 4)

    // println!(">> {}\n", pprust::expr_to_string(&ex));

    //     let assign = builder.stmt().let_().id("oo").build_expr(ex); // .neg().sub().u32(12).u8(1);
    //     // println!(">> {}\n", pprust::stmt_to_string(&assign));

    // load yaml data into string
    let mut s = String::new();
    File::open(&yaml_filename)
      .unwrap()
      .read_to_string(&mut s)
      .unwrap();

    let url: String = source_path.file_stem().unwrap().to_str().unwrap().into();
    let url: String = "/news".into();

    let docs = YamlLoader::load_from_str(&s).unwrap();

    // get first (only?) document
    let doc = &docs[0];

    // build slideshow from document
    let slideshow = Slideshow::from(doc);

    // turn into a string
    let slideshow_html_string = slideshow.to_revealjs_html_document();

    // add to module
    let slide_expr = builder.expr().lit().str(slideshow_html_string.as_str());

    // let slideshow_string = pprust::expr_to_string(&slide_expr);

    // let slide_html = builder.item().pub_().const_("HTML_DATA").with_expr(slide_expr).ty().id("&str");
    let slide_html = builder.stmt().let_().id("HTML_DATA").build_expr(slide_expr);


    // build.stmt().

    File::create(&target_path)
      .unwrap()
    //   .write_all(&pprust::stmt_to_string(&slide_html).as_bytes())
    //     File::create(&target_path)
    //       .unwrap()
    //       // .write_all(b"it works...").unwrap();
          .write_fmt(
            format_args!("
    r##\"{};
    pub fn get_slides() -> &'static str
    {{ return slides; }}\"##;", slideshow_html_string)
          ).unwrap();

    slide_links.push((url, format!("{} - {}", slideshow.date, slideshow.title)));

  }
  // let slide-links =
  // builder.mac().
  // builder.block().with_stmt(builder.))
  //   File::create(dest_path).unwrap().write_fmt(format_args!("
  // "))
  slide_links.reverse();
  let index_src = html! {
      (DOCTYPE) html {
      head {
          style " body { text-align: center;} a {color : #4444BB; } a:hover { color: lightgrey; } ul { list-style-type: None; margin: 0; padding: 0;} li { font-size: 16pt; margin-top: 14px; }"
      }
      body {
          h1 "Rusty News!"
          hr
          ul {
              @for (ref url, ref title) in slide_links  {
                  li { a href=(url) (title) }
              }
          }
      }
    }
  }.into_string();
  let stuff = builder.item()
                     .pub_()
                     .const_("INDEX_HTML")
                     .expr()
                     .lit()
                     .str(&index_src[..])
                     .ty()
                     .id("&str");

  //  let stuff = builder.expr()
  //         .add().u32(1).u32(2);
  let index = pprust::item_to_string(&stuff);
  slideshow_index_file.write_all(&index.into_bytes());


}


fn should_skip_file_creation(source_filename: &AsRef<Path>, target_filename: &AsRef<Path>) -> bool
{
  match (fs::metadata(source_filename), fs::metadata(target_filename)) {
    (Err(_), Err(_)) | (Ok(_), Err(_)) | (Err(_), Ok(_)) => false,
    (Ok(source_meta), Ok(target_meta)) => {
      match (source_meta.modified(), target_meta.modified()) {
        (Err(_), Err(_)) | (Ok(_), Err(_)) | (Err(_), Ok(_)) => false,
        // should skip if source is older than target
        (Ok(source_time), Ok(target_time)) => (source_time > target_time),
      }
    },
  }
}
