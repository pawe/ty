use wasm_bindgen::prelude::*;
use yew::prelude::*;

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
        html! {
            <>
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
