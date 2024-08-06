use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct IndodaxResponse {
    pub success: i16,
    pub return_: Option<Value>,
    pub error: Option<String>,
    pub error_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct AccountInfo {
    pub server_time: i64,
    pub balance: Balance,
    pub balance_hold: Balance,
    pub address: HashMap<String, String>,
    pub network: HashMap<String, Value>,
    pub memo_is_required: HashMap<String, HashMap<String, bool>>,
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub profile_picture: Option<String>,
    pub verification_status: String,
    pub gauth_enable: bool,
    pub withdraw_status: i16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Balance {
    pub idr: i64,
    #[serde(flatten)]
    pub other: HashMap<String, String>,
}