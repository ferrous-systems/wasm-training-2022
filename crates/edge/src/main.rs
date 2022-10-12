//! Default Compute@Edge template program.

use std::io::Cursor;

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};
use rustagram::image;
use rustagram::image::io::Reader as ImageReader;
use rustagram::{FilterType, RustagramFilter};

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Filter request methods...
    match req.get_method() {
        // Allow GET, POST and HEAD requests.
        &Method::GET | &Method::POST | &Method::HEAD => (),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, POST, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    // Pattern match on the path...
    match req.get_path() {
        // If request is to the `/` path...
        "/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("index.html")))
        }
        "/app.js" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::APPLICATION_JAVASCRIPT)
                .with_body(include_str!("app.js")))
        }

        "/image" => convert_image(req),

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}

fn bad_request(msg: &str) -> Result<Response, Error> {
    Ok(Response::from_status(StatusCode::BAD_REQUEST).with_body_text_plain(msg))
}

pub fn convert_image(mut req: Request) -> Result<Response, Error> {
    let filter = match req.get_query_parameter("filter") {
        Some(filter) => filter,
        None => return bad_request("missing filter"),
    };
    let filter: FilterType = match filter.parse() {
        Ok(filter) => filter,
        Err(_) => return bad_request("invalid filter"),
    };

    if !req.has_body() {
        return bad_request("missing image");
    }

    let body = req.take_body();
    let body = body.into_bytes();

    let img = match ImageReader::new(Cursor::new(body)).with_guessed_format() {
        Ok(img) => img,
        Err(_) => return bad_request("not an image"),
    };

    let img = match img.decode() {
        Ok(img) => img,
        Err(_) => return bad_request("not an image"),
    };

    let img = img.thumbnail(500, 500);
    let out = img.to_rgba8().apply_filter(filter);
    let mut bytes: Vec<u8> = Vec::new();
    out.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
        .unwrap();

    Ok(Response::from_status(StatusCode::OK)
        .with_body(bytes)
        .with_content_type(mime::IMAGE_PNG))
}
