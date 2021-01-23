// requires the serde and anyhow crates

use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use ty_lib::ThankYouStats;


#[derive(Debug)]
pub enum Msg {
    ReceiveResponse(Result<Vec<ThankYouStats>, anyhow::Error>),
}

#[derive(Debug)]
pub struct FetchServiceExample {
    fetch_task: Option<FetchTask>,
    list: Option<Vec<ThankYouStats>>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

/// Some of the code to render the UI is split out into smaller functions here to make the code
/// cleaner and show some useful design patterns.
impl FetchServiceExample {
    fn view_list(&self) -> Html {
        match self.list {
            Some(ref list) => {
                let list_element = |stats: &ThankYouStats| html!{
                  <li>
                    { stats.program.clone() } <span style="color: #bbb; margin-left: 12px;">{"tys: "} { stats.count } {", notes: "} { stats.note_count }</span>
                  </li>
                };
                
                html! {
                    <ul>{
                        for list.iter().map(list_element)
                    }
                    </ul>
                }
            }
            None => {
                // idk
                html! {
                     <></>
                }
            }
        }
    }
    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}
impl Component for FetchServiceExample {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
      let request = Request::get("http://localhost:8901/v0")
          .body(Nothing)
          .expect("Could not build request.");
      let callback =
          link
              .callback(|response: Response<Json<Result<Vec<ThankYouStats>, anyhow::Error>>>| {
                  let Json(data) = response.into_body();
                  Msg::ReceiveResponse(data)
              });
      let task = FetchService::fetch(request, callback).expect("failed to start request");
     
        Self {
            fetch_task: Some(task),
            list: None,
            link,
            error: None,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            ReceiveResponse(response) => {
                match response {
                    Ok(list) => {
                        self.list = Some(list);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                // we want to redraw so that the page displays the location of the ISS instead of
                // 'fetching...'
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                <h2>{"Most thanked programs"}</h2> 
                { self.view_fetching() }
                { self.view_list() }
                { self.view_error() }
            </>
        }
    }
}