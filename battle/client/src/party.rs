use game::{
    gui::{party::PartyGui, pokemon::PokemonDisplay},
    pokedex::{
        battle::party::BattleParty,
        pokemon::instance::PokemonInstance,
    },
};

pub fn battle_party_gui<
    ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord,
    A,
>(
    gui: &PartyGui,
    party: &BattleParty<ID, A, PokemonInstance>,
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