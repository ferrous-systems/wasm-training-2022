use std::process;

use clap::clap_app;
use rustagram::{image, RustagramFilter};

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Jan-Erik Rediger <janerik@fnordig.de>")
        (about: "Apply instagram filters to you photos")
        (@arg OUTPUT: -o --out "Output file name")
        (@arg INPUT: +required "Path to the input image file")
        (@arg FILTER: +required "Filter name")
    )
    .get_matches();

    let output = matches.value_of("OUTPUT").unwrap_or("output.jpg");
    let input = matches.value_of("INPUT").unwrap();
    let filter = matches.value_of("FILTER").unwrap();

    let filter_type = match filter.parse() {
        Ok(item) => item,
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };

    let img = image::open(input).unwrap();
    let out = img.to_rgba8().apply_filter(filter_type);
    out.save(output).unwrap();
}
