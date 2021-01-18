use crate::util::context::GameContext;
pub trait Loadable {

    fn load(&mut self);

    fn on_start(&mut self, _context: &mut GameContext) {

    }

    fn dispose(&mut self) {
        
    }

}

#[deprecated]
pub trait Completable {

    fn is_finished(&self) -> bool;

}