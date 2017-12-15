//! [file]: # (src/lib.rs)
//! [author]: # (Andrew Kubera <andrewkubera@gmail.com>)
//!
//! Main file for the news server
//!

#![feature(proc_macro)]

extern crate regex;
use regex::Regex;

extern crate rouille;
use rouille::{Request, Response};

extern crate maud;
use maud::{html, PreEscaped, Render};

extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};
use yaml_rust::yaml::{Array, Hash};

extern crate walkdir;
use walkdir::WalkDir;

use std::ffi::OsString;
use std::path::{Path, Display};
use std::fs::File;
use std::io::Read;

extern crate maud_pulldown_cmark;
use maud_pulldown_cmark::Markdown;

extern crate pulldown_cmark;
// use pulldown_cmark::html::push_html;
use pulldown_cmark::Parser;



fn main()
{
  rouille::start_server("0.0.0.0:9090", handle_requests);
}

fn handle_requests(request: &Request) -> Response
{
  if request.url() == "/" {
    index_page()
  }
  else if let Some(request) = request.remove_prefix("/static/") {
    println!(" -> Requested static file: {:?}", request.url());
    println!("      exists? {:?}", Path::new("client").join(request.raw_url()).exists());
    rouille::match_assets(&request, "client")
  }
  else if let Some(request) = request.remove_prefix("/data/") {
  // else if request.url().starts_with("/data") {
    println!("Requested: {:?}", request.url());

    let path = Path::new("data").join(request.raw_url());
    println!("Loading {:?} (exists: {:?})", path.to_str().unwrap(), path.exists());
    let mut buffer = String::new();
    if let Err(ioerr) = File::open(path).map(|mut file| file.read_to_string(&mut buffer)) {
      Response::html(html! {
        html {
          head { title { "error" } }
          body { h2 { "ERROR" }
                 p { " There was an error: " }
                 pre { (ioerr) }
               }
        } } ).with_status_code(404)
    } else {
      Response::html(yaml_src_to_slide(buffer))
    }
  } else {
    Response::html(html! { head { title { "Not Found" } }
                             body { h1 {"404 - Not Found"} }})
    .with_status_code(404)
  }
}

fn index_page() -> Response
{
  let yaml_files = WalkDir::new("data")
    .into_iter()
    .filter(|e| e.is_ok())
    .map(|e| e.unwrap() )
    .filter(|path| path.path().extension() == Some(&OsString::from("yaml")));

  // regex()
  Response::html(html! {
      html {
          head {
            meta charset="utf-8" {}
            title { "Rust News Roundup!" }
          }
          body style="max-width: 600px; margin: 40px auto; display:block;" {
            h1 style="text-align:center;" { "Rust News!" }
            ul {
              @for path in yaml_files {
                  li (news_link(path.path()))
              }
            }
          }
      }
      })
}


fn news_link(path: &Path) -> maud::PreEscaped<String>
{
  let link = path.display();

  // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
  // path.
  html! {
    a href=(link) { (path.display()) }
  }
}

