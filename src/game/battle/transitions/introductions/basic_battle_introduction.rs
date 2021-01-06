use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::entity::entity::Entity;
use crate::entity::entity::Ticking;
use crate::game::battle::battle::Battle;
use crate::game::battle::transitions::battle_transition_traits::BattleIntroduction;
use crate::game::battle::transitions::battle_transition_traits::BattleTransition;
use crate::gui::battle::battle_gui::BattleGui;
use crate::gui::gui::Activatable;
use crate::gui::gui::GuiComponent;
use crate::util::render_util::draw_bottom;
use crate::util::timer::Timer;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;

use super::util::intro_text::IntroText;
use super::util::player_intro::PlayerBattleIntro;

pub struct BasicBattleIntroduction {

    alive: bool,
    finished: bool,

    pub intro_text: IntroText,
    pub player_intro: PlayerBattleIntro,
    pub timer: Timer,

}

impl BasicBattleIntroduction {

    pub fn new(panel_x: isize, panel_y: isize) -> Self {

        Self {

            alive: false,
            finished: false,

            intro_text: IntroText::new(panel_x, panel_y, vec![String::from("Intro Text")]),
            player_intro: PlayerBattleIntro::new(),
            timer: Timer::new(60),

        }

    }

}

impl BattleTransition for BasicBattleIntroduction {

    fn reset(&mut self) {
        self.intro_text.load();
    }

}

impl Loadable for BasicBattleIntroduction {

    fn load(&mut self) {
        self.player_intro.load();
    }

    fn on_start(&mut self, context: &mut GameContext) {
        self.player_intro.on_start(context);
    }

}

impl Completable for BasicBattleIntroduction {

    fn is_finished(&self) -> bool {
        self.finished && !self.player_intro.should_update()
    }

}

impl Entity for BasicBattleIntroduction {

    fn spawn(&mut self) {
        self.alive = true;
        self.finished = false;
        self.intro_text.enable();
        self.intro_text.focus();
    }

    fn despawn(&mut self) {
        self.alive = false;
        self.finished = false;
        self.intro_text.disable();
        self.timer.despawn();
        self.reset();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

impl Ticking for BasicBattleIntroduction {

    fn update(&mut self, context: &mut GameContext) {

        if self.intro_text.next() > 0 && self.intro_text.can_continue {
            if !self.intro_text.no_pause {
                self.intro_text.no_pause = true;
                self.timer.spawn();
            } else {
                self.timer.update();
                if self.player_intro.should_update() {
                    self.player_intro.update();
                } else if self.timer.is_finished() {
                    self.finished = true;
                }
            }       
        } else {
            self.intro_text.update(context);
        }
        
	}

    fn render(&self, ctx: &mut piston_window::Context, g: &mut opengl_graphics::GlGraphics, tr: &mut crate::engine::text::TextRenderer) {
        self.intro_text.render(ctx, g, tr);
	}
}

impl BattleIntroduction for BasicBattleIntroduction {

    fn input(&mut self, context: &mut GameContext) {
        self.intro_text.input(context);
    }

    fn setup_text(&mut self, battle: &Battle) {
        let mut opponent_string = String::from("Wild ");
		opponent_string.push_str(battle.opponent().pokemon.name.to_uppercase().as_str());
		opponent_string.push_str(" appeared!");
        let mut player_string = String::from("Go! ");
        player_string.push_str(battle.player().pokemon.name.to_uppercase().as_str());
        player_string.push_str("!");
        self.intro_text.text = vec![opponent_string, player_string];
    }

    fn render_offset(&self, ctx: &mut Context, g: &mut GlGraphics, battle: &Battle, offset: u16) {
        draw_bottom(ctx, g, &battle.opponent_textures[battle.opponent_active], 144 - offset as isize, 74);
        if self.player_intro.should_update() {
            self.player_intro.draw(ctx, g, offset);
        } else {
		    draw_bottom(ctx, g, &battle.player_textures[battle.player_active], 40 + offset as isize, 113);
        }        
    }

    fn update_gui(&mut self, battle_gui: &mut BattleGui) {
        if self.intro_text.can_continue {
            if self.intro_text.next() == self.intro_text.text.len() as u8 - 2 {
                battle_gui.opponent_pokemon_gui.spawn();
            } else if self.intro_text.next() == self.intro_text.text.len() as u8 - 1 {
                battle_gui.player_pokemon_gui.spawn();
            }
        }
        
    }

}