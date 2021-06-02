use std::time::Instant;

static OUTPUT: &str = "output/world.bin";

fn main() {

    let start = Instant::now();
    world_builder::compile(firecore_dependencies::ser::deserialize(&std::fs::read("../../../build/data/dex.bin").unwrap()).unwrap(), "world", OUTPUT);
    println!("Completed in {}ms!", start.elapsed().as_millis());

    // match std::fs::read(OUTPUT) {
    //     Ok(bytes) => {
    //         match postcard::from_bytes::<firecore_world_lib::serialized::>(&bytes) {
    //             Ok(world) => {
    //                 println!("Successfully decoded serialized world!");
    //                 for palette in &world.palettes {
    //                     if palette.id == 0 {
    //                         match std::fs::read("world/textures/Palette0B.png") {
    //                             Ok(bytes) => {
    //                                 if palette.bottom.len() == bytes.len() {
    //                                     if palette.bottom == bytes {
    //                                         println!("Palette is equal to file!");
    //                                     }
    //                                 } else {
    //                                     println!("Palette 0 is not equal to file!");
    //                                 }
    //                             }
    //                             Err(err) => {
    //                                 panic!("{}", err);
    //                             }
    //                         }
    //                     }
    //                 }

    //             }
    //             Err(err) => {
    //                 eprintln!("Could not decode serialized world with error {}", err);
    //             }
    //         }
    //     }
    //     Err(err) => {
    //         eprintln!("Could not read output file with error {}", err);
    //     }
    // }
}