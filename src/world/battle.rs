

use crate::pokedex::pokemon::{owned::SavedPokemon, party::Party, stat::StatSet};

use rand::Rng;


use crate::game::battle_glue::{BattleEntry, BattleId};

pub const DEFAULT_RANDOM_BATTLE_SIZE: usize = 2;

pub fn random_wild_battle(random: &mut impl Rng, pokedex: u16, size: usize) -> BattleEntry {
    let mut party = Party::new();
    for _ in 0..size {
        let id = random.gen_range(0..pokedex) + 1;
        let ivs = StatSet::random_iv(random);
        let level = random.gen_range(1..=100);
        party.push(SavedPokemon::generate(random, id, level, None, Some(ivs)));
    }
    BattleEntry {
        id: BattleId::Wild,
        party,
        trainer: None,
        active: 1,
    }
}
