#![allow(non_snake_case)]
use horrorshow::prelude::*;

pub fn REVEAL_SCRIPTS() -> String
{
  let txt = html! {
    script(src="/static/lib/js/head.min.js") {}
    script(src="/static/js/reveal.min.js") {}
    script {:
      "
        Revleal.initialize({
        controls: true,
        progress: true,
        history: true,
        center: true,

        transition: 'slide',
      })
    "
    }
  };
  return txt.into_string().unwrap();
}


pub fn HEAD() -> String
{
  let hypertext = html!{
    meta(charset="utf-8") {}
    title {: "Rust News Roundup"}
    meta(name="viewport", content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, minimal-ui") {}
    script(src="/js/jquery-3.4.1.min.js") {}
    link(rel="stylesheet", href="/static/reveal.min.css") {}
    link(rel="stylesheet", href="/revealjs/theme/simple.css") {}
  };

  return hypertext.into_string().unwrap();
}
