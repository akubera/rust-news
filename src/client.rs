

use stdweb::{self, console};

// mod hello;
// use hello::message;

// include!(concat!(env!("OUT_DIR"), "/hello.rs"));



fn setup_interface()
{
  use horrorshow::{
    prelude::*,
    html,
  };
  use stdweb::web::IElement;

  // let h1 = html! {
  //   h1 { this is some text }
  // }.into_string().unwrap();

  let h1 = "<h1> Text me maybe </h1>";

  let body = stdweb::web::document().body().unwrap();
  body.append_html(&h1);
}


pub fn runstuff()
{
  setup_interface();

  let message = "Hello, 世界!";
  console!(log, message);
  // js! {
  //     alert( @{message} );
  // }
}
