use std::fs::read_dir;
use std::path::Path;

fn main() {
    build_icon_directory("../Tools/Original Game Sources/Pokemon FireRed/graphics/pokemon", "../Subprojects/PokedexGenJava/pokedex/textures/icon");
}

fn build_icon_directory(pret_input: &str, output_dir: &str) {
    let path = Path::new(output_dir);
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
    for entry in read_dir(pret_input).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();
        if path.is_dir() {
            let input = path.join("icon.png");
            let output = Path::new(output_dir).join(name + ".png");
            println!("Input: {:?}", input);
            println!("Output: {:?}", output);
            if let Err(err) = std::fs::copy(&input, &output) {
                eprintln!("Could not copy {:?} to {:?} with error {}", input, output, err);
            }
        }        
    }
    std::fs::copy(Path::new(pret_input).join("unown").join("e").join("icon.png"), Path::new(output_dir).join("unown.png")).unwrap();
}