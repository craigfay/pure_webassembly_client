use yew::prelude::*;
use yew::services::fetch::FetchTask;

use crate::error::Error;
use crate::services::{Data};
use crate::types::{Package, PackageWrapper};


pub struct App {
    link: ComponentLink<Self>,
    value: i64,
    current_package: Option<Package>,
    current_package_response: Callback<Result<PackageWrapper, Error>>,
    current_package_task: Option<FetchTask>,
}

pub enum Msg {
    AddOne,
    CurrentPackageResponse(Result<PackageWrapper, Error>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            current_package: None,
            current_package_response: link.callback(Msg::CurrentPackageResponse),
            current_package_task: None,
            value: 0,
            link,
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
            Msg::CurrentPackageResponse(Ok(wrapper)) => {
                self.current_package = Some(wrapper.package);
                self.current_package_task = None;
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
            </>
        }
    }
}

