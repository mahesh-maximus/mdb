
use micro_http::MediaType;
use crate::{request::{Body, StatusCode, Response, Version, Request}};
use crate::common::get_content;

pub(crate) fn parse_signin(_request: &Request) -> Response  {
    println!("request.parse_signin fn");
    let mut response = Response::new(Version::Http11, StatusCode::OK);
    response.set_content_type(MediaType::TextHtml);
    response.set_body(Body::new(get_content(format!("{}{}", "/mdb/frontend/", "signin.html"))));
    response
}