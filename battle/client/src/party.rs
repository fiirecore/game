use game::{
    gui::{party::PartyGui, pokemon::PokemonDisplay},
    pokedex::{
        battle::{
            party::{knowable::BattlePartyKnown, BattleParty},
            view::{PokemonView, UnknownPokemon},
            Active, PartyIndex,
        },
        pokemon::instance::PokemonInstance,
    },
};

use battle::player::BattlePlayer;

pub trait BattlePartyView<ID> {
    fn id(&self) -> &ID;

    fn name(&self) -> &str;

    fn active(&self, active: Active) -> Option<&dyn PokemonView>;

    fn active_mut(&mut self, active: Active) -> Option<&mut dyn PokemonView>;

    fn active_len(&self) -> usize;

    fn len(&self) -> usize;

    fn active_eq(&self, active: Active, index: Option<PartyIndex>) -> bool;

    fn index(&self, active: Active) -> Option<PartyIndex>;

    fn pokemon(&self, index: PartyIndex) -> Option<&dyn PokemonView>;

    // fn pokemon_mut(&mut self, index: PartyIndex) -> Option<&mut dyn PokemonView>;

    fn replace(&mut self, active: Active, new: Option<PartyIndex>);

    fn any_inactive(&self) -> bool;

    // fn update_hp(&mut self, active: usize, hp: f32);
}

pub trait BattlePartyEditableView<ID>: BattlePartyView<ID> {
    fn add(&mut self, index: PartyIndex, unknown: UnknownPokemon);
}

impl<ID, P: PokemonView> BattlePartyView<ID> for BattleParty<ID, Option<usize>, P> {
    fn id(&self) -> &ID {
        &self.id
    }

    fn name(&self) -> &str {
        BattleParty::name(&self)
    }

    fn active(&self, active: usize) -> Option<&dyn PokemonView> {
        self.active
            .get(active)
            .copied()
            .flatten()
            .map(|active| &self.pokemon[active] as _)
    }

    fn active_mut(&mut self, active: usize) -> Option<&mut dyn PokemonView> {
        self.active
            .get(active)
            .copied()
            .flatten()
            .map(move |active| &mut self.pokemon[active] as _)
    }

    fn active_len(&self) -> usize {
        self.active.len()
    }

    fn len(&self) -> usize {
        self.pokemon.len()
    }

    fn active_eq(&self, active: usize, index: Option<usize>) -> bool {
        self.active
            .get(active)
            .map(|i| i == &index)
            .unwrap_or_default()
    }

    fn index(&self, active: Active) -> Option<PartyIndex> {
        self.active.get(active).copied().flatten()
    }

    fn pokemon(&self, index: usize) -> Option<&dyn PokemonView> {
        self.pokemon.get(index).map(|p| p as _)
    }

    fn replace(&mut self, active: usize, new: Option<usize>) {
        self.active[active] = new;
    }

    fn any_inactive(&self) -> bool {
        self.pokemon
            .iter()
            .enumerate()
            .any(|(i, p)| !(self.active.contains(&Some(i)) || p.fainted()))
    }
}

impl<ID> BattlePartyEditableView<ID> for BattleParty<ID, Option<usize>, PokemonInstance> {
    fn add(&mut self, _index: PartyIndex, _unknown: UnknownPokemon) {}
}

impl<ID> BattlePartyEditableView<ID> for BattleParty<ID, Option<usize>, Option<UnknownPokemon>> {
    fn add(&mut self, index: PartyIndex, unknown: UnknownPokemon) {
        self.pokemon[index] = Some(unknown);
    }
}

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
