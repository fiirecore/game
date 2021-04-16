use std::time::Instant;

use firecore_util::Coordinate;
use firecore_util::Direction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let world: firecore_world_lib::serialized::SerializedWorld = postcard::from_bytes(&std::fs::read("output/world.bin")?)?;

    let set = "pallet_houses".parse().unwrap();
    let map = "oak_lab".parse().unwrap();

    let map = world.manager.map_set_manager.map_sets.get(&set).unwrap().maps.get(&map).unwrap();

    let start = Coordinate::new(0, 2).position(Direction::Right);
    let end = Coordinate::new(0xC, 0x9);

    let now = Instant::now();

    if let Some(path) = firecore_world_lib::character::movement::astar::pathfind(start, end, map) {
        println!("Found path in {} microseconds", now.elapsed().as_micros());
        for coordinate in path {
            println!("{:?}", coordinate);
        }
    } else {
        eprintln!("Could not find a path to specified location!");
    }

    Ok(())

}