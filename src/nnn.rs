

use stdweb::{self, js, console};

// mod hello;
// use hello::message;

// include!(concat!(env!("OUT_DIR"), "/hello.rs"));



pub fn runstuff()
{
  let message = "Hello, 世界!";
  console!(log, message);
  // js! {
  //     alert( @{message} );
  // }
}
