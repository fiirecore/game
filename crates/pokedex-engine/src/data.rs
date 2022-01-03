use engine::{error::ImageError, graphics::Texture, Context};

use firecore_engine::EngineContext;
use firecore_pokedex_engine_builder::SerializedPokedexEngine;

use crate::texture::{ItemTextures, NpcGroupTextures, PokemonTextures};

pub struct PokedexClientData {
    pub health_bar: Texture,
    pub bag_background: Texture,
    pub party: PokedexPartyData,
    pub pokemon_textures: PokemonTextures,
    pub item_textures: ItemTextures,
    pub npc_group_textures: NpcGroupTextures,
}

pub struct PokedexPartyData {
    pub background: Texture,
    pub ball: Texture,
    pub select: Texture,
    pub summary: PokedexSummaryData,
}

pub struct PokedexSummaryData {
    pub pages: [Texture; 3],
    pub background: Texture,
}

impl PokedexClientData {
    pub fn new(ctx: &mut Context, eng: &mut EngineContext, data: SerializedPokedexEngine) -> Result<Self, ImageError> {
        let mut pokemon_textures = PokemonTextures::with_capacity(data.pokemon.len());

        for (id, (textures, cry)) in data.pokemon {
            if let Err(err) = pokemon_textures.insert(ctx, id, textures) {
                engine::log::warn!("Cannot add pokemon texture for {} with error {}", id, err);
            }

            #[cfg(feature = "audio")]
            if !cry.is_empty() {
                engine::sound::add_sound(ctx, eng, crate::CRY_ID, Some(id), cry);
            }
        }

        let mut item_textures = ItemTextures::with_capacity(data.items.len());

        for (id, texture) in data.items.into_iter() {
            item_textures.insert(id, Texture::new(ctx, &texture)?);
        }

        let mut npc_group_textures = NpcGroupTextures::with_capacity(data.npc_groups.len());

        for (id, texture) in data.npc_groups {
            npc_group_textures.insert(id, Texture::new(ctx, &texture)?);
        }

        Ok(Self {
            health_bar: Texture::new(ctx, include_bytes!("../assets/health.png"))?,
            bag_background: Texture::new(ctx, include_bytes!("../assets/bag/items.png"))?,
            party: PokedexPartyData {
                background: Texture::new(ctx, include_bytes!("../assets/party/background.png"))?,
                ball: Texture::new(ctx, include_bytes!("../assets/party/ball.png"))?,
                select: Texture::new(ctx, include_bytes!("../assets/party/select.png"))?,
                summary: PokedexSummaryData {
                    background: Texture::new(
                        ctx,
                        include_bytes!("../assets/party/summary/pokemon.png"),
                    )?,
                    pages: [
                        Texture::new(ctx, include_bytes!("../assets/party/summary/info.png"))?,
                        Texture::new(ctx, include_bytes!("../assets/party/summary/skills.png"))?,
                        Texture::new(ctx, include_bytes!("../assets/party/summary/moves.png"))?,
                    ],
                },
            },
            pokemon_textures,
            item_textures,
            npc_group_textures,
        })
    }
}
