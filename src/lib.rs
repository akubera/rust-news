//! [file]: # (src/lib.rs)
//!
//! Main crate holding code to host the server.
//!

#![feature(plugin)]
#![plugin(walkdir)]

extern crate pencil;

use pencil::{Pencil, Request, Response, PencilResult};

fn index(_: &mut Request) -> PencilResult
{
  let text = list_dir!(data);
  return Ok(Response::from(text[0]));
}


pub fn add_news_routes(app: &mut Pencil)
{
  app.get("/", "index", index);
}
