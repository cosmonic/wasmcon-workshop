use http::uri::PathAndQuery;
use serde::Serialize;

use crate::exports::wasi::http::incoming_handler::IncomingRequest;
use crate::wasi::{
    http::http_types::{
        finish_outgoing_stream, incoming_request_method, incoming_request_path_with_query,
        new_fields, new_outgoing_response, outgoing_response_write, set_response_outparam, Method,
        ResponseOutparam,
    },
    io::streams::write,
};

/// Helper for writing a HTTP response out, using WIT-driven (WASI) interfaces
pub fn write_http_response(
    response_outparam: ResponseOutparam,
    status_code: u16,
    headers: &[(String, Vec<u8>)],
    body: impl AsRef<[u8]>,
) {
    // Add headers
    let headers = new_fields(headers);

    // Create new outgoing response and related stream
    let outgoing_response =
        new_outgoing_response(status_code, headers).expect("failed to create response");
    let outgoing_stream =
        outgoing_response_write(outgoing_response).expect("failed to write outgoing response");

    // Write out repsonse body to outgoing straem
    write(outgoing_stream, body.as_ref()).expect("failed to write output to stream");
    finish_outgoing_stream(outgoing_stream);

    // Set the response on the param
    set_response_outparam(response_outparam, Ok(outgoing_response))
        .expect("failed to set response");
}

pub fn method_and_path(request: IncomingRequest) -> (Method, PathAndQuery) {
    // Decipher method
    let method = incoming_request_method(request);

    // Get path of request, then trim and split
    let request_path = PathAndQuery::from_maybe_shared(
        incoming_request_path_with_query(request)
            .expect("failed to retrieve path and query from request"),
    )
    .expect("failed to parse path & query");
    (method, request_path)
}

/// Helper that returns content type of a json response
pub fn content_type_json() -> [(String, Vec<u8>); 1] {
    [("Content-Type".into(), "application/json".into())]
}

/// The response that is sent by the API after an operation
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ApiResponse {
    Error { error: String },
    Success { counter: i32 },
}

impl ApiResponse {
    /// Generate an error response
    pub fn error(error: impl AsRef<str>) -> Self {
        ApiResponse::Error {
            error: error.as_ref().to_string(),
        }
    }

    /// Generate an error response
    pub fn success(counter: i32) -> Self {
        ApiResponse::Success { counter }
    }

    /// Convert the ApiResponse into a bytes
    pub fn into_vec(self) -> Vec<u8> {
        serde_json::to_vec(&self).expect("failed to serialize API response")
    }
}
