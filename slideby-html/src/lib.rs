//! [file]: #(slideby-html/src/lib.rs)
//!
//! Generate HTML slides for the news report
//!
/*
#![feature(plugin)]
#![plugin(maud_macros)]
extern crate maud;
extern crate maud_pulldown_cmark;
extern crate yaml_rust;

use yaml_rust::Yaml;
use maud_pulldown_cmark::Markdown;
use maud::DOCTYPE;



/// A list of slides slideshow
pub struct Slideshow {
  pub author: String,
  pub date: String,
  pub title: String,
  pub slides: Vec<Slide>,
}

use std::iter::FromIterator;
use maud::Render;

impl Slideshow {
  pub fn get_url(&self) -> String
  {
    format!("/{}.html", self.date)
  }

  pub fn to_revealjs_html_document(&self) -> String
  {
    let mut result = String::new();

    let head = html! {
      meta charset="utf-8" /
      meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, minimal-ui" /
      script src="https://code.jquery.com/jquery-2.1.4.min.js" {}
      link rel="stylesheet" href="/static/reveal.min.css" {}
      link rel="stylesheet" href="/static/css/theme/simple.css" {}
      link rel="stylesheet" href="/static/lib/css/zenburn.css" {}
      style {
          "
              .reveal ul {
                font-family: monospace;
              }
              .reveal section > ul {
                margin-top: 40px;
              }
              .reveal section > ul > ul {
                list-style-type: none;
              }
              .reveal section > ul > ul > li:before {
                content: \"- \";
              }
          "
      }

    };
    (html! {
      (DOCTYPE)
      html {
        head { (head) }
        body {

          @for slide in self.slides.iter() {
            section {
                (slide.title)
            }
          }

          script src="static/lib/js/head.min.js"
          script src="static/js/reveal.js"

          script
          ("
          Reveal.initialize({
            controls: true,
            progress: true,
            history: true,
            center: true,

            transition: 'slide', // none/fade/slide/convex/concave/zoom

            // Optional reveal.js plugins
            dependencies: [
              { src: 'lib/js/classList.js', condition: function() { return !document.body.classList; } },
              { src: 'plugin/markdown/marked.js', condition: function() { return !!document.querySelector( '[data-markdown]' ); } },
              { src: 'plugin/markdown/markdown.js', condition: function() { return !!document.querySelector( '[data-markdown]' ); } },
              { src: 'plugin/highlight/highlight.js', async: true, callback: function() { hljs.initHighlightingOnLoad(); } },
              { src: 'plugin/zoom-js/zoom.js', async: true },
              { src: 'plugin/notes/notes.js', async: true }
            ]
          });
          ")
        }
      }})
      .render_to(&mut result);
    return result;
    // html! {
    // // ("<!DOCTYPE html>")
    // html {
    //   head {
    //     meta charset="utf-8"
    //     meta name="viewport"
    //     script src="https://code.jquery.com/jquery-2.1.4.min.js"
    //   }
    //   body {
    //     p "Hello world"
    //     // script "
    //     // Reveal.initialize({
    //     //   controls: true
    //     // })
    //     // "
    //   }
    // }}.into_string()
  }
}

pub struct Slide {
  pub title: String,
}

impl maud::Render for Slide {
  fn render(&self) -> maud::Markup
  {
    html! {
          section {

          }
      }
  }
}

impl<'a> From<&'a Yaml> for Slide {
  fn from(yaml: &'a Yaml) -> Self
  {
    Slide { title: yaml["title"].as_str().unwrap().into() }
  }
}

impl<'a> From<&'a Yaml> for Slideshow {
  fn from(yaml: &Yaml) -> Self
  {
    let title = yaml["title"].as_str().unwrap().into();
    let author = yaml["author"].as_str().unwrap().into();
    let date = yaml["date"].as_str().unwrap().into();

    let yaml_slides = yaml["slides"].as_vec().unwrap();

    let slides = yaml_slides.iter().map(|y| y.into()).collect();

    // let slides = Vec::new();
    Slideshow {
      author: author,
      title: title,
      date: date,
      slides: slides,
    }

  }
}
*/
