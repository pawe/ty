use comrak::{markdown_to_html, ComrakOptions};
use wasm_bindgen::prelude::*;
use yew::{prelude::*, Component, ComponentLink, Html, ShouldRender};

mod detail;
mod list;

use detail::Detail;
use list::FetchServiceExample;

use lazy_static::lazy_static;

lazy_static! {
    static ref BASEURL: String = "https://ty.paulweissenbach.com".into();
}

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // propably not a good idea to keep this in the view fn
        let readme = yew::utils::document().create_element("div").unwrap();
        readme.set_inner_html(&markdown_to_html(
            include_str!("../../README.md"),
            &ComrakOptions::default(),
        ));

        let readme_html = Html::VRef(readme.into());

        html! {
            <>
                {readme_html}
                <FetchServiceExample />
                <Detail />
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
