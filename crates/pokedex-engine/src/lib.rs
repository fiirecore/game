pub extern crate firecore_base as engine;
pub use firecore_pokedex_engine_core::*;
// pub use battle::pokedex;

// #[deprecated(note = "add battle moves to battle-gui crate")]
// pub mod battle_move;

pub mod gui;
pub mod texture;

/// Holds the string "cry"
pub const CRY_ID: tinystr::TinyStr8 =
    unsafe { tinystr::TinyStr8::from_bytes_unchecked(7959107u64.to_ne_bytes()) };

#[cfg(feature = "audio")]
pub fn add_cries(app: &mut engine::App, plugins: &mut engine::Plugins, cries: crate::engine::HashMap<pokedex::pokemon::PokemonId, Vec<u8>>) {
    for (id, cry) in cries {
        if !cry.is_empty() {
            engine::sound::add_sound(
                app,
                plugins,
                crate::CRY_ID,
                engine::sound::SoundVariant::Num(id as _),
                cry,
            );
        }
    }
}
