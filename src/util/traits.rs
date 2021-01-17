use std::path::Path;
use crate::util::context::GameContext;
pub trait Loadable {

    fn load(&mut self);

    fn on_start(&mut self, _context: &mut GameContext) {

    }

    fn dispose(&mut self) {
        
    }

}

pub trait Completable {

    fn is_finished(&self) -> bool;

}

pub trait PersistantData {

    fn load<P>(path: P) -> Self where P: AsRef<Path>;

    fn save(&self);

    fn reload(&mut self);

}

pub trait PersistantDataLocation: PersistantData {

    fn load_from_file() -> Self;

}