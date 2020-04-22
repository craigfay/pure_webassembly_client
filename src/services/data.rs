
use yew::callback::Callback;
use yew::services::fetch::FetchTask;
use dotenv_codegen::dotenv;

use crate::types::Package;
use crate::error::Error;
use super::Requests;

const ARTICLE_API: &str = dotenv!("ARTICLE_API");

pub struct Data {
    requests: Requests,
}

impl Data {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Fetch article data
    pub fn article(
        &mut self,
        callback: Callback<Result<Package, Error>>,
    ) -> FetchTask {
        self.requests.get::<Package>(
            ARTICLE_API.to_string(),
            callback,
        ) 
    }
}

