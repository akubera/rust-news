///
/// \file slideshow_slide
///

use std::path::Path;

pub struct SlideshowSlide {
    html: String,
}

impl SlideshowSlide {
    pub fn new(&mut self, filename: Path) -> Self {
        yaml_to_html!("data/2016-03-10.yaml"))
        SlideshowSlide { html: "".into_strig() }
    }
}



const HEAD:&'static str = html! {
    head {
        title {: "Rust News Roundup!" }
        meta(charset="utf-8")
        meta(name="viewport", content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, minimal-ui")
        script(src="https://code.jquery.com/jquery-2.1.4.min.js")
        link(rel="stylesheet", href="static/css/reveal.css")
        link(rel="stylesheet", href="static/css/theme/simple.css")
        link(rel="stylesheet" href="static/lib/css/zenburn.css")
        style {
            : "
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
    }
}.into_string().unwrap();


const REVEAL_SCRIPTS:&'static str = html! {
    script(src="static/lib/js/head.min.js")
    script(src="static/js/reveal.js")
    script {:"
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
    "}
}.into_string().unwrap();
