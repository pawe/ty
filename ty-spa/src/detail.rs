use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use yew::{Component, ComponentLink, Html, InputData, ShouldRender};
use yew::{events::KeyboardEvent};

use ty_lib::ThankYouDetail;

#[derive(Debug)]
pub enum Msg {
    GetDetails,
    UpdateQuery(String),
    ReceiveResponse(Result<ThankYouDetail, anyhow::Error>),
}

#[derive(Debug)]
pub struct Detail {
    fetch_task: Option<FetchTask>,
    query: String,
    detail: Option<ThankYouDetail>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Detail {
    fn view_detail(&self) -> Html {
        match self.detail {
            Some(ref detail) => {
                let notes = |note: &String| {
                    html! {
                      <li>
                        { note }
                      </li>
                    }
                };

                html! {
                  <>
                    <p>{"Notes for "} <em>{ detail.program.clone() }</em> </p>
                    <ul>
                      { for detail.notes.iter().map(notes)}
                    </ul>
                  </>
                }
            }
            None => {
                html! {}
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! {}
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

impl Component for Detail {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fetch_task: None,
            query: "".to_string(),
            detail: None,
            link,
            error: None,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            UpdateQuery(query) => {
              self.query = query;
              true
            },
            GetDetails => {
                let url = format!("http://localhost:8901/v0/tool/{}/detail", self.query);
                let request = Request::get(url)
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link.callback(
                    |response: Response<Json<Result<ThankYouDetail, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::ReceiveResponse(data)
                    },
                );
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.error = None;
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(detail) => {
                        self.detail = Some(detail);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                self.error = None;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
          <>
            <h2>{"Enter program name"}</h2>
            <input
              value=&self.query
              oninput=self.link.callback(|e: InputData| Msg::UpdateQuery(e.value))
              onkeypress=self.link.batch_callback(|e: KeyboardEvent| {
                if e.key() == "Enter" {
                  vec![Msg::GetDetails]
                } else {
                  vec![]
                }
              })
            />
            { self.view_fetching() }
            { self.view_detail()}
            { self.view_error() }
          </>
        }
    }
}
