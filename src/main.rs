//
// \file main.rs
// \brief Main file for the news server
//
#![feature(plugin)]
#![plugin(news)]

extern crate pencil;
extern crate webbrowser;

use std::thread;
use pencil::{Pencil, Request, Response, PencilResult};


mod news_server;

use news_server::HTTPServer;


fn hello(_: &mut Request) -> PencilResult
{
    Ok(Response::from(yaml_to_html!("data/2016-05-12.yaml")))
    // Ok(Response::from("Hello World!"))
}


/// Entry point for rust news application
///
fn main()
{
  // let server = HTTPServer { port : 3000 };
  // println!("listening on port: {}", server.port);

  let mut app = Pencil::new(".");
  app.static_folder = "reveal.js".into();
  app.get("/", "hello", hello);
  app.enable_static_file_handling();
  thread::spawn(|| {
      webbrowser::open("http://127.0.0.1:5000");
  });
  app.run("127.0.0.1:5000");
}
