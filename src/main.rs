
#[macro_use]
extern crate horrorshow;
extern crate iron;
extern crate router;

use router::Router;

use iron::prelude::*;
use iron::mime::Mime;
use iron::status;

mod public_html;

use public_html::index::make_html;

fn main()
{
  let get_root = |_: &mut Request| {
    let content_type = "text/html".parse::<Mime>().unwrap();
    Ok(Response::with((status::Ok, content_type, make_html())))
  };

  let mut router = Router::new();

  router.get("/", get_root);

  Iron::new(router).http("localhost:3000").unwrap();

}



// fn main()
// {
// let actual = make_html();
// let expected = "\
// <html>\
//   <head>\
//     <title>Hello world!</title>\
//   </head>\
//   <body>\
//     <h1 id=\"heading\">Hello! This is &lt;html /&gt;</h1>\
//     <p>Let's <i>count</i> to 10!</p>\
//     <ol id=\"count\">\
//       <li first>1</li>\
//       <li>2</li>\
//       <li>3</li>\
//       <li>4</li>\
//       <li>5</li>\
//       <li>6</li><li>7</li>\
//       <li>8</li>\
//       <li>9</li>\
//       <li>10</li>\
//     </ol>\
//     <br /><br />\
//     <p>Easy!</p>\
//   </body>\
// </html>";
// assert_eq!(expected, actual);
// }