fn yaml_src_to_slide(src: String) -> maud::PreEscaped<String>
{
  let yaml_result = YamlLoader::load_from_str(&src);
  let yaml = match yaml_result {
    Err(err) => return html! { html { head { title { "ERROR" } }
                                      body { h1 { "ERROR" }
                                            pre { (err) } } } },
    Ok(ref docs) => &docs[0]
  };

  let title = yaml["title"].as_str().unwrap();
  let author = yaml["author"].as_str().unwrap();
  let date = yaml["date"].as_str().unwrap();
  let slide_iter = yaml["slides"].as_vec().unwrap().into_iter();

  html! {
    html {
      head {
        meta charset="utf-8" {}
        meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, minimal-ui" {}
        link href="/static/revealjs/css/reveal.css" rel="stylesheet" {}
        link href="/static/revealjs/css/theme/simple.css" rel="stylesheet" id="theme" {}
        // link href="/static/revealjs/css/theme/black.css" rel="stylesheet" id="theme" {}
        link href="/static/revealjs/lib/css/zenburn.css" rel="stylesheet" {}
        link href="/static/github.css" rel="stylesheet" {}
        script src="https://code.jquery.com/jquery-2.1.4.min.js" {}
        style {"
              .reveal ul, .reveal div {
                font-family: Helvetica, sans;
                font-weight: lighter;
              }
              .reveal section > ul {
                list-style-type: none;
              }
              .reveal section > ul > li:before {
                content: \"• \";
              }
              .reveal section > ul > li:before {
                content: \"• \";
              }
              .reveal section > ul > ul {
                list-style-type: none;
              }
              .reveal section > ul > ul > li:before {
                content: \"- \";
              }
              .reveal code {
                background-color: #F0F0F0;
                padding: 4px 15px;
              }
              "}
      }
      body style="transition: -webkit-transform 0.8s ease 0s;" {
        div class="reveal slide center has-vertical-slides has-horizontal-slides ready" {
          div class="slides" {
            section {
              h2 { (escape(title)) }
              p { (date) }
              p { small { (author) } }
            }
            @for slide_data in slide_iter {
              (make_slide(slide_data.as_hash().unwrap()))
            }
          }
        }
        script type="text/javascript" src="/static/revealjs/lib/js/head.min.js" {}
        script type="text/javascript" src="/static/revealjs/js/reveal.js" {}
        script type="text/javascript" src="/static/revealjs/plugin/markdown/marked.js" {}
        script type="text/javascript" src="/static/revealjs/plugin/markdown/markdown.js" {}
        script type="text/javascript" src="/static/revealjs/plugin/highlight/highlight.js" {}
        script { "
            Reveal.initialize({
              controls: true,
              progress: true,
              history: true,
              center: true,

              transition: 'slide', // none/fade/slide/convex/concave/zoom

              // Optional reveal.js plugins
              dependencies: [
                { src: '/static/revealjs/lib/js/classList.js', condition: function() { return !document.body.classList; } },
                { src: '/static/revealjs/plugin/markdown/marked.js', condition: function() { return !!document.querySelector( '[data-markdown]' ); } },
                { src: '/static/revealjs/plugin/markdown/markdown.js', condition: function() { return !!document.querySelector( '[data-markdown]' ); } },
                { src: '/static/revealjs/plugin/highlight/highlight.js', async: true, callback: function() { hljs.initHighlightingOnLoad(); } },
                { src: '/static/revealjs/plugin/zoom-js/zoom.js', async: true },
                { src: '/static/revealjs/plugin/notes/notes.js', async: true }
              ]
            });
            " }
      }
    }
  }
}

fn make_slide(data: &Hash) -> maud::PreEscaped<String>
{
  html! {
    section {
      @if let Some(title) = data.get(&Yaml::from_str("title")) {
        h3 { (escape(title.as_str().unwrap())) }
      }
      @if let Some(bullets) = data.get(&Yaml::from_str("bullets")) {
        @if let Some(vec) = bullets.as_vec() {
          (make_bullets(vec))
        }
      }
    }
  }
}


fn make_bullets(data: &Array) -> maud::PreEscaped<String>
{
  html! {
    ul {
      @for b in data {
        @match *b {
          Yaml::String(ref s) => {
            li { (escape(s)) }
          },
          Yaml::Array(ref a) => {
            { (make_bullets(a)) }
          },
          _ => {}
        }
      }
    }
  }
}

// fn make_bullet(s: &str) -> maud::PreEscaped<String>
// fn make_bullet(s: &str) -> maud::PreEscaped<String>
// {
//   // println!("> {}",s);
//   // let mut html_buff = String::new();
//   // let md = parser.html();
//   return PreEscaped(format_str(s));
// }

// fn escape(s: &str) -> String
fn escape(s: &str) -> maud::PreEscaped<String>
{
  let mut html_buff = String::new();
  let parser = Parser::new(s);
  pulldown_cmark::html::push_html(&mut html_buff, parser);
  // return html_buff.replace("{{br}}", "<br/>");
  // return html_buff;

  // return PreEscaped(html_buff.replace("{{br}}", "<br/>"));
  return PreEscaped(custom_format(html_buff));
}


fn custom_format(s: String) -> String
{
  s.replace("{{br}}", "<br/>")
}
