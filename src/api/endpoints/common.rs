// use this file for util functions
// maybe when we have more function we could refactore them further

use reqwest::RequestBuilder;

pub trait Endpoint {
    fn endpoint_url(workspace_id: &str) -> String;
}

pub fn add_auth_header(request: RequestBuilder, api_key: &str) -> RequestBuilder {
    request.header("Authorization", format!("Bearer {}", api_key))
}
