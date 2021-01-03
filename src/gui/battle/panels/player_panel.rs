use log::warn;
use opengl_graphics::{GlGraphics, Texture};
use piston_window::Context;
use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;

use crate::gui::gui::{GuiComponent, Activatable};
use crate::gui::battle::battle_gui::BattleActivatable;

use crate::gui::battle::panels::battle_panel::BattlePanel;
use crate::gui::battle::panels::fight_panel::FightPanel;

use crate::gui::battle::intro_text::IntroText;
use crate::gui::battle::battle_text::BattleText;

use crate::game::pokedex::pokemon::pokemon_instance::PokemonInstance;
//use crate::game::battle::battle_manager::BattleManager;
use crate::game::battle::battle::Battle;

use crate::gui::battle::pokemon_gui::*;

use crate::util::{texture_util::texture_from_path, file_util::asset_as_pathbuf, render_util::draw};
pub struct PlayerPanel {

    alive: bool,
    
	x: isize,
    y: isize,
    
	background: Texture,

    pub battle_panel: BattlePanel,
    pub fight_panel: FightPanel,

    pub intro_text: IntroText,
    
    pub battle_text: BattleText,
    other_pokemon: String,
    other_move: String,

}

impl PlayerPanel {

	pub fn new(panel_x: isize, panel_y: isize) -> Self {

		Self {

            alive: false,

            x: panel_x,
            y: panel_y,

            background: texture_from_path(asset_as_pathbuf("gui/battle/panel.png")),
            
            battle_panel: BattlePanel::new(panel_x, panel_y),
            fight_panel: FightPanel::new(panel_x, panel_y),
            
            intro_text: IntroText::new(panel_x, panel_y),
            
            battle_text: BattleText::new(panel_x, panel_y),

            other_pokemon: String::new(),
            other_move: String::new(),
            
        }
        
	}

	pub fn load(&mut self) {
		
    }
    
    pub fn input(&mut self, context: &mut GameContext, battle: &mut Battle, player_pokemon_gui: &mut PlayerPokemonGui, opponent_pokemon_gui: &mut OpponentPokemonGui) {
        if self.intro_text.in_focus() {
            self.intro_text.input(context);
        } else if self.battle_panel.in_focus() {
            self.battle_panel.input(context);
            if context.keys[0] == 1 {
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
            self.fight_panel.input(context);
            if context.keys[0] == 1 {
                let strings = self.do_move(context, battle, self.fight_panel.cursor_position as usize);
                self.fight_panel.disable();
                if battle.player_pokemon.speed > battle.opponent_pokemon.speed {
                    self.battle_text.update_text(&battle.player_pokemon.instance.as_ref().unwrap().pokemon.name.to_uppercase(), &strings.0);
                    self.other_pokemon = battle.opponent_pokemon.instance.as_ref().unwrap().pokemon.name.to_uppercase();
                    self.other_move = strings.1;
                } else {
                    self.battle_text.update_text(&battle.opponent_pokemon.instance.as_ref().unwrap().pokemon.name.to_uppercase(), &strings.1);
                    self.other_pokemon = battle.player_pokemon.instance.as_ref().unwrap().pokemon.name.to_uppercase();
                    self.other_move = strings.0;
                }
                
                self.battle_text.enable(); // battle text here
                self.battle_text.focus();
            }
        } else if self.battle_text.in_focus() {
            self.battle_text.input(context, battle, player_pokemon_gui, opponent_pokemon_gui);
        }
    }

    fn do_move(&mut self, context: &mut GameContext, battle: &mut Battle, index: usize) -> (String, String) {
        let str0;
        let str1;

        let opponent_move_num;
        let opponent_move_size = battle.opponent_pokemon.instance.as_ref().unwrap().moves.len();

        if opponent_move_size != 0 {
            opponent_move_num = context.random.rand_range(0..opponent_move_size as u32) as usize;
        } else {
            opponent_move_num = 0;
        }

        if battle.player_pokemon.speed > battle.opponent_pokemon.speed {
            str0 = battle.player_move(index);
            str1 = battle.opponent_move(opponent_move_num);
        } else {
            str1 = battle.opponent_move(opponent_move_num);
            str0 = battle.player_move(index);
        }
        (str0, str1)
    }

    pub fn update_text(&mut self, instance: &PokemonInstance) {
        let mut player_string = String::from("Go! ");
        player_string.push_str(instance.pokemon.name.to_uppercase().as_str());
        player_string.push_str("!");
        //println!("{}", player_string);
        self.intro_text.player_text = player_string;
        self.battle_panel.update_text(instance);
        self.fight_panel.update_names(instance);
    }

}

impl GuiComponent for PlayerPanel {

	fn enable(&mut self) {
        self.alive = true;
	}

	fn disable(&mut self) {
		self.alive = false;
		self.intro_text.disable();
        self.battle_text.disable();
        self.battle_panel.disable();
        self.fight_panel.disable();
	}

	fn is_active(& self) -> bool {
		self.alive
	}

	fn update(&mut self, context: &mut GameContext) {
        if self.is_active() {
            self.intro_text.update(context);

            match self.intro_text.next() {
                1 => {
                    self.intro_text.update_text(self.intro_text.player_text.clone());
                    self.intro_text.enable();
                    self.intro_text.no_pause = true;
                },
                2 => {
                    self.intro_text.disable();
                    self.battle_panel.enable();
                    self.battle_panel.focus();
                },
                _ => {}
            }
            if self.battle_panel.next() == 1 {
                self.battle_panel.disable();
                self.fight_panel.enable();
                self.fight_panel.focus();
            } else if self.fight_panel.next() == 1 {
                self.fight_panel.disable();
                self.battle_panel.enable();
                self.battle_panel.focus();
            } else if self.battle_text.next() == 1 {
                self.battle_text.update_text(&self.other_pokemon, &self.other_move);
            } else if self.battle_text.next() == 2 {
                self.battle_text.disable();
                self.battle_panel.enable();
                self.battle_panel.focus();
            }
            
            if self.battle_panel.in_focus() {
                self.battle_panel.update(context);
            } else if self.fight_panel.in_focus() {
                self.fight_panel.update(context);
            } else if self.battle_text.in_focus() {
                self.battle_text.update(context);
            }
        }
              
	}

	fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		if self.is_active() {
			draw(ctx, g, &self.background, self.x, self.y);
			self.intro_text.render(ctx, g, tr);
            self.battle_text.render(ctx, g, tr);
            self.battle_panel.render(ctx, g, tr);
            self.fight_panel.render(ctx, g, tr);
		}
	}

	fn update_position(&mut self, x: isize, y: isize) {
        self.intro_text.update_position(x, y);
        self.battle_text.update_position(x, y);
        self.battle_text.update_position(x, y);
        self.fight_panel.update_position(x, y);
	}

}