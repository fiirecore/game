use pokedex::{
    pokemon::{Health, Level, PokemonRef, instance::PokemonInstance},
};

use super::PokemonView;

#[derive(Default, Debug, Clone)]
pub struct UnknownPokemon {
    pokemon: PokemonRef,
    name: String,
    level: Level,
    hp: f32, // % of hp
    pub instance: Option<PokemonInstance>
}

impl UnknownPokemon {

    pub fn new(pokemon: &PokemonInstance) -> Self {
        Self {
            pokemon: pokemon.pokemon,
            name: pokemon.name().to_owned(),
            level: pokemon.level,
            hp: pokemon.percent_hp(),
            instance: None,
        }
    }

}

impl PokemonView for UnknownPokemon {

    fn pokemon(&self) -> PokemonRef {
        self.pokemon
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn level(&self) -> Level {
        self.level
    }

    fn set_hp(&mut self, hp: f32) {
        self.hp = hp.max(0.0);
    }

    fn hp(&self) -> f32 {
        self.hp
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
    fn pokemon(&self) -> PokemonRef { self.pokemon }

    fn name(&self) -> &str { PokemonInstance::name(self) }

    fn level(&self) -> Level { self.level }

    fn set_hp(&mut self, hp: f32) { self.current_hp = (hp.max(0.0) * self.max_hp() as f32) as Health }

    fn hp(&self) -> f32 { self.percent_hp() }

    fn fainted(&self) -> bool { PokemonInstance::fainted(self) }

    fn instance(&self) -> Option<&PokemonInstance> { Some(self) }

    fn instance_mut(&mut self) -> Option<&mut PokemonInstance> { Some(self) }

}

impl PokemonView for Option<UnknownPokemon> {
    fn pokemon(&self) -> PokemonRef {
        self.as_ref().map(|v| v.pokemon()).unwrap_or_default()
    }

    fn name(&self) -> &str {
        self.as_ref().map(|v| v.name()).unwrap_or("Unknown")
    }

    fn level(&self) -> Level {
        self.as_ref().map(|v| v.level()).unwrap_or_default()
    }

    fn set_hp(&mut self, hp: f32) {
        if let Some(v) = self.as_mut() {
            v.set_hp(hp);
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

}