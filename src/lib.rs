#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]


// pub mod yaml_to_slides;


extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;


#[macro_use]
extern crate horrorshow;


use std::io::prelude::*;
use syntax::codemap::Span;
use syntax::parse::token;
use syntax::ast::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use rustc_plugin::Registry;
use std::fs::File;
use std::error::Error;
// use syntax::parse::token::InternedString;
// use syntax::util::interner::{RcStr, StrInterner};

use horrorshow::prelude::*;

extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};  // , YamlEmitter};

const HEAD:&'static str = "<head>
<meta charset='utf-8'>
<title>Rust News Roundup!</title>

<meta name='viewport' content='width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, minimal-ui'>
<script src='https://code.jquery.com/jquery-2.1.4.min.js'></script>

<link rel='stylesheet', href='static/css/reveal.css'>
<!--link rel='stylesheet', href='css/theme/moon.css'-->
<!--link rel='stylesheet' href='css/theme/black.css' id='theme'-->
<link rel='stylesheet', href='static/css/theme/simple.css'>

<link rel='stylesheet' href='static/lib/css/zenburn.css'>
<style>
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
</style>
</head>
";


const REVEAL_SCRIPTS:&'static str = "
  <script src='static/lib/js/head.min.js'></script>
  <script src='static/js/reveal.js'></script>
  <script>
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
</script>";


fn build_bullets(bullets: &Vec<Yaml>) -> Result<String, &str>
{
  let mut res: String = String::new();
  res.push_str("<ul>");


  let mut ves: Vec<String> = vec!();

  for b in bullets {
    match *b {
        Yaml::String(ref s) => //res.push(&*s),
            ves.push(format!("<li>{}</li>", s)),
        Yaml::Array(ref a) =>
            ves.push(format!("<ul>{}</ul>", build_bullets(a).unwrap())),
        _ => {
            return Err("Bad bullets value!"); //ves.join("\n");
        }
    }
  }
  return Ok(ves.join("\n"));
  // res
  // return format!("<ul>{}</ul>", res.join("\n"));
}


pub fn expand_rn(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
    -> Box<MacResult + 'static>
{
    if args.len() != 1 {
        cx.span_err(
            sp,
            &format!("argument should be a single identifier, but got {} arguments", args.len())
        );
        return DummyResult::any(sp);
    }


    let text = match args[0] {
        TokenTree::Token(_, token::Ident(s, _)) => s.to_string(),
        TokenTree::Token(_, token::Literal(s, _)) => match s {
            token::Lit::Str_(astr) => astr.to_string(),
            _ => {
                cx.span_err(sp, "argument should be a string literal");
                return DummyResult::any(sp);
            }
        },
        _ => {
            cx.span_err(sp, "argument should be a single identifier literal");
            return DummyResult::any(sp);
        }
    };
    println!(">> {:?}", text);

    let mut f = File::open(&text).unwrap();
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", &text,
                                                   Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", &text, s),
    }

    let s = &*s;

    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];


    let title_slide = html! {
        section {
          h1 {: doc["title"].as_str().unwrap() }
          p {: doc["date"].as_str().unwrap() }
          p { small {: doc["author"].as_str().unwrap() }}
        }
    }.into_string().unwrap();

    let mut slide_vec = vec![title_slide];
    let title_key = Yaml::from_str("title");
    let bullets_key = Yaml::from_str("bullets");

    for d in doc["slides"].as_vec().unwrap() {
        let next_slide = d.as_hash().unwrap();

        let title = if next_slide.contains_key(&title_key) {
          next_slide[&title_key].as_str().unwrap()
        } else {
          ""
        };

        let bullets = if next_slide.contains_key(&bullets_key) {
          build_bullets(d["bullets"].as_vec().unwrap()).unwrap()
        } else {
          "".into()
        };



        slide_vec.push( html! {
            section {
                h3 {: title}
                ul {: raw!(bullets) }
            }
        }.into_string().unwrap());
    }

    let content = slide_vec.join("\n");

    let BODY = format!(
        "<div class='reveal'><div class='slides'>{content}</div></div>",
        content = content
    );
    let BODY = format!("{}{}",
        BODY,
        REVEAL_SCRIPTS
    );

    // BODY += String::from_str(REVEAL_SCRIPTS);
    let HTML = format!("<!DOCTYPE html><html>{head}<body>{body}</body></html>",
                       head = HEAD,
                       body = BODY);


    println!(":: {}", HTML);

    return MacEager::expr(cx.expr_str(sp, token::intern_and_get_ident(&*HTML)));

}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("yaml_to_html", expand_rn);
}
