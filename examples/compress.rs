use std::time::Instant;
fn main() {
    let now = Instant::now();
    #[cfg(feature = "compress")]
    sevenz_rust::compress_to_path("examples/data/sample", "examples/data/sample.7z")
        .expect("compress ok");
    println!("compress done : {:?}", now.elapsed());
}
