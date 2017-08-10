//! [file]: # (src/lib.rs)
//! [author]: # (Andrew Kubera <andrewkubera@gmail.com>)
//!
//! Main file for the news server
//!

// #![feature(pub_restricted)]
// #![feature(plugin)]
// extern crate pencil;
// extern crate webbrowser;

// #[allow(plugin_as_library)]
// extern crate news;

extern crate time;

// extern crate pencil;
// mod main_pencil;
// use main_pencil::serve;

extern crate iron;
extern crate hyper;
extern crate staticfile;
extern crate router;
extern crate mount;
mod main_iron;
use main_iron::serve;

/// Entry point for rust news application
///
fn main()
{
  serve();
}
//   main_iron();
//   // main_pencil();
// }
