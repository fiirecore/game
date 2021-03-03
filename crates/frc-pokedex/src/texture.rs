pub enum PokemonTexture {
    Front,
    Back,
    Icon,
}

impl PokemonTexture {
    pub fn path(self) -> &'static str {
        match self {
            PokemonTexture::Front => "front",
            PokemonTexture::Back => "back",
            PokemonTexture::Icon => "icon",
        }
    }
}