//! [file]: # (src/walkdir/src/lib.rs)
//!
//! Sub-crate of 'news' that yields the plugins used to
//! generate html from yaml files.
//!

#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate syntax_pos;
extern crate yaml_rust;

use std::collections::BTreeMap;

use syntax::ext::base as ext_base;
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use syntax_pos::Span;
use syntax::tokenstream::TokenTree;
use yaml_rust::{Yaml, YamlLoader};

#[macro_use]
extern crate horrorshow;

use horrorshow::prelude::*;

use std::fs;

fn find_yaml_filenames(dirname: String) -> Vec<String>
{

  let mut result = Vec::new();

  for entry in fs::read_dir(dirname).unwrap() {
    let next_file = entry.unwrap().path();
    let filename = next_file.as_os_str().to_str().unwrap();
    if filename.ends_with(".yaml") {
      result.push(String::from(filename));
      // result.insert(0, yaml_item);
    }
  }
  return result;
}

fn token_tree_to_str(tt: &TokenTree) -> Result<String, String>
{
  match *tt {
    TokenTree::Token(_, token::Ident(s)) => Ok(s.to_string()),
    TokenTree::Token(_, token::Literal(s, _)) => {
      match s {
        token::Lit::Str_(astr) => Ok(astr.to_string()),
        _ => {
          Err("argument should be a string literal".to_string())
          // cx.span_err(sp, "argument should be a string literal");
          // return ext_base::DummyResult::any(sp);
        },
      }
    },
    _ => {
      Err(format!("Argument should be a single string-literal. Found {:?}", tt))
      // cx.span_err(sp,
      //             format!("Argument should be a single string-literal. Found {:?}",
      //                     args[0])
      //               .as_str());
      // return ext_base::DummyResult::any(sp);
    },
  }
}



/// Walks through directory, generating a list of strings
/// containing the names of the contents of the directory
pub fn _list_dir<'cx>(cx: &'cx mut ext_base::ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<ext_base::MacResult + 'cx>
{
  if args.len() != 1 {
    cx.span_err(sp,
                &format!("Argument should be a single string literal. Instead got {} arguments",
                         args.len()));
    return ext_base::DummyResult::any(sp);
  }

  let dirname = token_tree_to_str(&args[0]).unwrap();

  let yaml_file_vec = find_yaml_filenames(dirname);
  let mut subdocs = Vec::new();

  for filename in yaml_file_vec {
    let interned_string = token::intern_and_get_ident(filename.as_str());
    let yaml_item = cx.expr_str(sp, interned_string);
    subdocs.insert(0, yaml_item);
  }

  let e = cx.expr_vec(sp, subdocs);
  return ext_base::MacEager::expr(e);
}

fn build_bullet(bullet: &Yaml) -> Result<String, String>
{
  let bullet_txt = match bullet {
    &Yaml::String(ref s) => String::from(s.as_str()),
    &Yaml::Array(ref a) => build_bullets(a).unwrap(),
    _ => {
      return Err(String::from("Bad Value"));
    },
  };
  return Ok(format!("<li>{}</li>", bullet_txt));
}

/// Builds bullet HTML from yaml data
///
fn build_bullets(bullets: &Vec<Yaml>) -> Result<String, String>
{
  let mut ves: Vec<String> = vec![];

  for b in bullets {
    let bullet_text = build_bullet(b).unwrap();
    ves.push(bullet_text);
  }

  return Ok(format!("<ul>\n{}\n</ul>", ves.join("\n")));
}

fn expand_yaml_slide(slide_data: &std::collections::BTreeMap<yaml_rust::Yaml, yaml_rust::Yaml>) -> String
{
  let TITLE_KEY = Yaml::from_str("title");
  let BULLETS_KEY = Yaml::from_str("bullets");

  let title = match slide_data.get(&TITLE_KEY) {
    Some(&Yaml::String(ref s)) => String::from(s.as_str()),
    _ => String::new(),
  };

  let bullets = match slide_data.get(&BULLETS_KEY) {
    Some(&Yaml::Array(ref a)) => build_bullets(a).unwrap_or(String::new()),
    _ => String::new(),
  };

  // String::from(slide_data[&TITLE_KEY].as_str().unwrap_or(""))
  // } else {
  //   String::new() //""
  // };

  // let bullets = if slide_data.contains_key(&BULLETS_KEY) {
  //   build_bullets(slide_data[&BULLETS_KEY].as_vec().unwrap()).unwrap_or(String::new())
  // } else {
  //   String::new() //""
  // };

  let hypertext = box_html! {
    section {
      h3 {: &title }
      div {: raw!(&bullets) }
    }
  };

  return hypertext.into_string().unwrap();
}

