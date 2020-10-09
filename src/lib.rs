#![recursion_limit="1024"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod header;

use header::Header;

struct Model {
    link: ComponentLink<Self>,
}

pub enum Msg {
  Repaint,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header title="Gregor Holzmann" image="./assets/images/gholtz.svg" />
                <main>
                  <h2>{"Hiya. My name's Gregor Holzmann and I'm a creative software engineer."}</h2>
                  <p>{"I've been working in, on, around, on top of, and beneath the web for around 10 years. As an art school kid who fell in love with engineering, telling stories and solving problems with technology and design are what interest me most. I'm particularly interested in making the web a more accessible and delightful place."}</p>
                  <p>{"Throughout my career, I've worked with companies of all sizes to help improve the lives of their customers and clients, across all kinds of industries: healthcare, finance, insurance, retail, and telecom to name a few."}</p>
                  <p>{"I'm currently working at "} <a href="https://www.ey.com/en_us">{"EY"}</a> {" as a Creative Technologist."}</p>
                  <p>{"In my free time, I'm into cooking, video games, sculpting, cocktails, gardening, cycling, and hanging out with my super cool dog."}</p>
                  <p>{"Feel free to drop me a line, anytime. Here's where you can find me:"}</p>
                  <ul>
                    <li><a href="https://github.com/gregorholzmann/">{"GitHub"}</a></li>
                    <li><a href="https://www.linkedin.com/in/gregor-holzmann-616934116/">{"LinkedIn"}</a></li>
                    <li><a href="https://twitter.com/gregorholzmann">{"Twitter"}</a></li>
                    <li><a href="mailto:hello@gregorholtz.com">{"Email"}</a></li>
                  </ul>
                </main>
                <footer>
                {"This site was generated with "} <a href="https://yew.rs/docs/en/intro/">{"Yew"}</a>{", for really no reason at all."}
                </footer>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
