use std::time::Instant;

fn main() {
    println!("Starting font serialization");
    let start = Instant::now();
    let fonts = firecore_font_builder::compile("../../pokemon-assets/fonts");
    println!("Finished serializing fonts in {}ms.", start.elapsed().as_millis());
    println!("{:?}", fonts.iter().map(|f| &f.data.id).collect::<Vec<_>>());
}