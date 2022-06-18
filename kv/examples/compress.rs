use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;

fn main() {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(b"Hello World").unwrap();
    println!("{:?}", e.finish().unwrap());
}
