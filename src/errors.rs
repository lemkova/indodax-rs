use error_chain::error_chain;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IndodaxContentError {
    pub status: i64,
    pub error: String,
    pub error_code: String,
}

error_chain!{
    errors {
        IndodaxError(response: IndodaxContentError)
    }

    foreign_links {
        ReqError(reqwest::Error);
        InvalidHeader(reqwest::header::InvalidHeaderValue);
        SerdeError(serde_json::Error);
    }
}