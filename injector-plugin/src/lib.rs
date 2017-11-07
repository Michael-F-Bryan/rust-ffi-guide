#[macro_use]
extern crate log;
#[macro_use]
extern crate client;

use client::{Request, Response, Plugin};


#[derive(Debug, Default)]
pub struct Injector;

impl Plugin for Injector {
    fn name(&self) -> &'static str  {
        "Header Injector"
    }

    fn on_plugin_load(&self) {
        info!("Injector loaded");
    }

    fn on_plugin_unload(&self) {
        info!("Injector unloaded");
    }

    fn pre_send(&self, req: &mut Request) {
        req.headers.set_raw("some-dodgy-header", "true");
        debug!("Injected header into Request, {:?}", req);
    }

    fn post_receive(&self, res: &mut Response) {
        debug!("Received Response, {:?}", res);
        res.headers.remove_raw("some-dodgy-header");
    }
}

declare_plugin!(Injector, Injector::default);