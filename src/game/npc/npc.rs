use crate::engine::game_context::GameContext;
use crate::entity::util::direction::Direction;
use crate::io::data::npc::trainer::Trainer;

pub struct NPCInstance {

    // x and y local to map

    pub x: isize,
    pub y: isize,

    pub direction: Direction,

    pub name: String,
    pub sprite: u8,

    pub trainer: Option<Trainer>,

}

impl NPCInstance {

    pub fn interact(&mut self, direction: Direction, context: &mut GameContext) {
        self.direction = direction.inverse();
        self.test_trainer(context);
    }

    fn test_trainer(&self, context: &mut GameContext) {
        if self.trainer.is_some() {
            context.battle_context.trainer_battle(&self);
        }
    }

}