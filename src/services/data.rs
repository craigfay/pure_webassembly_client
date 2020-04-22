
use yew::callback::Callback;
use yew::services::fetch::FetchTask;
use dotenv_codegen::dotenv;

use crate::types::PackageWrapper;
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
        callback: Callback<Result<PackageWrapper, Error>>,
    ) -> FetchTask {
        self.requests.get::<PackageWrapper>(
            ARTICLE_API.to_string(),
            callback,
        ) 
    }
}

