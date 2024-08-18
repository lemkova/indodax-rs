use crate::{
    client::Client, 
    errors::Result,
    model::{AccountInfo, OrdersHistory},
    api::{API,Private},
};
use serde::de::DeserializeOwned;
use reqwest::header::{HeaderMap, HeaderValue, HeaderName, CONTENT_TYPE, USER_AGENT};
use std::time::{SystemTime, UNIX_EPOCH};

use urlencoding::encode;

use std::collections::HashMap;
use ring::hmac;


pub struct PrivateClient {
    client: Client,
    api_key: String,
    secret_key: String,
}

impl PrivateClient {
    pub fn new(api_key: String, secret_key: String, host: String) -> Self {
        PrivateClient {
            client: Client::new(host),
            api_key: api_key,
            secret_key: secret_key,
        }
    }

    pub fn get_info(&self) -> Result<AccountInfo> {
        self.post_request(API::Private(Private::GetInfo), None)
    }

    pub fn get_order_history(&self, pair: &str, count: i32) -> Result<OrdersHistory> {
        let count = count.to_string();
        let params = vec![("pair", pair), ("count", &count)];
        self.post_request(API::Private(Private::OrderHistory), Some(params))
    }

    pub fn post_request<T: DeserializeOwned>(&self, method: API, _params: Option<Vec<(&str, &str)>>) -> Result<T> {
        let client = &self.client;
        let mut params: HashMap<String, String>  = HashMap::new();

        params.insert("method".to_string(), method.into());
        
        if let Some(params) = _params {
            for (key, value) in params {
                params.insert(key.to_string(), value.to_string());
            }
        }

        let timestamp = build_timestamp_vec();

        for (key, value) in timestamp {
            params.insert(key, value);
        }

        let pre_sign = hash_map_to_url_encoded(&params);
        let signature = self.sign_query(pre_sign);
        let headers = self.build_private_headers(signature)?;

        // insert params to post 
        client.post::<T>(params, Some(headers))
    }

    fn build_private_headers(&self, sign: String) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("lmkv-bot"));
        custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
        custom_headers.insert(HeaderName::from_static("key"), HeaderValue::from_str(&self.api_key.as_str())?);
        custom_headers.insert(HeaderName::from_static("sign"), HeaderValue::from_str(&sign)?);
    

        Ok(custom_headers)
    }

    fn sign_query(&self, query: String) -> String {
        let key = hmac::Key::new(hmac::HMAC_SHA512, self.secret_key.as_bytes());
        let signature = hmac::sign(&key, query.as_bytes());
        bytes_to_hex(signature.as_ref())
    }    
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn build_timestamp_vec() -> Vec<(String, String)> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    vec![("timestamp".to_string(), timestamp.to_string()), ("recvWindow".to_string(), (timestamp + 5000).to_string())]
}

fn hash_map_to_url_encoded(map: &HashMap<String, String>) -> String {
    map.iter()
        .map(|(key, value)| format!("{}={}", encode(key), encode(value)))
        .collect::<Vec<String>>()
        .join("&")
}