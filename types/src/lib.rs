use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    #[serde(skip)]
    pub file_name: String,
    pub command: String,
    pub args: Vec<String>,
    pub log_file: String,
}
