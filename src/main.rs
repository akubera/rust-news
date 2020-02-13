#![allow(unused_imports)]

use stdweb::{web::{self, alert, IElement}, console, js};
// use horrorshow::html as HTML;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use slideby::{SlideShow, Slide, Bullet};

include!(concat!(env!("OUT_DIR"), "/precompiled_slides.rs"));


enum Msg {
  DoIt,
  SelectNews(usize),
}


struct Model {
  slideshows: Vec<SlideShow>,
  selected: Option<u8>,
}

impl Model {
  fn draw_index(&self) -> Html<Self>
  {
    html! {
      <>
        <ul>
          { for self.slideshows.iter().enumerate().rev().map(|(idx, s)| self.slideshow_link(idx, s)) }
        </ul>
        <button onclick=|_| Msg::DoIt>{ "Click me!" }</button>
      </>
    }
  }

  fn draw_slide(&self, index: usize) -> Html<Self>
  {
    /*
    let head = web::document().head().unwrap();
    let body = web::document().body().unwrap();

    let scripts = HTML! {
      script(src="/js/jquery-3.4.1.min.js") {}
      script(src="/revealjs/reveal.js") {}
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

    head.append_html(&format!("{}", scripts));

    let content = HTML! {
      div {
        h1 {: " TEXT" }
      }
    };

    body.append_html(&format!("{}", content));
    */

    let slideshow = &self.slideshows[index];
    html! {
      <>
        <script src="/js/jquery-3.4.1.min.js"></script>
        <script src="/revealjs/reveal.js"></script>
        <script>{r#"
          var reveal_load = setInterval(() => {
            if (typeof Reveal === "undefined") { return; }
            clearInterval(reveal_load);

            Reveal.initialize({
              controls: true,
              progress: true,
              history: true,
              center: true,

              transition: 'slide',
            })
          });
        "#}</script>
        <link rel="stylesheet" href="/reveal.min.css" />
        <link rel="stylesheet" href="/revealjs/theme/simple.css" />
        <style>{r##"
          .reveal .slides > section {
            left: 0;
          }
          .reveal section ul {
            font-family: monospace;
          }
          .reveal section > ul {
            margin-top: 40px;
          }
          .reveal section > ul ul {
            margin-bottom: 20px;
          }
          .slides > section > ul li {
            margin-top: 10px;
          }
          .reveal section ul {
            list-style-type: none;
          }
          .slides > section > ul > li > span:before {
            content: \"• \";
          }
          .reveal section > ul > li > ul > li > span:before {
            content: \"› \";
          }
          .reveal pre {
            display: inline-block;
            box-shadow: none;
            border: thin dashed black;
            padding-left: 14pt;
            line-height: 2.5em;
          }
        "##}</style>

        <div class="reveal">
          { slideshow_to_html(&slideshow) }
        </div>
      </>
    }
  }

  fn slideshow_link(&self, idx: usize, slideshow: &SlideShow) -> Html<Model>
  {
    html! {
      <li style={"margin: 5px 0;"}><button onclick=|_| { Msg::SelectNews(idx) } >{ slideshow.date.clone() } </button></li>
    }
  }

}

fn slideshow_to_html(slideshow: &SlideShow) -> Html<Model>
{
  let title_slide = html! {
    <section>
      <h1> { &slideshow.title } </h1>
      <p> { &slideshow.date } </p>
      <p> <small> { &slideshow.author } </small> </p>
    </section>
  };

  let bullets_to_html = |bullets: &Vec<Bullet>| { html! {
    <ul> {
      for bullets.iter().map(|b| match b {
        Bullet::Text(s) => html! { <li> { &s }</li> },
        Bullet::SubBullets(b) => html! { },
      })
    }    /*
      {
        for bullets.iter().map(|b| html! {
          <li> {
            if b.text.len() != 0 {
              html! { {&b.text} }
            } else if b.bullets.len() > 0 {
              html!  {}
            } else {
              html!  {}
            }
         } </li> })
      }
      */
    </ul>
  }};

  let slide_to_html = |s: &Slide| { html! {
    <section>
      {
        if let Some(title) = &s.title {
          html! { <h3> { title } </h3> }
        } else {
          html! {}
        }
      }

      {bullets_to_html(&s.bullets)}
    </section>
  }};

  html! {
    <div class="slides">
      { title_slide }
      { for slideshow.slides.iter().map(slide_to_html) }
    </div>
  }
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self
  {
    console!(log, "Creating Model");
    Model {
      slideshows: load_slideshows(),
      selected: None,
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender
  {
    match msg {
      Msg::DoIt => {
        // Update your model on events
        alert("Thanks");
        true
      },
      Msg::SelectNews(idx) => {
        self.selected = Some(idx as u8);
        true
      }
    }
  }

  fn view(&self) -> Html<Self>
  {
    if let Some(s) = self.selected {
      self.draw_slide(s as usize)
    } else {
      self.draw_index()
    }
  }
}

fn main()
{
  yew::start_app::<Model>();
}
