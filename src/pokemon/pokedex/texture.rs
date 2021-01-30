pub fn pokemon_texture(name: &str, side: Side) -> crate::util::texture::Texture {
    let path = std::path::PathBuf::from(super::DEX_DIR.join("textures/normal")).join(side.path()).join(name.to_lowercase() + ".png");
    match crate::io::ASSET_DIR.get_file(&path) {
        Some(file) => {
            return crate::util::texture::byte_texture(file.contents());
        }
        None => {
            macroquad::prelude::warn!("Could not find pokemon texture at {:?}", &path);
            return crate::util::texture::debug_texture();
        }
    }
}

pub enum Side {
    Front,
    Back,
}

impl Side {
    fn path(self) -> &'static str {
        match self {
            Side::Front => "front",
            Side::Back => "back",
        }
    }
}