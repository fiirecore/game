use crate::pokedex::pokemon::instance::{
    PokemonInstance,
    BorrowedPokemon,
};

#[derive(Debug, Clone)]
pub enum PokemonOption {
    Some(usize, BorrowedPokemon),
    None,
    ToReplace(usize), // new pokemon
}

impl PokemonOption {
    pub fn as_ref(&self) -> Option<&PokemonInstance> {
        match self {
            PokemonOption::Some(_, instance) => Some(instance.value()),
            // PokemonOption::Replace(_, instance, _) => Some(instance),
            _ => None,
        }
    }
    pub fn as_mut(&mut self) -> Option<&mut PokemonInstance> {
        match self {
            PokemonOption::Some(_, instance) => Some(instance.value_mut()),
            // PokemonOption::Replace(_, instance, _) => Some(instance),
            _ => None,
        }
    }

    pub fn take(&mut self) -> PokemonOption {
        std::mem::replace(self, Self::None)
    }

    pub fn replace(&mut self, new: usize) -> Option<(usize, BorrowedPokemon)> {
        if match self {
            PokemonOption::ToReplace(..) => false,
            _ => true,
        } {
            if let PokemonOption::Some(index, instance) = self.take() {
                *self = PokemonOption::ToReplace(new);
                return Some((index, instance));
            } else {
                *self = PokemonOption::ToReplace(new);
            }
        }
        None
    }

    pub fn is_active(&self) -> bool {
        match self {
            PokemonOption::Some(..) => true,
            PokemonOption::None | PokemonOption::ToReplace(..) => false,
        }
    }

}
