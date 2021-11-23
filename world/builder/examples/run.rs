use std::time::Instant;

fn main() {

    let start = Instant::now();
    let _ = firecore_world_builder::compile("world");
    println!("Completed in {}ms!", start.elapsed().as_millis());

}