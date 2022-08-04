use enum_map::{enum_map, EnumMap};

use firecore_pokedex_engine_core::PokemonOutput;

use crate::pokedex::{
    item::ItemId,
    pokemon::{PokemonId, PokemonTexture},
};

use engine::{
    egui::{self, EguiRegisterTexture},
    graphics::Graphics,
    graphics::Texture,
    utils::HashMap,
};

pub type TrainerGroupTextures = HashMap<crate::TrainerGroupId, Texture>;

#[derive(Default)]
pub struct PokemonTextures {
    textures: HashMap<PokemonId, EnumMap<PokemonTexture, Texture>>,
    egui_icons: HashMap<PokemonId, (egui::TextureId, (f32, f32))>,
    egui_front: HashMap<PokemonId, (egui::TextureId, (f32, f32))>,
}

#[derive(Default)]
pub struct ItemTextures {
    textures: HashMap<ItemId, Texture>,
    egui: HashMap<ItemId, (egui::TextureId, (f32, f32))>,
}

impl PokemonTextures {
    pub fn new(
        gfx: &mut Graphics,
        pokemon: PokemonOutput,
    ) -> Result<(Self, HashMap<PokemonId, Vec<u8>>), String> {
        let mut this = Self::default();
        let mut cries = HashMap::default();
        for (id, (textures, cry)) in pokemon {
            this.insert(gfx, id, textures)?;
            cries.insert(id, cry);
        }
        Ok((this, cries))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            textures: HashMap::with_capacity(capacity),
            egui_icons: HashMap::with_capacity(capacity),
            egui_front: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(
        &mut self,
        gfx: &mut Graphics,
        id: PokemonId,
        textures: EnumMap<PokemonTexture, Vec<u8>>,
    ) -> Result<(), String> {
        self.textures.insert(
            id,
            enum_map! {
                PokemonTexture::Front => {
                    let texture = gfx.create_texture().from_image(&textures[PokemonTexture::Front]).build()?;
                    self.egui_front.insert(id, (gfx.egui_register_texture(&texture), texture.size()));
                    texture
                },
                PokemonTexture::Back => gfx.create_texture().from_image(&textures[PokemonTexture::Back]).build()?,
                PokemonTexture::Icon => {
                    let texture = gfx.create_texture().from_image(&textures[PokemonTexture::Icon]).with_premultiplied_alpha().build()?;
                    self.egui_icons.insert(id, (gfx.egui_register_texture(&texture), texture.size()));
                    texture
                },
            },
        );
        Ok(())
    }

    pub fn get(&self, id: &PokemonId, side: PokemonTexture) -> Option<&Texture> {
        self.textures.get(id).map(|m| &m[side])
    }

    pub fn egui_id(
        &self,
        id: &PokemonId,
        side: PokemonTexture,
    ) -> Option<(egui::TextureId, (f32, f32))> {
        match side {
            PokemonTexture::Front => self.egui_front.get(id).copied(),
            PokemonTexture::Back => None,
            PokemonTexture::Icon => self.egui_icons.get(id).copied(),
        }
    }
}

impl ItemTextures {
    pub fn new(gfx: &mut Graphics, items: HashMap<ItemId, Vec<u8>>) -> Result<Self, String> {
        let mut this = Self::default();
        for (id, texture) in items {
            this.insert(gfx, id, &texture)?;
        }
        Ok(this)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            textures: HashMap::with_capacity(capacity),
            egui: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(
        &mut self,
        gfx: &mut Graphics,
        id: ItemId,
        texture: &[u8],
    ) -> Result<(), String> {
        let texture = gfx
            .create_texture()
            .from_image(&texture)
            .with_premultiplied_alpha()
            .build()?;
        self.egui
            .insert(id, (gfx.egui_register_texture(&texture), texture.size()));
        self.textures.insert(id, texture);
        Ok(())
    }

    pub fn get(&self, id: &ItemId) -> Option<&Texture> {
        self.textures.get(id)
    }

    pub fn egui_id(&self, id: &ItemId) -> Option<&(egui::TextureId, (f32, f32))> {
        self.egui.get(id)
    }
}
