
use stdweb;
// use crate::news;
// extern crate news;
mod client;


fn main()
{
  stdweb::initialize();
  stdweb::document().body().unwrap().append_html(&html)

  client::runstuff();

  stdweb::event_loop();
}
