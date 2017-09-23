//! [file]: # (src/lib.rs)
//! [author]: # (Andrew Kubera <andrewkubera@gmail.com>)
//!
//! Main file for the news server
//!
#![feature(proc_macro)]  // <- IMPORTANT don't forget this!

extern crate rouille;
extern crate maud;
use rouille::{Request, Response};
use maud::html;


fn main()
{
  rouille::start_server("0.0.0.0:9090", move |_request| {
    let name = "Friend";
    let markup = html! {
      html {
        head {
          title { "hi" }
        }
        body {
          p { "Hello, " (name) "!" }}
      }
    };
    Response::html(markup)
  });
}
