use deps::vec::ArrayVec;
use pokedex::pokemon::{Health, Level, PokemonRef, instance::PokemonInstance, party::PokemonParty};

pub mod gui;

pub trait BattlePartyTrait {

    fn active(&self, active: usize) -> Option<&dyn PokemonKnowData>;

    fn active_mut(&mut self, active: usize) -> Option<&mut dyn PokemonKnowData>;

    fn pokemon(&self, index: usize) -> Option<&dyn PokemonKnowData>;

    fn add(&mut self, index: usize, unknown: PokemonUnknown);

    fn replace(&mut self, active: usize, new: Option<usize>);

    fn any_inactive(&self) -> bool;
    
    // fn update_hp(&mut self, active: usize, hp: f32);
}

pub trait PokemonKnowData {

    fn pokemon(&self) -> PokemonRef;

    fn name(&self) -> &str;
    fn level(&self) -> Level;

    fn set_hp(&mut self, hp: f32);
    fn hp(&self) -> f32;

    fn fainted(&self) -> bool;

    fn instance(&self) -> Option<&PokemonInstance>;

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance>;

}

impl PokemonKnowData for PokemonInstance {
    fn pokemon(&self) -> PokemonRef {
        self.pokemon
    }

    fn name(&self) -> &str {
        PokemonInstance::name(self)
    }

    fn level(&self) -> Level {
        self.level
    }

    fn set_hp(&mut self, hp: f32) {
        self.current_hp = (hp.max(0.0) * self.max_hp() as f32) as Health
    }

    fn hp(&self) -> f32 {
        deps::log::info!("To - do: move hp / max hp into own function");
        self.hp() as f32 / self.max_hp() as f32
    }

    fn fainted(&self) -> bool {
        self.fainted()
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        Some(self)
    }

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
        Some(self)
    }

}

#[derive(Default, Clone)]
pub struct BattlePartyKnown {
    pub active: ArrayVec<[Option<usize>; 3]>,
    pub pokemon: PokemonParty,
}

impl BattlePartyTrait for BattlePartyKnown {
    fn active(&self, active: usize) -> Option<&dyn PokemonKnowData> {
        self.active.get(active).copied().flatten().map(|index| &self.pokemon[index] as _)
    }

    fn active_mut(&mut self, active: usize) -> Option<&mut dyn PokemonKnowData> {
        self.active.get(active).copied().flatten().map(move |index| &mut self.pokemon[index] as _)
    }

    fn pokemon(&self, index: usize) -> Option<&dyn PokemonKnowData> {
        self.pokemon.get(index).map(|i| i as _)
    }

    fn add(&mut self, index: usize, unknown: PokemonUnknown) {
        
    }

    fn replace(&mut self, active: usize, new: Option<usize>) {
        self.active[active] = new;
    }

    fn any_inactive(&self) -> bool {
        self.pokemon.iter().enumerate().any(|(i, p)| !(self.active.contains(&Some(i)) || p.fainted()))
    }

}

#[derive(Default, Clone)]
pub struct BattlePartyUnknown {
    pub active: ArrayVec<[Option<usize>; 3]>,
    pub pokemon: ArrayVec<[Option<PokemonUnknown>; 6]>,
}

impl BattlePartyTrait for BattlePartyUnknown {

    fn active(&self, active: usize) -> Option<&dyn PokemonKnowData> {
        self.active.get(active).copied().flatten().map(|active| &self.pokemon[active] as _)
    }
    
    fn active_mut(&mut self, active: usize) -> Option<&mut dyn PokemonKnowData> {
        // if let Some(active) = self.active.g
        self.active.get(active).copied().flatten().map(move |active| &mut self.pokemon[active] as _)
    }

    fn pokemon(&self, index: usize) -> Option<&dyn PokemonKnowData> {
        self.pokemon.get(index).map(|p| p as _)
    }

    fn add(&mut self, index: usize, unknown: PokemonUnknown) {
        self.pokemon[index] = Some(unknown);
    }

    fn replace(&mut self, active: usize, new: Option<usize>) {
        self.active[active] = new;
    }

    fn any_inactive(&self) -> bool {
        self.pokemon.iter().enumerate().filter(|(i, _)| !self.active.contains(&Some(*i))).any(|(_, unknown)| unknown.is_none() || !unknown.fainted())
    }

}

#[derive(Debug, Clone, Copy)]
pub struct PokemonUnknown {
    pub pokemon: PokemonRef,
    pub level: Level,
    pub hp: f32, // % of hp
    // pub moves:
}

impl PokemonUnknown {

    pub fn new(pokemon: &PokemonInstance) -> Self {
        Self {
            pokemon: pokemon.pokemon,
            level: pokemon.level,
            hp: pokemon.hp() as f32 / pokemon.max_hp() as f32,
        }
    }

}

impl PokemonKnowData for Option<PokemonUnknown> {

    fn pokemon(&self) -> PokemonRef {
        match self {
            Some(pokemon) => pokemon.pokemon,
            None => PokemonRef::Uninit(pokedex::pokemon::UNKNOWN_POKEMON),
        }
    }

    fn name(&self) -> &str {
        match self {
            Some(pokemon) => &pokemon.pokemon.value().name,
            None => "Unknown",
        }
    }

    fn level(&self) -> Level {
        match self {
            Some(pokemon) => pokemon.level,
            None => 0,
        }
    }

    fn set_hp(&mut self, hp: f32) {
        if let Some(pokemon) = self {
            pokemon.hp = hp.max(0.0);
        }
    }

    fn hp(&self) -> f32 {
        match self {
            Some(p) => p.hp,
            None => 0.0,
        }
    }

    fn fainted(&self) -> bool {
        match self {
            Some(p) => p.hp == 0.0,
            None => false,
        }
    }

    fn instance(&self) -> Option<&PokemonInstance> {
        None
    }

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance> {
        None
    }

}