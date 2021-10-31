use std::time::Instant;

static OUTPUT: &str = "output/world.bin";

fn main() {

    let start = Instant::now();
    firecore_world_builder::compile("world", OUTPUT);
    println!("Completed in {}ms!", start.elapsed().as_millis());

}