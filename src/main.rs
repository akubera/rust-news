//! [file]: # (src/lib.rs)
//! [author]: # (Andrew Kubera <andrewkubera@gmail.com>)
//!
//! Main file for the news server
//!

#![feature(proc_macro)]

extern crate rouille;
extern crate maud;
use rouille::{Request, Response};
use maud::html;


fn main()
{
  rouille::start_server("0.0.0.0:9090", handle_stuff);
}

fn handle_stuff(request: &Request) -> Response
{
  if request.url() == "/" {
    index_page()
  } else if let Some(request) = request.remove_prefix("/static") {
    rouille::match_assets(&request, "/public")
  } else {
    Response::html(html! { head { title { "Not Found" } }
                             body { h1 {"404 - Not Found"} }})
    .with_status_code(404)
  }
}

fn index_page() -> Response
{
  let name = "Friend";

  Response::html(html! {
      html {
          head {
            title { "hi" }
          }
          body {
            p { "Hello, " (name) "!" }}
        }
      })
}
