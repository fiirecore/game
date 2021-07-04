use game::pokedex::{
    battle::{party::BattleParty, view::UnknownPokemon, Active, PartyIndex},
    pokemon::{
        instance::{BorrowedPokemon, PokemonInstance},
        Health, Level, PokemonRef,
    },
    status::StatusEffectInstance,
};

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

    fn replace(&mut self, active: Active, new: Option<PartyIndex>);

    fn any_inactive(&self) -> bool;
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

pub trait PokemonView {
    fn pokemon(&self) -> PokemonRef;

    fn name(&self) -> &str;

    fn set_level(&mut self, level: Level);
    fn level(&self) -> Level;

    fn set_hp(&mut self, hp: f32);
    fn hp(&self) -> f32;

    fn set_effect(&mut self, effect: StatusEffectInstance);
    fn effect(&mut self) -> Option<&mut StatusEffectInstance>;

    fn fainted(&self) -> bool;

    fn instance(&self) -> Option<&PokemonInstance>;

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance>;

    // fn instance_mut(&mut self) -> Option<&mut PokemonInstance>;
}

impl PokemonView for UnknownPokemon {
    fn pokemon(&self) -> PokemonRef {
        self.pokemon
    }

    fn name(&self) -> &str {
        UnknownPokemon::name(&self)
    }

    fn level(&self) -> Level {
        self.level
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    fn set_hp(&mut self, hp: f32) {
        self.hp = hp.max(0.0);
    }

    fn hp(&self) -> f32 {
        self.hp
    }

    fn set_effect(&mut self, effect: StatusEffectInstance) {
        self.effect = Some(effect);
    }

    fn effect(&mut self) -> Option<&mut StatusEffectInstance> {
        self.effect.as_mut()
    }

    fn fainted(&self) -> bool {
        self.hp == 0.0
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        self.instance.as_ref()
    }

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
        self.instance.as_mut()
    }
}

impl PokemonView for PokemonInstance {
    fn pokemon(&self) -> PokemonRef {
        self.pokemon
    }

    fn name(&self) -> &str {
        PokemonInstance::name(self)
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    fn level(&self) -> Level {
        self.level
    }

    fn set_hp(&mut self, hp: f32) {
        self.current_hp = (hp.max(0.0) * self.max_hp() as f32) as Health
    }

    fn hp(&self) -> f32 {
        self.percent_hp()
    }

    fn fainted(&self) -> bool {
        PokemonInstance::fainted(self)
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        Some(self)
    }

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
        Some(self)
    }

    fn set_effect(&mut self, effect: StatusEffectInstance) {
        self.effect = Some(effect);
    }

    fn effect(&mut self) -> Option<&mut StatusEffectInstance> {
        self.effect.as_mut()
    }
}

impl PokemonView for Option<UnknownPokemon> {
    fn pokemon(&self) -> PokemonRef {
        self.as_ref().map(|v| v.pokemon()).unwrap_or_default()
    }

    fn name(&self) -> &str {
        self.as_ref().map(|v| v.name()).unwrap_or("Unknown")
    }

    fn set_level(&mut self, level: Level) {
        if let Some(u) = self.as_mut() {
            u.set_level(level)
        }
    }

    fn level(&self) -> Level {
        self.as_ref().map(|v| v.level()).unwrap_or_default()
    }

    fn set_hp(&mut self, hp: f32) {
        if let Some(u) = self.as_mut() {
            u.set_hp(hp);
        }
    }

    fn hp(&self) -> f32 {
        self.as_ref().map(|v| v.hp()).unwrap_or_default()
    }

    fn fainted(&self) -> bool {
        self.as_ref().map(|v| v.fainted()).unwrap_or_default()
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        self.as_ref().map(|u| u.instance()).flatten()
    }

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
        self.as_mut().map(|u| u.instance_mut()).flatten()
    }

    fn set_effect(&mut self, effect: StatusEffectInstance) {
        if let Some(u) = self {
            u.set_effect(effect);
        }
    }

    fn effect(&mut self) -> Option<&mut StatusEffectInstance> {
        self.as_mut().map(|u| u.effect()).flatten()
    }
}

impl PokemonView for BorrowedPokemon {
    fn pokemon(&self) -> PokemonRef {
        self.pokemon
    }

    fn name(&self) -> &str {
        PokemonInstance::name(self)
    }

    fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    fn level(&self) -> Level {
        self.level
    }

    fn set_hp(&mut self, hp: f32) {
        self.current_hp = (hp.max(0.0) * self.max_hp() as f32) as Health
    }

    fn hp(&self) -> f32 {
        self.percent_hp()
    }

    fn fainted(&self) -> bool {
        PokemonInstance::fainted(self)
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        Some(self)
    }

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
        Some(self)
    }

    fn set_effect(&mut self, effect: StatusEffectInstance) {
        self.effect = Some(effect);
    }

    fn effect(&mut self) -> Option<&mut StatusEffectInstance> {
        self.effect.as_mut()
    }
}
