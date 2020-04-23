use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub data: ArticleWrapper,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleWrapper {
    pub article: Article, 
}


#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub title: String, 
    pub subTitle: String,
    pub publishedDate: String,
    pub heroMedia: Media,
    pub articleBody: Vec<ArticleBodyBlock>,
    pub assets: Assets,
    pub relatedArticles: Vec<ArticlePreview>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticlePreview {
    pub title: String, 
    pub publishedDate: String,
    pub contributors: Vec<Contributor>,
    pub thumbnail: Thumbnail,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    pub displayName: String, 
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String, 
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub r#type: String,
    pub r#ref: i32,
    pub image: Image,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleBodyBlock {
    pub r#type: String,
    pub r#ref: Option<i32>,
    pub text: Option<String>,
    pub image: Option<Image>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Assets {
    pub videoPlaylist: HashMap<i32, Video>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub title: String,
    pub href: String,
}

