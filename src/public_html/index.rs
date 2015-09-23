
use horrorshow::prelude::*;

pub fn
make_html() -> String
{

    let s = html! {

    b { : "something!" }
}.into_string().unwrap();

let res = html! {
html {
  head {
    title { : "Rust News Roundup!" }
    meta(charset="utf-8");
  }
  body {
    div(class="reveal") { div(class="slides") {


    : raw! {s}

    } }
  } // body
}
}.into_string().unwrap();

return res;
}
