use std::time::Instant;

fn main() {
    println!("Starting font serialization");
    let start = Instant::now();
    font_builder::compile("fonts", "output/fonts.bin");
    println!("Finished serializing fonts in {}ms.", start.elapsed().as_millis());
}