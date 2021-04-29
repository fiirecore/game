use std::fs::copy;
use std::path::Path;

fn main() {

    let pokemon_dir = "pokedex/entries";
    let textures_dir = "pokedex/textures";

    let base = Path::new(textures_dir);
    let icon = base.join("icon");
    let base = base.join("normal");
    let new = Path::new("pokedex/pokemon");

    for entry in std::fs::read_dir(pokemon_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_stem().unwrap();

        // Create new folder

        let folder = new.join(name);

        if !folder.exists() {
            std::fs::create_dir_all(&folder).unwrap();
        }

        let image = name.to_string_lossy().to_ascii_lowercase() + ".png";

        copy(&path, folder.join(entry.file_name())).unwrap(); // Copy pokemon entry

        copy(base.join("front").join(&image), folder.join("normal_front.png")).unwrap(); // copy front texture
        copy(base.join("back").join(&image), folder.join("normal_back.png")).unwrap(); // copy back texture
        
        let mut icon_path = icon.join(&image);
        if !icon_path.exists() {
            if image.starts_with("deo") {
                icon_path = icon.join("deoxys.png");
            } else {
                icon_path = icon.join(image.replace("-", "_"));
            }
        };

        copy(icon_path, folder.join("icon.png")).unwrap();
        
    }

}