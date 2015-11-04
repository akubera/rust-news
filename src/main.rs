///
/// \file main.rs
/// \brief Main file for the news server
///

mod news_server;

use news_server::HTTPServer;

/// Entry point for rust news application
///
fn main()
{
  let server = HTTPServer { port : 3000 };
  println!("listening on port: {}", server.port);
}
