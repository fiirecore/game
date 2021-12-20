use crate::{
    engine::{
        graphics::{draw_rectangle, Color, DrawParams, Texture},
        math::Rectangle,
        Context,
    },
    world::npc::NpcTypes,
};
use firecore_battle_gui::pokedex::texture::NpcGroupTextures;
use firecore_world::{
    character::{
        npc::NpcType,
        sprite::{SpriteIndexType, SpriteIndexes},
    },
    serialized::SerializedNpcType,
};
use std::collections::HashMap;
use worldlib::{
    character::npc::{Npc, NpcTypeId},
    positions::Direction,
    TILE_SIZE,
};

use crate::world::RenderCoords;

pub type NpcTextures = HashMap<NpcTypeId, Texture>;

pub struct NpcData {
    pub npcs: NpcTextures,
    pub types: NpcTypes,
    pub trainer: NpcGroupTextures,
}

impl NpcData {
    pub fn new(ctx: &mut Context, npc_types: Vec<SerializedNpcType>) -> Self {
        let mut npcs = NpcTextures::with_capacity(npc_types.len());
        let mut types = NpcTypes::with_capacity(npc_types.len());
        let mut trainer = NpcGroupTextures::new(Default::default());
        for npc in npc_types {
            npcs.insert(
                npc.config.identifier,
                Texture::new(ctx, &npc.texture).unwrap(),
            );
            types.insert(
                npc.config.identifier,
                NpcType {
                    sprite: match npc.config.sprite {
                        SpriteIndexType::Still => SpriteIndexes::STILL,
                        SpriteIndexType::Walk => SpriteIndexes::WALK,
                    },
                    message: npc.config.text_color,
                    trainer: npc.config.trainer,
                },
            );
            trainer.insert(
                npc.config.identifier,
                Texture::new(ctx, &npc.texture).unwrap(),
            );
        }

        Self {
            npcs,
            types,
            trainer,
        }
    }

    pub fn draw(&self, ctx: &mut Context, npc: &Npc, screen: &RenderCoords) {
        let x = ((npc.character.position.coords.x + screen.offset.x) << 4) as f32 - screen.focus.x
            + npc.character.offset.x;
        let y = ((npc.character.position.coords.y - 1 + screen.offset.y) << 4) as f32
            - screen.focus.y
            + npc.character.offset.y;

        if let Some(texture) = self.npcs.get(&npc.type_id) {
            texture.draw(
                ctx,
                x,
                y,
                DrawParams {
                    flip_x: npc.character.position.direction == Direction::Right,
                    source: Some(Rectangle::new(
                        current_texture_pos(&self.types, npc),
                        0.0,
                        16.0,
                        32.0,
                    )),
                    ..Default::default()
                },
            );
        } else {
            draw_rectangle(
                ctx,
                x,
                y + TILE_SIZE,
                TILE_SIZE,
                TILE_SIZE * 2.0,
                Color::rgb(1.0, 0.0, 0.0),
            );
        }
    }
}

pub fn current_texture_pos(npc_types: &NpcTypes, npc: &Npc) -> f32 {
    let index = (npc.character.offset.offset().abs() as usize >> 3) + npc.character.sprite as usize;

    npc_types
        .get(&npc.type_id)
        .map(|npc_type| {
            (match npc.character.position.direction {
                Direction::Down => npc_type.sprite.down[index],
                Direction::Up => npc_type.sprite.up[index],
                _ => npc_type.sprite.side[index],
            } << 4) as f32
        })
        .unwrap_or_default()
}
