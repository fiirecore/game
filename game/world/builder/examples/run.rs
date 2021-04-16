use std::time::Instant;
use firecore_world_lib::serialized::SerializedWorld;

static OUTPUT: &str = "output/world.bin";

fn main() {

    let start = Instant::now();
    world_builder::compile("world/maps", "world/textures", "world/npcs", OUTPUT);
    println!("Completed in {}ms!", start.elapsed().as_millis());

    match std::fs::read(OUTPUT) {
        Ok(bytes) => {
            let result: Result<SerializedWorld, postcard::Error> = postcard::from_bytes(&bytes);
            match result {
                Ok(world) => {
                    println!("Successfully decoded serialized world!");
                    for palette in &world.palettes {
                        if palette.id == 0 {
                            match std::fs::read("world/textures/Palette0B.png") {
                                Ok(bytes) => {
                                    if palette.bottom.len() == bytes.len() {
                                        if palette.bottom == bytes {
                                            println!("Palette is equal to file!");
                                        }
                                    } else {
                                        println!("Palette 0 is not equal to file!");
                                    }
                                }
                                Err(err) => {
                                    panic!("{}", err);
                                }
                            }
                        }
                    }

                }
                Err(err) => {
                    eprintln!("Could not decode serialized world with error {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Could not read output file with error {}", err);
        }
    }
}