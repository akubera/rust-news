

use stdweb::web::alert;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod client;

struct Model { }

enum Msg {
  DoIt,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self
  {
    Model { }
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
      // Render your model here
      <button onclick=|_| Msg::DoIt>{ "Click me!" }</button>
    }
  }
}

fn main()
{
  yew::start_app::<Model>();
}
