use crate::util::battle_data::TrainerData;
use firecore_util::Entity;
use crate::battle::battle::Battle;
use crate::gui::battle::battle_gui::BattleGui;
use firecore_util::Completable;

pub mod managers {
    pub mod battle_screen_transition_manager;
    pub mod battle_opener_manager;
    pub mod battle_introduction_manager;
    pub mod battle_closer_manager;
}

pub mod screen_transitions {
    pub mod flash_battle_screen_transition;
    pub mod trainer_battle_screen_transition;
    //pub mod vertical_close_battle_screen_transition;
}

pub mod openers {             
    pub mod trainer_battle_opener;
    pub mod wild_battle_opener;
}

pub mod introductions {
    pub mod trainer_battle_introduction;
    pub mod basic_battle_introduction;
    pub mod util {
        pub mod player_intro;
    }
}

pub mod closers {
    pub mod basic_battle_closer;
}

pub trait BattleTransition: Entity + Completable {

    fn on_start(&mut self);

    fn update(&mut self, delta: f32);

    fn render(&self);

}

pub trait BattleScreenTransition: BattleTransition {

    fn render_below_player(&self) {}

}

pub trait BattleOpener: BattleTransition  {

    fn offset(&self) -> f32;

    fn render_below_panel(&self) {}

}

pub trait BattleIntroduction: BattleTransition {

    fn update_gui(&mut self, battle_gui: &mut BattleGui, delta: f32);

    fn input(&mut self, delta: f32);

    fn setup(&mut self, battle: &Battle, trainer_data: Option<&TrainerData>);

    fn render_offset(&self, battle: &Battle, offset: f32);

}

pub trait BattleCloser: BattleTransition {

    fn world_active(&self) -> bool;

}