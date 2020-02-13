

use stdweb::web::alert;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use slideby::SlideShow;

include!(concat!(env!("OUT_DIR"), "/precompiled_slides.rs"));

struct Model {
  slideshows: Vec<SlideShow>,
}

enum Msg {
  DoIt,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self
  {
    Model {
      slideshows: load_slideshows()
    }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender
  {
    match msg {
      Msg::DoIt => {
        // Update your model on events
        alert("Thanks");
        true
      }
    }
  }

  fn view(&self) -> Html<Self>
  {
    html! {
      <>
        <ul>
          { for self.slideshows.iter().map(slideshow_link) }
        </ul>
        <button onclick=|_| Msg::DoIt>{ "Click me!" }</button>
      </>
    }
  }
}

fn slideshow_link(slideshow: &SlideShow) -> Html<Model>
{
  html! {
    <li>{ slideshow.date.clone() }</li>
  }
}

fn main()
{
  yew::start_app::<Model>();
}
