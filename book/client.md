# The Core Client Library

Before we can do anything else we'll need to create the core client library that
the GUI calls into. To reduce the amount of state being maintained, each request 
will create a new [`reqwest::Client`] and accept a `Request` object, returning 
some generic `Response`. 

This isn't overly specific to doing FFI, in fact we probably won't write any FFI
bindings or C++ in this chapter. That said, it's still a very important stage
because poor architecture decisions here can often make life hard for you down 
the road. In general, making the interface as small and high level as possible 
will vastly reduce the implementation complexity.

The first thing to do is set up error handling using `error-chain`. I have 
`cargo-edit` installed (`cargo install cargo-edit`), so adding it to my 
`Cargo.toml` is as simple as running

```
$ cargo add error-chain
```

You'll then need to add the corresponding `extern crate` statement to `lib.rs`. 
While you're at it, add also the `reqwest`, `cookie`, `chrono`, `fern`, `log` 
and `libc` crates both to `Cargo.toml` and `lib.rs`, as we are going to use 
them as well afterwards.

```rust
// client/src/lib.rs

extern crate chrono;
extern crate cookie;
#[macro_use]
extern crate error_chain;
extern crate fern;
extern crate libc;
#[macro_use]
extern crate log;
extern crate reqwest;
```

Now create an `errors.rs` module.

```rust
// client/src/errors.rs

error_chain!{
    foreign_links {
        Reqwest(::reqwest::Error);
    }
}
```

First lets create a `Request` object;

```rust
// client/src/request.rs

use cookie::CookieJar;
use reqwest::{self, Method, Url};
use reqwest::header::{Cookie, Headers};


/// A HTTP request.
#[derive(Debug, Clone)]
pub struct Request {
    pub destination: Url,
    pub method: Method,
    pub headers: Headers,
    pub cookies: CookieJar,
    pub body: Option<Vec<u8>>,
}
```

Add a constructor method as used by `request_create()`.

```rust
impl Request {
    pub fn new(destination: Url, method: Method) -> Request {
        let headers = Headers::default();
        let cookies = CookieJar::default();
        let body = None;

        Request {
            destination,
            method,
            headers,
            cookies,
            body,
        }
    }
}
```

We'll also need to be able to convert our `Request` into a `reqwest::Reqwest` 
before we can send it so lets add a helper method for that.

```rust
impl Request {
    pub(crate) fn to_reqwest(&self) -> reqwest::Request {
        let mut r = reqwest::Request::new(self.method.clone(), self.destination.clone());

        r.headers_mut().extend(self.headers.iter());

        let mut cookie_header = Cookie::new();

        for cookie in self.cookies.iter() {
            cookie_header.set(cookie.name().to_owned(), cookie.value().to_owned());
        }
        r.headers_mut().set(cookie_header);

        r
    }
}
```

We also want to create our own vastly simplified `Response` so it can be 
accessed by the C++ GUI, it gets a helper method too.

```rust
// client/src/response.rs

use std::io::Read;
use reqwest::{self, StatusCode};
use reqwest::header::Headers;

use errors::*;


#[derive(Debug, Clone)]
pub struct Response {
    pub headers: Headers,
    pub body: Vec<u8>,
    pub status: StatusCode,
}

impl Response {
    pub(crate) fn from_reqwest(original: reqwest::Response) -> Result<Response> {
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
```

> **Note:** everything in a `Request` and `Response` has been marked as
> public because it's designed to be a dumb container of everything necessary
> to build a request.

To help out with debugging the FFI bindings later on we'll add in logging via 
the `log` and `fern` crates. In a GUI program it's often not feasible to add in
`println!()` statements and logging is a great substitute. Having a log file is
also quite useful if you want to look back over a session to see what requests 
were sent and what the server responded with.

```rust
// client/src/utils.rs

use std::sync::{Once, ONCE_INIT};
use fern;
use log::LogLevelFilter;
use chrono::Local;

use errors::*;

/// Initialize the global logger and log to `rest_client.log`.
///
/// Note that this is an idempotent function, so you can call it as many
/// times as you want and logging will only be initialized the first time.
#[no_mangle]
pub extern "C" fn initialize_logging() {
    static INITIALIZE: Once = ONCE_INIT;
    INITIALIZE.call_once(|| {
        fern::Dispatch::new()
            .format(|out, message, record| {
                let loc = record.location();

                out.finish(format_args!(
                    "{} {:7} ({}#{}): {}{}",
                    Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.level(),
                    loc.module_path(),
                    loc.line(),
                    message,
                    if cfg!(windows) { "\r" } else { "" }
                ))
            })
            .level(LogLevelFilter::Debug)
            .chain(fern::log_file("rest_client.log").unwrap())
            .apply()
            .unwrap();
    });
}
```

Initializing logging will usually panic if you call it multiple times, therefore
we're using `std::sync::Once` so that `initialize_logging()` will only ever set
up `fern` once. 

The logging initializing itself looks pretty gnarly, although that's mainly 
because of the large `format_args!()` statement and having to make sure we add
in line endings appropriately.

We'll also add a `backtrace()` helper to the `utils` module. This just takes an
`Error` and iterates through it, logging a nice stack trace.

```rust
// client/src/utils.rs

/// Log an error and each successive thing which caused it.
pub fn backtrace(e: &Error) {
    error!("Error: {}", e);

    for cause in e.iter().skip(1) {
        warn!("\tCaused By: {}", cause);
    }
}
```

We'll also create a generic `send_request()` function which takes a `Request` 
object and sends it, retrieving the resulting `Response`. Thanks to our two 
helper functions the implementation is essentially trivial (modulo some logging
stuff).

```rust
// client/src/lib.rs

use reqwest::Client;
pub use request::Request;
pub use response::Response;
use errors::*;


/// Send a `Request`.
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

        trace!("{:#?}", req);
    }

    let client = Client::builder()
        .build()
        .chain_err(|| "The native TLS backend couldn't be initialized")?;

    client
        .execute(req.to_reqwest())
        .chain_err(|| "The request failed")
        .and_then(|r| Response::from_reqwest(r))
}
```

You'll notice that `chain_err()` has been used whenever anything may fail. This
allows us to give the user some sort of stack trace of errors and what caused 
them, providing a single high level error message (i.e. "The native TLS backend 
couldn't be initialized"), while still retaining the low level context if they 
want to drill down and find out *exactly* what went wrong.

This method of error handling ties in quite nicely with the `backtrace()` helper
defined earlier. As you'll see later on, they can prove invaluable for 
debugging issues when passing things between languages.


Register the four new modules in `lib.rs`.

```rust
// client/src/lib.rs

pub mod errors;
pub mod utils;
mod request;
mod response;
```

Now we've got something to work with, we can start writing some FFI bindings.


[`reqwest::Client`]: https://docs.rs/reqwest/0.8.0/reqwest/struct.Client.html
[`HeaderMap`]: https://docs.rs/reqwest/0.8.0/reqwest/struct.Client.html
[`CookieJar`]: https://docs.rs/cookie/0.10.1/cookie/