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

use syntax::ext::base as ext_base;
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use syntax_pos::Span;
use syntax::tokenstream::TokenTree;

use std::fs;

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
                &format!("Argument should be a single string identifier. Instead got {} arguments",
                         args.len()));
    return ext_base::DummyResult::any(sp);
  }

  let dirname = token_tree_to_str(&args[0]).unwrap();

  let mut subdocs = Vec::new();

  for entry in fs::read_dir(dirname).unwrap() {
    let next_file = entry.unwrap().path();
    let filename = next_file.as_os_str().to_str().unwrap();
    if filename.ends_with(".yaml") {
      let interned_string = token::intern_and_get_ident(filename); // dir.path().as_str());
      let yaml_item = cx.expr_str(sp, interned_string);
      // subdocs.push(yaml_item);
      subdocs.insert(0, yaml_item);
    }
  }

  let e = cx.expr_vec(sp, subdocs);
  return ext_base::MacEager::expr(e);
}


// pub fn _list_dir<'cx>(cx: &'cx mut ext_base::ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<ext_base::MacResult + 'cx>


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut rustc_plugin::Registry)
{
  reg.register_macro("list_dir", _list_dir);
}
