
use horrorshow::prelude::*;

pub fn make_html() -> String
{

  let s = html! {
    b { : "something!" }
  }
            .into_string()
            .unwrap();

  let res = html! {
    html {
      head {
        meta(charset="utf-8");
        title: "Rust News Roundup!";
        link(rel="stylesheet", href="reveal.min.css");
      }
      body {
        div(class="reveal") {
          div(class="slides") { : Raw(s) }
        }
      }
    }
  };

  return res.into_string()
            .unwrap();
}
