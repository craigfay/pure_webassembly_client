use yew::callback::Callback;
use yew::services::fetch::FetchTask;

use super::Requests;

pub struct Data {
    requests: Requests,
}

impl Data {
    pub fn new() -> Self {
        Self {
            requests: Requests::new(),
        }
    }

    /// Fetch article data package
    pub fn package(
        &mut self,
        callback: Callback<Result<PackageWrapper, Error>>,
    ) -> FetchTask {
        self.requests.get::<PackageWrapper>(
            "".to_string(),
            callback,
        ) 
    }
}

