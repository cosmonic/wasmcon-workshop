wit_bindgen::generate!({
    world: "hello-cosmo",
    exports: {
        "wasi:http/incoming-handler": HelloCosmo
    }
});

use wasi::{
    // imports from the wit module
    http::http_types::{ResponseOutparam, Method},
    logging::logging::{log, Level},
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
        log(Level::Info, "rust-component", "beginning handle");
        let (method, request_path) = method_and_path(request);
        log(Level::Info, "rust-component", request_path.path());

        // Generate an outgoing request
        match (method, request_path.path()) {
            (Method::Get, "/") => {
                write_http_response(response, 200, &content_type_json(), "{\"hello\":\"cosmo\"}")
            }
            _ => {
                write_http_response(response, 404, &content_type_json(), "not found")
            }
        }
    }
}
