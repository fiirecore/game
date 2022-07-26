use engine::graphics::Texture;

use crate::engine::{
    notan::app::App,
    notan::prelude::{Graphics, Plugin, Plugins},
    sound::SoundVariant,
};

use firecore_pokedex_engine_builder::SerializedPokedexEngine;

use crate::texture::{ItemTextures, PokemonTextures, TrainerGroupTextures};

pub struct PokedexClientData {
    pub health_bar: Texture,
    // pub bag_background: Texture,
    // pub party: PokedexPartyData,
    pub pokemon_textures: PokemonTextures,
    pub item_textures: ItemTextures,
    pub trainer_group_textures: TrainerGroupTextures,
}

impl Plugin for PokedexClientData {}

// pub struct PokedexPartyData {
//     pub background: Texture,
//     pub ball: Texture,
//     pub select: Texture,
//     pub summary: PokedexSummaryData,
// }

// pub struct PokedexSummaryData {
//     pub pages: [Texture; 3],
//     pub background: Texture,
// }

impl PokedexClientData {
    pub fn build(
        app: &mut App,
        plugins: &mut Plugins,
        gfx: &mut Graphics,
        data: SerializedPokedexEngine,
    ) -> Result<Self, String> {
        let mut pokemon_textures = PokemonTextures::with_capacity(data.pokemon.len());

        for (id, (textures, cry)) in data.pokemon {
            if let Err(err) = pokemon_textures.insert(gfx, id, textures) {
                engine::log::warn!("Cannot add pokemon texture for {} with error {}", id, err);
            }

            #[cfg(feature = "audio")]
            if !cry.is_empty() {
                engine::sound::add_sound(
                    app,
                    plugins,
                    crate::CRY_ID,
                    SoundVariant::Num(id as _),
                    cry,
                );
            }
        }

        let mut item_textures = ItemTextures::with_capacity(data.items.len());

        for (id, texture) in data.items.into_iter() {
            item_textures.insert(gfx, id, texture)?;
        }

        let mut trainer_group_textures =
            TrainerGroupTextures::with_capacity(data.trainer_groups.len());

        for (id, texture) in data.trainer_groups {
            trainer_group_textures.insert(id, gfx.create_texture().from_image(&texture).build()?);
        }

        Ok(Self {
            health_bar: gfx
                .create_texture()
                .from_image(include_bytes!("../assets/health.png"))
                .build()?,
            // bag_background: gfx
            //     .create_texture()
            //     .from_image(include_bytes!("../assets/bag/items.png"))
            //     .build()?,
            // party: PokedexPartyData {
            //     background: gfx
            //         .create_texture()
            //         .from_image(include_bytes!("../assets/party/background.png"))
            //         .build()?,
            //     ball: gfx
            //         .create_texture()
            //         .from_image(include_bytes!("../assets/party/ball.png"))
            //         .build()?,
            //     select: gfx
            //         .create_texture()
            //         .from_image(include_bytes!("../assets/party/select.png"))
            //         .build()?,
            //     summary: PokedexSummaryData {
            //         background: gfx
            //             .create_texture()
            //             .from_image(include_bytes!("../assets/party/summary/pokemon.png"))
            //             .build()?,
            //         pages: [
            //             gfx.create_texture()
            //                 .from_image(include_bytes!("../assets/party/summary/info.png"))
            //                 .build()?,
            //             gfx.create_texture()
            //                 .from_image(include_bytes!("../assets/party/summary/skills.png"))
            //                 .build()?,
            //             gfx.create_texture()
            //                 .from_image(include_bytes!("../assets/party/summary/moves.png"))
            //                 .build()?,
            //         ],
            //     },
            // },
            pokemon_textures,
            item_textures,
            trainer_group_textures,
        })
    }
}
