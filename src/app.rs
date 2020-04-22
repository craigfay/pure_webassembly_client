use yew::prelude::*;
use yew::services::fetch::FetchTask;
use log::debug;

use crate::error::Error;
use crate::services::{Data};
use crate::types::{Package, Article};

pub struct App {
    link: ComponentLink<Self>,
    value: i64,
    article: Option<Article>,
    current_package_response: Callback<Result<Package, Error>>,
    current_package_task: Option<FetchTask>,
    items: Vec<i32>,
}

pub enum Msg {
    AddOne,
    CurrentPackageResponse(Result<Package, Error>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            article: None,
            current_package_response: link.callback(Msg::CurrentPackageResponse),
            current_package_task: None,
            value: 0,
            link,
            items: vec![1,2,3],
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        let mut data = Data::new();
        let task = data.article(self.current_package_response.clone());
        self.current_package_task = Some(task);

        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::CurrentPackageResponse(Ok(package)) => {
                self.article = Some(package.data.article);
                self.current_package_task = None;
                debug!("{:?}", self.article);
            }
            Msg::CurrentPackageResponse(Err(_)) => {
                self.current_package_task = None;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <link href="main.css" rel="stylesheet"/>
                <div>
                    <button onclick=self.link.callback(|_| Msg::AddOne)>{ "Increment" }</button>
                    <p>{ self.value }</p>
                </div>

                <div>
                    { for self.items.iter().map(renderItem) }
                </div>
            </>
        }
    }
}

fn renderItem(n: &i32) -> Html {
    html! {
        <p>{ "Paragraph" }</p>
    }
}
