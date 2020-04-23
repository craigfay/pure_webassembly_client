
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
            <>
                <header>
                    <h3>{ "Related Articles" }</h3>
                </header>
                <div class={"grid"}>
                    { for self.props.previews.iter().map(renderArticlePreview) }
                </div>
            </>
        }
    }
}

fn renderArticlePreview(preview: &ArticlePreview) -> Html {
    html! {
        <div class={"article-preview"}>
            <img src={ &preview.thumbnail.url } />
            <header>
                <h4>{ &preview.title }</h4>
                <span class={"contributor"}>{ &preview.contributors[0].displayName }</span>
            </header>
        </div>
    }
}

