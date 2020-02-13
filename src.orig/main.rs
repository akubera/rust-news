//! [file]: # (src/lib.rs)
//! [author]: # (Andrew Kubera <andrewkubera@gmail.com>)
//!
//! Main file for the news server
//!

#![feature(plugin)]
extern crate pencil;
extern crate webbrowser;

#[allow(plugin_as_library)]
extern crate news;


/// Entry point for rust news application
///
fn main()
{
  let mut app = pencil::Pencil::new(".");
  app.static_folder = "client".into();
  news::add_news_routes(&mut app);
  app.enable_static_file_handling();
  std::thread::spawn(|| {
    let _ = webbrowser::open("http://127.0.0.1:5000");
  });
  app.run("127.0.0.1:5000");
}
