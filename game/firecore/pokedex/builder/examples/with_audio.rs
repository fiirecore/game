use std::time::Instant;

fn main() {
    println!("Building dex...");
    let start = Instant::now();
    dex_builder::compile("pokedex/pokemon", "pokedex/moves", "pokedex/items", "output/dex.bin", true);
    println!("Finished in {}ms!", start.elapsed().as_millis());
}