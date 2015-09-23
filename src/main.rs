
#[macro_use]
extern crate horrorshow;
extern crate iron;
#[macro_use]
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate logger;

use mount::Mount;
use router::Router;

use iron::prelude::*;
use iron::mime::Mime;
use iron::status;

use iron::middleware::Handler;

use staticfile::Static;

mod public_html;

use public_html::index::make_html;

use std::fs;

use std::path::Path;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use logger::Logger;

fn main()
{
    // let first = dir.next().unwrap().unwrap();
    // println!("{:?}", first.path());
  //
  //   let data_dir = Path::new("./data");
  //
  //   for entry in fs::read_dir(data_dir).unwrap() {
  //       let filename = entry.unwrap();
  //       parse_slide_file(filename);
  //   }
  //
  // let get_root = |_: &mut Request| {
  //   let content_type = "text/html".parse::<Mime>().unwrap();
  //   Ok(Response::with((status::Ok, content_type, make_html())))
  // };

  let mut mount = Mount::new();
  mount.mount("/", Static::new(Path::new("client")));

  let mut router = Router::new();
  // router.get("/", get_root);

  // router.get("/", mount);
  router.get("/", |req: &mut Request| {
    Ok(Response::with((status::Ok, "Hello World!")))
  });


  // chain.link_before(mount);
  // chain.link(router);


  let host_str = "localhost:3000";
  println!("Running on http://{}", host_str);

  // Iron::new(router).http(host_str).unwrap();
  let mut chain = Chain::new(router);
  chain.link(mount);
  chain.link(Logger::new(None));
  Iron::new(chain).http(host_str).unwrap();

}


extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};
use std::collections::BTreeMap;

fn parse_slide_file(path : fs::DirEntry)
{
  let mut f = fs::File::open(path.path()).unwrap();
  let mut s = String::new();
  f.read_to_string(&mut s);
  // let docs = YamlLoader::load_from_string(s).unwrap();
  let docs = YamlLoader::load_from_str(&s).unwrap();
  let doc = &docs[0];

  let data = doc.as_hash().unwrap();

  let date_key = Yaml::String("date".to_string());

  let date = data.get(&date_key).unwrap();

  println!("{:?} {}", path.path(), date.as_str().unwrap());
  // println!("  {:?}", date);

}
