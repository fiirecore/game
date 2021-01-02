use std::path::Path;
use crate::engine::game_context::GameContext;
pub trait Loadable {

    fn load(&mut self);

    fn on_start(&mut self, _context: &mut GameContext) {

    }

    fn dispose(&mut self) {
        
    }

}

pub trait PersistantData {

    fn load<P>(path: P) -> Self where P: AsRef<Path>;

    fn save(&self);

}

pub trait PersistantDataLocation: PersistantData {

    fn load_from_file() -> Self;

}