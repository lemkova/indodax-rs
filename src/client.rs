use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::blocking::Response;
use reqwest::StatusCode;
use crate::{
    errors::{IndodaxContentError,Result, ErrorKind},
    model::IndodaxResponse,
};
use serde::de::DeserializeOwned;

use std::collections::HashMap;
#[derive(Clone)]
pub struct Client {
    base_url: String,
    pub req_client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(base_url: String) -> Self {
        Client {
            base_url,
            req_client: reqwest::blocking::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    pub fn post<T: DeserializeOwned>(&self, params: HashMap<String, String>, header: Option<HeaderMap<HeaderValue>>) -> Result<T> {
        let client = &self.req_client;
        // path = base_url + "tapi"
        let path = format!("{}/tapi", self.base_url);
        let mut request = client.post(&path);

        if let Some(header) = header {
            request = request.headers(header);
        }

        let response = request
            .form(&params)
            .send()?;

        self.handler(response)
    }

    pub fn get<T: DeserializeOwned>(&self, path: String, params: Vec<(&str, &str)>, header: Option<HeaderMap<HeaderValue>>) -> Result<T> {
        let client = &self.req_client;
        let endpoint = format!("{}{}", self.base_url, path);
        let mut request = client.get(&endpoint);

        if let Some(header) = header {
            request = request.headers(header);
        }

        let response = request
            .query(&params)
            .send()?;

        self.handler(response)
    }

    fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        match response.status() {
            StatusCode::OK => {
                let indodax_response: IndodaxResponse  = response.json()?;
                if indodax_response.success == 1 {
                    if let Some(return_value) = indodax_response.return_ {
                        Ok(serde_json::from_value(return_value)?)
                    } else {
                        Err("Missing return value".into())
                    }
                } else {
                    let error = IndodaxContentError {
                        status: indodax_response.success as i64,
                        error: indodax_response.error.unwrap_or_else(|| "Unknown error".into()),
                        error_code: indodax_response.error_code.unwrap_or_else(|| "Unknown error".into()),
                    };
                    Err(ErrorKind::IndodaxError(error).into())
                }
            },
            StatusCode::BAD_REQUEST => Err("Bad request".into()),
            StatusCode::UNAUTHORIZED => Err("Unauthorized".into()),
            StatusCode::FORBIDDEN => Err("Forbidden".into()),
            StatusCode::NOT_FOUND => Err("Not found".into()),
            StatusCode::INTERNAL_SERVER_ERROR => Err("Internal server error".into()),
            StatusCode::SERVICE_UNAVAILABLE => Err("Service unavailable".into()),
            _ => Err("Unknown error".into()),
        }
    }
}