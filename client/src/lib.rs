//! The business logic for a REST client.

extern crate chrono;
extern crate libloading;
extern crate cookie;
#[macro_use]
extern crate error_chain;
extern crate fern;
extern crate libc;
#[macro_use]
extern crate log;
extern crate reqwest;

mod plugins;
pub mod errors;
pub mod utils;
pub mod ffi;
mod request;
mod response;

pub use request::Request;
pub use response::Response;
pub use plugins::{PluginManager, Plugin};

use reqwest::Client;
use errors::*;


/// Perform a single `GET` request.
pub fn send_request(req: &Request) -> Result<Response> {
    info!("Sending a GET request to {}", req.destination);
    if log_enabled!(::log::LogLevel::Debug) {
        debug!("Sending {} Headers", req.headers.len());
        for header in req.headers.iter() {
            debug!("\t{}: {}", header.name(), header.value_string());
        }
        for cookie in req.cookies.iter() {
            debug!("\t{} = {}", cookie.name(), cookie.value());
        }
    }

    let client = Client::builder()
        .build()
        .chain_err(|| "The native TLS backend couldn't be initialized")?;

    client
        .execute(req.to_reqwest())
        .chain_err(|| "The request failed")
        .and_then(|r| Response::from_reqwest(r))
}
