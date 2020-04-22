use yew::prelude::*;
use yew::services::fetch::FetchTask;
use log::debug;

use web_sys::{Node};
use yew::virtual_dom::VNode;

use crate::error::Error;
use crate::services::{Data};
use crate::types::{Package, Article, ArticleBodyBlock};

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
            }
            Msg::CurrentPackageResponse(Err(_)) => {
                self.current_package_task = None;
            }
        }
        true
    }

    fn view(&self) -> Html {
        return match &self.article {
            Some(article) => html! {
                <>
                    <h1>{ &article.title }</h1>
                    <h2>{ &article.subTitle }</h2>
                    <img src={ &article.heroMedia.image.url } />

                    {
                        for article.articleBody.iter()
                            .map(|b| renderArticleBodyBlock(&b, &article))                        
                    }
                </>
            },
            None => html! {
                <div></div>
            },
        }

    }
}

fn setInnerHtml(tag: &str, innerHtml: &str) -> Html {
    let element = web_sys::window().unwrap()
        .document().unwrap()
        .create_element(tag).unwrap();

    element.set_inner_html(innerHtml);
    let node = Node::from(element);
    VNode::VRef(node)
}

fn renderArticleBodyBlock(block: &ArticleBodyBlock, article: &Article) -> Html {
    match &block.r#type[..] {
        "paragraph" => html! {
           match &block.text.as_ref() {
               Some(innerHtml) => setInnerHtml("p", innerHtml),
               None => html! {}
           }
        },
        "image" => html! {
            <img
                src={ &block.image.as_ref().unwrap().url }
                alt={ &block.image.as_ref().unwrap().title }
            />
        },
        "video" => html! {
            <video
                src={ &article.assets.videoPlaylist[ &block.r#ref.unwrap() ].href }
                controls={true}
            /> 
        },
        _ => html! { <div>{ "Unknown Block" }</div> }
    }
}

