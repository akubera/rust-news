
use horrorshow::prelude::*;

pub fn
make_html() -> String
{

let res = html! {
    html {
      head {
          title { : "Hell world!" }
          meta(charset="utf-8");
      }
      body {
        // attributes
        h1(id="heading") {
            // Insert escaped text
            : "Hello! This is <html />"
        }
        p { // Insert raw text (unescaped)
          : raw!("Let's <i>count</i> to 10!") }
        ol(id="count") {
            // You can embed for loops, while loops, and if statements.
            @ for i in 0..10 {
                li(first? = (i == 0)) {
                    // Format some text.
                    : format_args!("{}", i+1)
                }
            }
        }
        // You need semi-colons for tags without children.
        br; br;
        p {
        // You can also embed closures.
            |tmpl| { tmpl << "Easy!"; }
        }
      }
    }
}.into_string().unwrap();

return res;
}
