use serde::{Deserialize, Serialize};

use pokedex::{
    item::bag::SavedBag,
    pokemon::{owned::SavedPokemon, party::Party},
};

use text::MessagePage;

use crate::{
    character::{
        npc::{
            group::{MessageColor, NpcGroupId},
            trainer::BadgeId,
            Npc, NpcId,
        },
        trainer::Worth,
    },
    map::TransitionId,
    positions::Location,
    state::GlobalBattleState,
};

use super::{manager::WorldNpcData, WorldMapSettings};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BattleEntry<P> {
    pub id: BattleId,
    pub party: Party<P>,
    pub active: usize,
    pub trainer: Option<TrainerEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize)]
pub enum BattleId {
    Default,
    Player,
    Wild,
    Trainer(NpcId),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrainerEntry {
    pub id: NpcId,
    pub location: Location,
    pub name: String,
    pub bag: SavedBag,
    pub badge: Option<BadgeId>,
    pub sprite: NpcGroupId,
    pub transition: TransitionId,
    pub defeat: Vec<MessagePage<MessageColor>>,
    pub worth: Worth,
}

impl BattleEntry<SavedPokemon> {
    pub fn trainer(
        world: &mut GlobalBattleState,
        map: &Location,
        settings: &WorldMapSettings,
        data: &WorldNpcData,
        id: &NpcId,
        npc: &Npc,
    ) -> Option<Self> {
        if let Some(trainer) = npc.trainer.as_ref() {
            if !world.battled(map, id) {
                return Some(BattleEntry {
                    id: BattleId::Trainer(*id),
                    party: trainer.character.party.clone(),
                    active: 1,
                    trainer: Some(TrainerEntry {
                        id: *id,
                        location: *map,
                        name: data
                            .trainers
                            .get(&trainer.group)
                            .map(|g| format!("{} {}", g.prefix, npc.character.name))
                            .unwrap_or_else(|| npc.character.name.clone()),
                        bag: trainer.character.bag.clone(),
                        badge: trainer.badge,
                        sprite: npc.group,
                        transition: settings.transition,
                        defeat: trainer
                            .defeat
                            .iter()
                            .map(|lines| MessagePage {
                                lines: lines.to_owned(),
                                wait: None,
                                color: data
                                    .groups
                                    .get(&npc.group)
                                    .map(|g| g.message)
                            })
                            .collect(),
                        worth: trainer.character.worth,
                    }),
                });
            }
        }
        None
    }
}

impl Default for BattleId {
    fn default() -> Self {
        Self::Default
    }
}

impl core::fmt::Display for BattleId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}
