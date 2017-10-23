//! The business logic for a REST client.

extern crate chrono;
extern crate cookie;
#[macro_use]
extern crate error_chain;
extern crate fern;
#[macro_use]
extern crate log;
extern crate reqwest;

pub mod errors;
pub mod utils;
mod methods;

use std::io::Read;
use cookie::CookieJar;
use reqwest::{Client, Method, StatusCode, Url};
use reqwest::header::{Cookie, Headers};

use errors::*;


#[derive(Debug, Clone)]
pub struct Request {
    pub destination: Url,
    pub method: Method,
    pub headers: Headers,
    pub cookies: CookieJar,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub headers: Headers,
    pub body: Vec<u8>,
    pub status: StatusCode,
}

impl Response {
    pub fn from_reqwest(original: reqwest::Response) -> Result<Response> {
        let mut original = original.error_for_status()?;
        let headers = original.headers().clone();
        let status = original.status();

        let mut body = Vec::new();
        original
            .read_to_end(&mut body)
            .chain_err(|| "Unable to read the response body")?;

        Ok(Response {
            status,
            body,
            headers,
        })
    }
}