pub fn expand_yaml_file(filename: &str) -> String
{
  use std::fs::File;
  use std::error::Error;
  use std::io::Read;

  let mut f = File::open(filename).unwrap();
  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Err(why) => panic!("Couldn't read {}: {}", &filename, Error::description(&why)),
    _ => (),
  }

  let docs = YamlLoader::load_from_str(s.as_str()).unwrap();
  let doc = &docs[0];

  let title = doc["title"].as_str().unwrap_or("");
  let date = doc["date"].as_str().unwrap_or("");
  let author = doc["author"].as_str().unwrap_or("");

  let slides = doc["slides"].as_vec().unwrap();

  let mut slide_vec = Vec::new();

  for slide in slides.iter() {
    let next_slide = slide.as_hash().unwrap();
    let slide_html = expand_yaml_slide(next_slide);
    slide_vec.push(slide_html);
  }

  let result = box_html! {
    html {
      head {
        meta(charset="utf-8") {}
        title {: "Rust News Roundup"}
        meta(name="viewport", content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no") {}
        // script(src="https://code.jquery.com/jquery-2.1.4.min.js") {}
        script(src="http://code.jquery.com/jquery-3.1.0.min.js", integrity="sha256-cCueBR6CsyA4/9szpPfrX3s49M9vUU5BgtiJj06wt/s=", crossorigin="anonymous") {}
        link(rel="stylesheet", href="/static/reveal.min.css") {}
        link(rel="stylesheet", href="/static/revealjs/theme/simple.css") {}
        // link(rel="stylesheet", href="/static/revealjs/theme/white.css") {}
        style {:raw!("
          .reveal .slides > section {
            left: 0;
          }
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
        ")}
      }
      body {
        div(class="reveal") {
          div(class="slides") {: raw!(&slide_vec.join("\n")) }
        }
        script(src="/static/revealjs/js/head.min.js") {}
        script(src="/static/revealjs/reveal.js") {}
        script {:
          "
            Reveal.initialize({
              controls: false,
              progress: true,
              history: false,
              center: true,
              transition: 'slide',
              margin: 0.1
            });
        "
        }

      }
    }
  };

  return result.into_string().unwrap();
}

pub fn yaml_file_to_html<'cx>(cx: &'cx mut ext_base::ExtCtxt,
                              sp: Span,
                              args: &[TokenTree])
                              -> Box<ext_base::MacResult + 'cx>
{
  let filename = token_tree_to_str(&args[0]).unwrap();
  let html = expand_yaml_file(filename.as_str()); //.unwrap();
  let interned_string = token::intern_and_get_ident(html.as_str());
  let e = cx.expr_str(sp, interned_string);
  return ext_base::MacEager::expr(e);
}


pub fn yaml_files_to_html_vec<'cx>(cx: &'cx mut ext_base::ExtCtxt,
                                   sp: Span,
                                   args: &[TokenTree])
                                   -> Box<ext_base::MacResult + 'cx>
{
  let dirname = token_tree_to_str(&args[0]).unwrap();

  let mut subdocs = Vec::new();

  for i in find_yaml_filenames(dirname).iter() {
    let s = expand_yaml_file(i.as_str());
    let interned_string = token::intern_and_get_ident(s.as_str());
    subdocs.insert(0, cx.expr_str(sp, interned_string));
  }

  let e = cx.expr_vec(sp, subdocs);
  return ext_base::MacEager::expr(e);
}


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut rustc_plugin::Registry)
{
  reg.register_macro("yaml_files_in_dir", _list_dir);
  reg.register_macro("yaml_files_to_html_vec", yaml_files_to_html_vec);
  // reg.register_macro("yaml_file_to_html", yaml_file_to_html);
  // reg.register_macro("map_dir_to_html", map_dir_to_html);
}
