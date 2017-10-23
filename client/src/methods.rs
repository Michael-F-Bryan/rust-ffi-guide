use std::io::Read;
use cookie::CookieJar;
use reqwest::{Client, Method, StatusCode, Url};
use reqwest::header::{Cookie, Headers};

use {Request, Response};
use errors::*;


/// Perform a single `GET` request.
pub fn get(req: Request) -> Result<Response> {
    info!("Sending a GET request to {}", req.destination);
    if log_enabled!(::log::LogLevel::Debug) {
        debug!("Sending {} Headers", req.headers.len());
        for header in req.headers.iter() {
            debug!("\t{}: {}", header.name(), header.value_string());
        }
    }

    let client = Client::builder()
        .build()
        .chain_err(|| "The native TLS backend couldn't be initialized")?;

    let mut rb = client.get(req.destination);

    rb.headers(req.headers);
    let mut cookie_header = Cookie::new();

    for cookie in req.cookies.iter() {
        let name = cookie.name().to_owned();
        let value = cookie.value().to_owned();

        debug!("\t{} = {}", name, value);
        cookie_header.set(name, value);
    }
    rb.header(cookie_header);

    rb.send()
        .chain_err(|| "The request failed")
        .and_then(|r| Response::from_reqwest(r))
}
