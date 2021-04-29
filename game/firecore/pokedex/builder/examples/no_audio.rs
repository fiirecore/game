use std::time::Instant;

fn main() {
    println!("Building dex...");
    let start = Instant::now();
    dex_builder::compile("../../pokemon-game/assets/pokedex/pokemon", "../../pokemon-game/assets/pokedex/moves", "../../pokemon-game/assets/pokedex/items", "builder/output/dex.bin", false);
    println!("Finished in {}ms!", start.elapsed().as_millis());
}