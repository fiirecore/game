#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PokemonTexture {
    Front,
    Back,
    Icon,
}

impl PokemonTexture {
    pub const fn path(self) -> &'static str {
        match self {
            PokemonTexture::Front => "front",
            PokemonTexture::Back => "back",
            PokemonTexture::Icon => "icon",
        }
    }
}