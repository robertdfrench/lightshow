use anyhow::anyhow;

use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use serde_cbor;
use doors;

#[derive(Serialize, Deserialize)]
pub enum Query {
    Counter {
        key: String,
        method: Method
    },
    Text(TextMethod)
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Counter(u64),
    Text(String),
}

#[derive(Serialize, Deserialize)]
pub enum Method {
    Create,
    Delete,
    Get,
    Increment,
}

#[derive(Serialize, Deserialize)]
pub enum TextMethod {
    Delete{ key: String },
    Read{ key: String },
    Write{ key: String, value: String },
}

pub static PATH: &str = "/tmp/doordb";

pub struct Client {
    door: doors::Client,
}

impl Client {
    pub fn new() -> Self {
        let door = doors::Client::open(PATH).expect("Failed to open door");
        Self{ door }
    }

    pub fn text_delete(&self, key: &str) -> Result<String> {
        let query = Query::Text(TextMethod::Delete{key: key.to_string()});
        let query_bytes = serde_cbor::to_vec(&query).expect("Failed to serialize query");
        let response = self.door.call_with_data(&query_bytes).expect("Door call failed");
        let response: Result<Response, String> =
            serde_cbor::from_slice(response.data()).expect("Failed to deserialize response");
        match response {
            Ok(r) => match r {
                Response::Text(value) => Ok(value),
                _ => Err(anyhow!("Response from server was not a text")),
            },
            Err(context) => Err(anyhow!(context))
        }
    }

    pub fn text_read(&self, key: &str) -> Result<String> {
        let query = Query::Text(TextMethod::Read{key: key.to_string()});
        let query_bytes = serde_cbor::to_vec(&query).expect("Failed to serialize query");
        let response = self.door.call_with_data(&query_bytes).expect("Door call failed");
        let response: Result<Response, String> =
            serde_cbor::from_slice(response.data()).expect("Failed to deserialize response");
        match response {
            Ok(r) => match r {
                Response::Text(value) => Ok(value),
                _ => Err(anyhow!("Response from server was not a text")),
            },
            Err(context) => Err(anyhow!(context))
        }
    }

    pub fn text_write(&self, key: &str, value: &str) -> Result<String> {
        let query = Query::Text(TextMethod::Write{key: key.to_string(), value: value.to_string()});
        let query_bytes = serde_cbor::to_vec(&query).expect("Failed to serialize query");
        let response = self.door.call_with_data(&query_bytes).expect("Door call failed");
        let response: Result<Response, String> =
            serde_cbor::from_slice(response.data()).expect("Failed to deserialize response");
        match response {
            Ok(r) => match r {
                Response::Text(value) => Ok(value),
                _ => Err(anyhow!("Response from server was not a text")),
            },
            Err(context) => Err(anyhow!(context))
        }
    }

    pub fn counter_query(&self, method: Method, key: &str) -> Result<u64> {
        let query = Query::Counter { key: key.to_string(), method };
        let query_bytes = serde_cbor::to_vec(&query).expect("Failed to serialize query");
        let response = self.door.call_with_data(&query_bytes).expect("Door call failed");
        let response: Result<Response, String> =
            serde_cbor::from_slice(response.data()).expect("Failed to deserialize response");
        match response {
            Ok(r) => match r {
                Response::Counter(value) => Ok(value),
                _ => Err(anyhow!("Response from server was not a counter")),
            },
            Err(context) => Err(anyhow!(context))
        }
    }
}
