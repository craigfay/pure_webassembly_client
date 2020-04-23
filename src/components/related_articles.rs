
use yew::prelude::*;
use crate::types::ArticlePreview;

pub enum Msg {
    Increase,
}

pub struct RelatedArticles {
    props: Props,
    link: ComponentLink<Self>,
}


#[derive(Clone, Properties)]
pub struct Props {
    pub previews: Vec<ArticlePreview>,
}

impl Component for RelatedArticles {
    type Properties = Props;
    type Message = Msg;


    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        RelatedArticles { props, link }
    }


    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }


    fn view(&self) -> Html {
        html! {
            <h1>{ "Related" }</h1>
        }
    }
}

