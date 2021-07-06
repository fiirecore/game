use std::time::Instant;

static OUTPUT: &str = "output/world.bin";

fn main() {

    let start = Instant::now();
    firecore_world_builder::compile(firecore_dependencies::ser::deserialize(&std::fs::read("../../../build/data/dex.bin").unwrap()).unwrap(), "world", OUTPUT);
    println!("Completed in {}ms!", start.elapsed().as_millis());

}