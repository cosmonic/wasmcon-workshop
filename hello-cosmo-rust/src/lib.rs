wit_bindgen::generate!({
    world: "hello-cosmo",
    exports: {
        "wasi:http/incoming-handler": HelloCosmo
    }
});

use wasi::{
    // imports from the wit module
    http::http_types::{ResponseOutparam},
};

mod http_helper;
use http_helper::*;

use crate::{
    // exports from the wit module
    exports::wasi::http::incoming_handler::{Guest, IncomingRequest},
};

/// Implementation struct for the 'hello-cosmo' world (see: wit/hello-cosmo.wit)
struct HelloCosmo;

/// Implementation of the WIT-driven incoming-handler interface for our implementation struct
impl Guest for HelloCosmo {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {

    }
}
