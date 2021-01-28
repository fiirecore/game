use macroquad::prelude::warn;
use crate::util::input;
use crate::util::text_renderer::TextRenderer;

use crate::util::texture::Texture;

use crate::gui::gui::{GuiComponent, Activatable};

use crate::gui::battle::panels::battle_panel::BattlePanel;
use crate::gui::battle::panels::fight_panel::FightPanel;

use crate::game::pokedex::pokemon::pokemon_instance::PokemonInstance;
//use crate::battle::battle_manager::BattleManager;
use crate::battle::battle::Battle;

use crate::util::{texture::byte_texture, render::draw};
pub struct PlayerPanel {

    alive: bool,
    
	x: f32,
    y: f32,
    
	background: Texture,

    pub battle_panel: BattlePanel,
    pub fight_panel: FightPanel,

}

impl PlayerPanel {

	pub fn new(panel_x: f32, panel_y: f32) -> Self {

		Self {

            alive: false,

            x: panel_x,
            y: panel_y,

            background: byte_texture(include_bytes!("../../../../include/gui/battle/panel.png")),
            
            battle_panel: BattlePanel::new(panel_x, panel_y),
            fight_panel: FightPanel::new(panel_x, panel_y),
            
        }
        
	}
    
    pub fn input(&mut self, delta: f32, battle: &mut Battle) {
        if self.battle_panel.in_focus() {
            self.battle_panel.input(delta);
            if input::pressed(crate::util::input::Control::A) {
                match self.battle_panel.cursor_position {
                    0 => {
                        self.battle_panel.next = 1;
                    },
                    1 => {
                        warn!("bag button unimplemented");
                    },
                    2 => {
                        warn!("pokemon button unimplemented");
                    },
                    3 => {
                        battle.run();
                    },
                    _ => {}
                }
            }        
        } else if self.fight_panel.in_focus() {
            self.fight_panel.input(delta);
            if input::pressed(crate::util::input::Control::A) {
                self.fight_panel.disable();

                battle.queue_player_move(self.fight_panel.cursor_position as usize);
                battle.queue_opponent_move();
                battle.pmove_queued = true;
                battle.omove_queued = true;
                //let strings = self.do_move(context, battle, self.fight_panel.cursor_position as usize);
                
                //if battle.player().base.speed > battle.opponent().base.speed {
                //    self.battle_text.update_text(&battle.player().pokemon.data.name.to_uppercase(), &strings.0);
                //    self.other_pokemon = battle.opponent().pokemon.data.name.to_uppercase();
                //    self.other_move = strings.1;
                //} else {
                //    self.battle_text.update_text(&battle.opponent().pokemon.data.name.to_uppercase(), &strings.1);
                //    self.other_pokemon = battle.player().pokemon.data.name.to_uppercase();
                //    self.other_move = strings.0;
                //}
                
            }
        }// else if self.battle_text.in_focus() {
        //    self.battle_text.input(context, battle, player_pokemon_gui, opponent_pokemon_gui);
        //}
    }

    /*
    fn do_move(&mut self, context: &mut GameContext, battle: &mut Battle, index: usize) -> (String, String) {
        let str0;
        let str1;

        let opponent_move_num;
        let opponent_move_size = battle.opponent().moves.len();

        if opponent_move_size != 0 {
            opponent_move_num = context.random.rand_range(0..opponent_move_size as u32) as usize;
        } else {
            opponent_move_num = 0;
        }

        if battle.player().base.speed > battle.opponent().base.speed {
            str0 = battle.player_move(index);
            str1 = battle.opponent_move(opponent_move_num);
        } else {
            str1 = battle.opponent_move(opponent_move_num);
            str0 = battle.player_move(index);
        }
        (str0, str1)
    }
    */

    pub fn update_text(&mut self, instance: &PokemonInstance) {
        self.battle_panel.update_text(instance);
        self.fight_panel.update_names(instance);
    }

    pub fn start(&mut self) {
        self.battle_panel.enable();
        self.battle_panel.focus();
    }

}

impl GuiComponent for PlayerPanel {

	fn enable(&mut self) {
        self.alive = true;
	}

	fn disable(&mut self) {
		self.alive = false;
        
        self.battle_panel.disable();
        self.fight_panel.disable();
	}

	fn is_active(& self) -> bool {
		self.alive
	}

	fn update(&mut self, delta: f32) {
        if self.is_active() {
            if self.battle_panel.next() == 1 {
                self.battle_panel.disable();
                self.fight_panel.enable();
                self.fight_panel.focus();
            } else if self.fight_panel.next() == 1 {
                self.fight_panel.disable();
                self.battle_panel.enable();
                self.battle_panel.focus();
            }/* else if self.battle_text.next() == 1 {
                self.battle_text.update_text(&self.other_pokemon, &self.other_move);
            } else if self.battle_text.next() == 2 {
                self.battle_text.disable();
                self.battle_panel.enable();
                self.battle_panel.focus();
            }*/
            
            if self.battle_panel.in_focus() {
                self.battle_panel.update(delta);
            } else if self.fight_panel.in_focus() {
                self.fight_panel.update(delta);
            }
        }
              
	}

	fn render(&self, tr: &TextRenderer) {
		if self.is_active() {
			draw(self.background, self.x as f32, self.y as f32);
            self.battle_panel.render(tr);
            self.fight_panel.render(tr);
		}
	}

	fn update_position(&mut self, x: f32, y: f32) {
        //self.intro_text.update_position(x, y);
        self.fight_panel.update_position(x, y);
	}

}