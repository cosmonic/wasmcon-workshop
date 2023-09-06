wit_bindgen::generate!({
    world: "hello-cosmo",
    exports: {
        "wasi:http/incoming-handler": HelloCosmo
    }
});

use wasi::{
    // imports from the wit module
    http::http_types::{Method, ResponseOutparam},
    keyvalue::types::open_bucket,
    logging::logging::{log, Level},
};

mod http_helper;
mod kv;
use http_helper::*;

use crate::exports::wasi::http::incoming_handler::{Guest, IncomingRequest};

/// Implementation struct for the 'hello-cosmo' world (see: wit/hello-cosmo.wit)
struct HelloCosmo;

fn increment(key: String) -> Result<i32, String> {
    log(Level::Info, "rust-component", "incrementing counter");
    // Retrieve the bucket
    // Retrieve bucket or return early with error
    let bucket = if let Ok(value) = open_bucket("") {
        value
    } else {
        return Err("failed to retrieve bucket".to_string());
    };

    // Increment the counter
    let updated_value =
        match kv::increment_counter(bucket, &key, 1) {
            Ok(value) => {
                log(
                    Level::Info,
                    "rust-component",
                    format!("new value: {value}").as_str(),
                );
                return Ok(value);
            }
            Err(_) => Err("failed to increment default counter".to_string()),
        };

    // Build & write the response the response
    return updated_value;
}

/// Implementation of the WIT-driven incoming-handler interface for our implementation struct
impl Guest for HelloCosmo {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        log(Level::Info, "rust-component", "beginning handle");
        let (method, request_path) = method_and_path(request);
        log(Level::Info, "rust-component", request_path.path());

        let trimmed_path: Vec<&str> = request_path.path().trim_matches('/').split('/').collect();
        // Generate an outgoing request
        match (method, trimmed_path.as_slice()) {
            (Method::Get, [""]) => {
                write_http_response(response, 200, &content_type_json(), "{\"hello\":\"cosmo\"}")
            },
            (Method::Get, ["api", "counter"]) => {
                match increment("default".to_string()) {
                    Ok(value) => write_http_response(
                        response,
                        200,
                        &content_type_json(),
                        ApiResponse::success(value).into_vec(),
                    ),
                    Err(message) => {
                        write_http_response(
                            response,
                            500,
                            &content_type_json(),
                            ApiResponse::error(message).into_vec(),
                        );
                        return;
                    }
                };
            },
            (Method::Get, ["api", "counter", key]) => {
                match increment(key.to_string()) {
                    Ok(value) => write_http_response(
                        response,
                        200,
                        &content_type_json(),
                        ApiResponse::success(value).into_vec(),
                    ),
                    Err(message) => {
                        write_http_response(
                            response,
                            500,
                            &content_type_json(),
                            ApiResponse::error(message).into_vec(),
                        );
                        return;
                    }
                };
            },
            _ => {
                write_http_response(response, 404, &content_type_json(), "not found")
            }
        }
    }
}
