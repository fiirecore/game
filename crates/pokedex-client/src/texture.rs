use bevy::{
    prelude::{Assets, Handle, Image, Resource, Deref, DerefMut},
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::BevyDefault,
    },
};
use enum_map::{EnumMap};

use firecore_pokedex_client_data::PokemonOutput;

use crate::pokedex::{
    item::ItemId,
    pokemon::{PokemonId, data::PokemonTexture},
};

use engine::{
    bevy_egui::{
        egui::{self, TextureId, Vec2},
        EguiContext,
    },
    HashMap,
};

#[derive(Default, Resource, Deref, DerefMut)]
pub struct TrainerGroupTextures(HashMap<crate::pokedex::trainer::TrainerGroupId, Handle<Image>>);

#[derive(Resource, Default)]
pub struct PokemonTextures {
    textures: HashMap<PokemonId, EnumMap<PokemonTexture, Handle<Image>>>,
    egui: HashMap<PokemonId, EnumMap<PokemonTexture, TextureId>>,
}

#[derive(Resource, Default)]
pub struct ItemTextures {
    textures: HashMap<ItemId, Handle<Image>>,
    egui: HashMap<ItemId, TextureId>,
}

impl PokemonTextures {
    pub const ICON_SIZE: Vec2 = Vec2::splat(32.0);
    pub const SIZE: Vec2 = Vec2::splat(64.0);

    pub fn new(
        images: &mut Assets<Image>,
        egui: &mut EguiContext,
        pokemon: PokemonOutput,
    ) -> (Self, HashMap<PokemonId, Vec<u8>>) {
        let mut this = Self::default();
        let mut cries = HashMap::default();
        for (id, (textures, cry)) in pokemon {
            this.insert(images, egui, id, textures);
            cries.insert(id, cry);
        }
        (this, cries)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            textures: HashMap::with_capacity(capacity),
            egui: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(
        &mut self,
        images: &mut Assets<Image>,
        egui: &mut EguiContext,
        id: PokemonId,
        bytes: EnumMap<PokemonTexture, Vec<u8>>,
    ) {

        let [front, back, icon] = bytes.into_array();

        let front = images.add(Image::new(
            Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            front,
            TextureFormat::bevy_default(),
        ));
        let back = images.add(Image::new(
            Extent3d {
                width: 64,
                height: 64,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            back,
            TextureFormat::bevy_default(),
        ));
        let icon = images.add(Image::new(
            Extent3d {
                width: 32,
                height: 64,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            icon,
            TextureFormat::bevy_default(),
        ));

        let egui_front = egui.add_image(front.clone());
        let egui_back = egui.add_image(back.clone());
        let egui_icon = egui.add_image(icon.clone());

        self.egui.insert(
            id,
            EnumMap::from_array([egui_front, egui_back, egui_icon]),
        );

        self.textures.insert(
            id,
           EnumMap::from_array([front, back, icon])
        );
    }

    pub fn get(&self, id: &PokemonId, side: PokemonTexture) -> Option<&Handle<Image>> {
        self.textures.get(id).map(|m| &m[side])
    }

    pub fn get_egui(&self, id: &PokemonId, side: PokemonTexture) -> Option<egui::TextureId> {
        self.egui.get(id).map(|textures| textures[side])
    }
}

impl ItemTextures {
    pub const SIZE: Vec2 = Vec2::new(32.0, 32.0);

    pub fn new(
        images: &mut Assets<Image>,
        egui: &mut EguiContext,
        items: HashMap<ItemId, Vec<u8>>,
    ) -> Self {
        let mut this = Self::default();
        for (id, texture) in items {
            this.insert(images, egui, id, texture);
        }
        this
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            textures: HashMap::with_capacity(capacity),
            egui: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert(
        &mut self,
        images: &mut Assets<Image>,
        egui: &mut EguiContext,
        id: ItemId,
        texture: Vec<u8>,
    ) {
        let image = images.add(Image::new(
            Extent3d {
                width: 32,
                height: 32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            texture,
            TextureFormat::bevy_default(),
        ));
        self.egui.insert(id, egui.add_image(image.clone()));
        self.textures.insert(id, image);
    }

    pub fn get(&self, id: &ItemId) -> Option<&Handle<Image>> {
        self.textures.get(id)
    }

    pub fn get_egui(&self, id: &ItemId) -> Option<&egui::TextureId> {
        self.egui.get(id)
    }
}
