use std::fs;
use micro_http::MediaType;
use crate::request::{Body, StatusCode, Response, Version, Request};

pub(crate) fn parse_not_found(_request: &Request) -> Response  {
    println!("request.parse_Not_found fn");
    let mut response = Response::new(Version::Http11, StatusCode::OK);
    let file_name = format!("{}{}", "/mdb/frontend/", "404.html");
    println!("Response filename : {}", file_name);
    let content = fs::read_to_string(file_name).unwrap();
    let response_body = content;

    response.set_content_type(MediaType::TextHtml);
    response.set_body(Body::new(response_body));
    response
}