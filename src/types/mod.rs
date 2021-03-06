mod package;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use package::*;

/// API error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

