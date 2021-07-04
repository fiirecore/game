use game::{
    gui::{party::PartyGui, pokemon::PokemonDisplay},
    pokedex::battle::party::knowable::BattlePartyKnown,
};

use battle::player::BattlePlayer;

#[deprecated(note = "replace with one function that uses into for pokemon display")]
pub fn battle_party_known_gui<
    ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord,
>(
    gui: &PartyGui,
    party: &BattlePartyKnown<ID>,
    exitable: bool,
) {
    gui.spawn(
        party
            .pokemon
            .iter()
            .cloned()
            .map(|instance| PokemonDisplay::new(std::borrow::Cow::Owned(instance)))
            .collect(),
        Some(false),
        exitable,
    );
}

pub fn battle_party_gui<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord>(
    gui: &PartyGui,
    player: &BattlePlayer<ID>,
    exitable: bool,
) {
    gui.spawn(
        player
            .party
            .cloned()
            .into_iter()
            .map(|instance| PokemonDisplay::new(std::borrow::Cow::Owned(instance)))
            .collect(),
        Some(false),
        exitable,
    );
}
