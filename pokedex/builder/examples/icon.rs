use std::fs::read_dir;
use std::path::Path;

const FILENAME: &str = "icon.png";

fn main() {
    build_icon_directory("../Tools/Original Game Sources/Pokemon FireRed/graphics/pokemon", "assets/pokedex/pokemon");
}

fn build_icon_directory(pret_input: &str, output_dir: &str) {
    let output = Path::new(output_dir);
    if !output.exists() {
        std::fs::create_dir_all(&output).unwrap();
    }
    for entry in read_dir(pret_input).unwrap() {
        let entry = entry.unwrap();
        let mut name = entry.file_name().to_string_lossy().to_string();
        name[..1].make_ascii_uppercase();
        underscore_to_dash(&mut name);
        let input = entry.path();
        if input.is_dir() {
            let input = input.join(FILENAME);
            let output = output.join(name).join(FILENAME);
            if let Err(err) = std::fs::copy(&input, &output) {
                eprintln!("Could not copy {:?} to {:?} with error {}", input, output, err);
            }
        }        
    }
    std::fs::copy(Path::new(pret_input).join("unown").join("e").join(FILENAME), output.join("Unown").join(FILENAME)).unwrap();
}

fn underscore_to_dash(string: &mut String) {
    if let Some(pos) = string.find('_') {
        // string[pos..pos+1] = "-";
        string.replace_range(pos..pos+1, "-");
    }
}