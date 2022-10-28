use rustagram::{image, RustagramFilter};

fn main() {
    let mut args = std::env::args().skip(1);
    let input = args.next().expect("INPUT required");
    let filter = args.next().expect("FILTER required");
    let output = args.next().unwrap_or_else(|| "output.jpg".to_string());

    let filter_type = filter.parse().expect("can't parse filter name");
    let img = image::open(input).unwrap();
    let out = img.to_rgba8().apply_filter(filter_type);
    out.save(output).unwrap();
}
