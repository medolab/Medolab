use candid::{CandidType, Decode, Encode};
use ic_cdk_macros::query;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize)]
struct HttpRequest {
    method: String,
    url: String,
    headers: Vec<HttpHeader>,
    body: Vec<u8>,
}

#[derive(CandidType, Clone, Deserialize)]
struct HttpResponse {
    status: u16,
    headers: Vec<HttpHeader>,
    body: Vec<u8>,
}

#[derive(CandidType, Clone, Deserialize)]
struct HttpHeader {
    name: String,
    value: String,
}

#[query]
fn http_request(request: HttpRequest) -> HttpResponse {
    // For local development, the full URL will include canister IDs
    // We'll just check if the base path is "/hello" regardless of query parameters
    let path = request.url.split('?').next().unwrap_or("");
    
    if path == "/" || path == "/hello" {
        HttpResponse {
            status: 200,
            headers: vec![
                HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "text/plain".to_string(),
                }
            ],
            body: "Hello, world!".as_bytes().to_vec(),
        }
    } else {
        HttpResponse {
            status: 404,
            headers: vec![
                HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "text/plain".to_string(),
                }
            ],
            body: "404 Not Found".as_bytes().to_vec(),
        }
    }
}