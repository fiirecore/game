use game::pokedex::pokemon::instance::PokemonInstance;

#[derive(Clone)]
pub enum PokemonOption {
    Some(usize, PokemonInstance),
    None,
    ToReplace(usize),
}

impl PokemonOption {
    pub fn as_ref(&self) -> Option<&PokemonInstance> {
        match self {
            PokemonOption::Some(_, instance) => Some(instance),
            // PokemonOption::Replace(_, instance, _) => Some(instance),
            _ => None,
        }
    }
    pub fn as_mut(&mut self) -> Option<&mut PokemonInstance> {
        match self {
            PokemonOption::Some(_, instance) => Some(instance),
            // PokemonOption::Replace(_, instance, _) => Some(instance),
            _ => None,
        }
    }

    pub fn take(&mut self) -> PokemonOption {
        std::mem::replace(self, Self::None)
    }

    pub fn replace(&mut self, new: usize) -> Option<(usize, PokemonInstance)> {
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

    pub fn is_some(&self) -> bool {
        match self {
            PokemonOption::None => false,
            _ => true,
        }
    }

}
