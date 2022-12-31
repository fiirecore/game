pub extern crate firecore_base as engine;
use bevy::prelude::Plugin;
pub use firecore_pokedex_client_data::*;

// #[deprecated(note = "add battle moves to battle-gui crate")]
// pub mod battle_move;

pub mod gui;
pub mod texture;

pub struct PokedexClientPlugin;

impl Plugin for PokedexClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<texture::TrainerGroupTextures>()
            .init_resource::<texture::PokemonTextures>()
            .init_resource::<texture::ItemTextures>();
    }
}

pub mod dex {
    use bevy::prelude::*;
    use firecore_pokedex_client_data::pokedex::{Dex, Identifiable};

    #[derive(Resource, Deref, DerefMut)]
    pub struct BevyDex<I: Identifiable>(pub Dex<I>);
}

pub mod audio {

    use {
        crate::pokedex::pokemon::PokemonId,
        bevy::{
            prelude::{Assets, AudioSource},
            utils::HashMap,
        },
        engine::audio::{AudioSources, SoundId, SoundVariant},
    };

    /// Holds the string "cry"
    pub const CRY_ID: engine::audio::SoundNameId =
        unsafe { engine::audio::SoundNameId::from_bytes_unchecked(7959107u64.to_ne_bytes()) };

    pub fn add_cries(
        sources: &mut Assets<AudioSource>,
        handles: &mut AudioSources,
        cries: HashMap<PokemonId, Vec<u8>>,
    ) {
        for (id, cry) in cries {
            if !cry.is_empty() {
                handles.insert_sound(sources, SoundId(CRY_ID, SoundVariant::Num(id.0 as _)), cry);
            }
        }
    }
}
