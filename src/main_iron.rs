

use iron::prelude::*;
use iron::Handler;
use iron::{self, headers, BeforeMiddleware, AfterMiddleware, typemap, response};

use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use time::precise_time_ns;
use staticfile::Static;
use router::Router;
use mount::Mount;

mod html_index {
  include!(concat!(env!("OUT_DIR"), "/slideshow-index.rs"));
}


fn index(_: &mut Request) -> IronResult<Response>
{
  let mut resp = Response::with((iron::status::Ok, html_index::INDEX_HTML));
  resp.headers.set(ContentType(
    Mime(TopLevel::Text, SubLevel::Html, vec![]),
  ));
  Ok(resp)
}

fn news_slide(_: &mut Request) -> IronResult<Response>
{

  // let HTML_DATA = include!(concat!(env!("OUT_DIR"), "/2017-08.rs"));
  // let HTML_DATA = include_str!(concat!(env!("OUT_DIR"), "/2017-08.rs"));
  // let HTML_DATA = "foo";
  let mut resp = Response::with((iron::status::Ok, HTML_DATA));
  resp.headers.set(ContentType(
    Mime(TopLevel::Text, SubLevel::Html, vec![]),
  ));
  Ok(resp)
}



pub fn serve()
{
  let static_files = Static::new("client");

  let mut paths = Mount::new();
  paths.mount("/static", static_files);
  paths.mount("/news", news_slide);
  paths.mount("/", index);


  // paths.mount("/", index);

  let mut chain = Chain::new(paths);

  chain.link((ResponseTime, ResponseTime));


  let addr = ("localhost", 2000);
  println!("listening on {}:{}", addr.0, addr.1);
  Iron::new(chain).http(addr).unwrap();
}

struct ResponseTime;

impl typemap::Key for ResponseTime {
  type Value = u64;
}

impl BeforeMiddleware for ResponseTime {
  fn before(&self, req: &mut Request) -> IronResult<()>
  {
    req.extensions.insert::<ResponseTime>(precise_time_ns());
    Ok(())
  }
}

impl AfterMiddleware for ResponseTime {
  fn after(&self, req: &mut Request, res: Response) -> IronResult<Response>
  {
    let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
    println!("Request took: {} ms", (delta as f64) / 1000000.0);
    Ok(res)
  }
}
