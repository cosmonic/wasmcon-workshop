wit_bindgen::generate!({
    world: "hello-cosmo",
    exports: {
        "wasi:http/incoming-handler": HelloCosmo
    }
});

use wasi::{
    http::http_types::{Method, ResponseOutparam},
    keyvalue::types::open_bucket,
};

mod http_helper;
mod kv;
mod ui;
use http_helper::*;
use ui::get_static_asset;

use crate::{
    exports::wasi::http::incoming_handler::{Guest, IncomingRequest},
    kv::increment_counter,
};

// NOTE: custom buckets are not yet supported
const BUCKET: &str = "";

/// Implementation struct for the 'hello-cosmo' world (see: wit/hello-cosmo.wit)
struct HelloCosmo;

/// Implementation of the WIT-driven incoming-handler interface for our implementation struct
impl Guest for HelloCosmo {
    fn handle(request: IncomingRequest, response: ResponseOutparam) {
        let (method, request_path) = method_and_path(request);
        let trimmed_path: Vec<&str> = request_path.path().trim_matches('/').split('/').collect();

        // Generate an outgoing request
        match (method, trimmed_path.as_slice()) {
            // GET /api/counter
            //
            // Retrieve value of the counter
            (Method::Get, ["api", "counter"]) => {
                // Retrieve the bucket
                // Retrieve bucket or return early with error
                let bucket = if let Ok(v) = open_bucket(BUCKET) {
                    v
                } else {
                    write_http_response(
                        response,
                        500,
                        &content_type_json(),
                        ApiResponse::error("failed to retrieve bucket").into_vec(),
                    );
                    return;
                };

                // Increment the counter
                let updated_value = match kv::increment_counter(bucket, &String::from("default"), 1)
                {
                    Ok(v) => v,
                    Err(_) => {
                        write_http_response(
                            response,
                            500,
                            &content_type_json(),
                            ApiResponse::error("failed to increment default counter").into_vec(),
                        );
                        return;
                    }
                };

                // Build & write the response the response
                eprintln!("[success] successfully incremented default counter");
                write_http_response(
                    response,
                    200,
                    &content_type_json(),
                    ApiResponse::success(updated_value).into_vec(),
                )
            }

            // GET /api/counter/:counter_name
            //
            // Update a counter
            (Method::Get, ["api", "counter", counter]) => {
                // Retrieve bucket or return early with error
                let bucket = if let Ok(v) = open_bucket(BUCKET) {
                    v
                } else {
                    write_http_response(
                        response,
                        500,
                        &content_type_json(),
                        ApiResponse::error("failed to retreive bucket").into_vec(),
                    );
                    return;
                };

                // Increment the counter
                let updated_value = match increment_counter(bucket, &counter.to_string(), 1) {
                    Ok(v) => v,
                    Err(e) => {
                        write_http_response(
                            response,
                            500,
                            &content_type_json(),
                            ApiResponse::error(format!("{e}")).into_vec(),
                        );
                        return;
                    }
                };

                // Write out HTTP response
                eprintln!("[success] successfully incremented [{counter}] counter");
                write_http_response(
                    response,
                    200,
                    &content_type_json(),
                    ApiResponse::success(updated_value).into_vec(),
                );
            }

            // GET /*
            //
            // Any other GET request is interpreted as a static asset request for the UI
            (Method::Get, asset_path) => {
                let path = asset_path.join("/");
                match get_static_asset(&path) {
                    Ok((content_type, bytes)) => write_http_response(
                        response,
                        200,
                        &[("Content-Type".into(), content_type.into_bytes())],
                        bytes,
                    ),
                    Err(err) => {
                        eprintln!("[error] failed to retreive static asset @ [{path}]: {err:?}");
                        write_http_response(response, 404, &Vec::new(), "not found");
                    }
                };
            }

            // ???
            //
            // All other method + path combinations are unrecognized operations
            _ => write_http_response(
                response,
                400,
                &content_type_json(),
                ApiResponse::error("unrecognized operation").into_vec(),
            ),
        };
    }
}
