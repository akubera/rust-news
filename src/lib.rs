//! [file]: # (src/lib.rs)
//!
//! Main crate holding code to host the server.
//!

#![feature(plugin)]
#![plugin(walkdir)]

#[macro_use]
extern crate horrorshow;

extern crate pencil;

use pencil::{Pencil, Request, Response, PencilResult};

mod html;

use horrorshow::prelude::*;

pub fn add_news_routes(app: &mut Pencil)
{
  // let foo = yaml_files_in_dir!("data");


  fn index(_: &mut Request) -> PencilResult
  {
    // let data = foo[0];
    let data = yaml_files_in_dir!("data");
    let hypertext = box_html! {
    html {
      head {
        : raw!(html::HEAD())
      }
      body {
        div {
          ul {
            @ for x in &data {
              li {
                a(href=["/slides/", &x[5..]].join("")) { : &x[5..15] }
              }
            }
          }
        }
        hr {}
      }
    }
  };
    // println!(">>> {}", hypertext);
    return Ok(Response::from(hypertext.into_string().unwrap()));
  }

  /// Return html string of revealjs slide
  fn slides(req: &mut Request) -> PencilResult
  {
    let slide_name = req.view_args.get("slide_name").unwrap();

    let yaml_files = yaml_files_in_dir!("data");
    let yaml_data = yaml_files_to_html_vec!("data");
    // let yaml_res
    for (filename, data) in yaml_files.iter().zip(yaml_data.iter()) {
      if filename.ends_with(slide_name) {
        return Ok(Response::from(*data));
      }
      // let url = ["/", filename].join("");
      // app.get(url, "slide")
    }
    return Ok(Response::from("Not Found"));
  }

  app.get("/", "index", index);
  app.get("/slides/<slide_name:string>", "slides", slides);
}
