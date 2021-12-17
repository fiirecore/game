use std::time::Instant;

fn main() {

    println!("{}", ron::to_string(&firecore_world::map::chunk::Connection(firecore_world::positions::Location { map: None, index: tinystr::tinystr16!("celadon") }, 8)).unwrap());

    let start = Instant::now();
    let _ = firecore_world_builder::compile("assets/world");
    println!("Completed in {}ms!", start.elapsed().as_millis());

}