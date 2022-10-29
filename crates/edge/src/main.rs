use std::io::Cursor;

use fastly::http::{Method, StatusCode};
use fastly::{mime, Error, Request, Response};
use rustagram::image;
use rustagram::image::io::Reader;
use rustagram::RustagramFilter;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Pattern match on the path...
    match (req.get_method(), req.get_path()) {
        // If request is to the `/` path...
        (&Method::GET, "/") => Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body(include_str!("index.html"))),
        (&Method::GET, "/app.js") => Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::APPLICATION_JAVASCRIPT)
            .with_body(include_str!("app.js"))),

        (&Method::POST, "/image") => convert_image(req),

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}

pub fn convert_image(mut req: Request) -> Result<Response, Error> {
    let filter_str = req.get_query_parameter("filter").unwrap();
    let filter = filter_str.parse().unwrap();

    if !req.has_body() {
        return Ok(
            Response::from_status(StatusCode::BAD_REQUEST)
                .with_body_text_plain("missing image")
        );
    }

    let body = req.take_body();
    let body = body.into_bytes();

    let img = Reader::new(Cursor::new(body))
        .with_guessed_format()
        .unwrap();

    let img = img.decode().unwrap();

    let img = img.thumbnail(500, 500);
    let out = img.to_rgba8().apply_filter(filter);
    let mut bytes: Vec<u8> = Vec::new();
    out.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;

    Ok(Response::from_status(StatusCode::OK)
        .with_body(bytes)
        .with_content_type(mime::IMAGE_PNG))
}
