use yew::prelude::*;

pub struct Header {
    link: ComponentLink<Self>,
    title: String,
    image: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub title: String,
    pub image: String,
}

pub enum Msg {
  Repaint,
}

impl Component for Header {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
      Header {
            link,
            title: props.title,
            image: props.image,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.image = props.image;
        true
    }

    fn view(&self) -> Html {
        html! {
            <header>
                <h1>{ &self.title }</h1>
                <img src=&self.image alt=&self.title />
            </header>
        }
    }
}
