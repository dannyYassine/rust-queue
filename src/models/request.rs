use std::{collections::HashMap, fmt::Debug, str::FromStr};

use axum::{body::Body, extract::Request as AxumRequest};
use http_body_util::BodyExt;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    #[serde(flatten)]
    pub params: HashMap<String, String>,
}

impl QueryParams {
    pub fn get<'a, T>(&self, key: &'a str) -> Option<T>
    where
        T: FromStr + Debug,
    {
        let value = self.params.get(key);

        if value.is_none() {
            return None;
        }

        let value = value.unwrap().parse::<T>();

        return match value {
            Ok(val) => Some(val),
            _ => None,
        };
    }
}

#[derive(Debug)]
pub struct Request(pub AxumRequest<Body>);

impl Request {
    pub fn get_query_params(&self) -> QueryParams {
        let uri = self.0.uri().clone();

        // Parse the query parameters from the URI
        let query_string = uri.query().unwrap_or("");
        let query_string = query_string.split("?").last().unwrap();

        let params: QueryParams =
            serde_urlencoded::from_str(query_string).expect("Failed to parse query string");

        return params;
    }
    pub async fn payload<T>(&mut self) -> T
    where
        T: for<'de> Deserialize<'de>,
    {
        let body = self.0.body_mut();
        let mut json_string = String::new();

        while let Some(next) = body.frame().await {
            let frame = next.unwrap();
            if let Some(chunk) = frame.data_ref() {
                let byte_slice = chunk.as_ref();
                // Attempt to convert the byte slice into a string
                let string = std::str::from_utf8(byte_slice).unwrap();

                json_string = json_string + string;
            }
        }

        return from_str::<T>(&json_string).unwrap();
    }
    pub fn parse_into<T>(&self) -> T
    where
        T: for<'de> Deserialize<'de>,
    {
        let uri = self.0.uri().clone();

        let query_params = uri.query().unwrap_or("");
        let query_params = query_params.split("::").last().unwrap();

        let params: T = serde_qs::from_str(query_params).unwrap();

        return params;
    }
}
