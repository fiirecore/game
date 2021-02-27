use macroquad::prelude::warn;
use crate::battle::battle_pokemon::BattlePokemon;
use crate::util::Entity;
use crate::gui::Focus;
use crate::util::Input;
use crate::io::input;
use crate::util::graphics::Texture;
use crate::gui::GuiComponent;
use crate::gui::battle::panels::battle_panel::BattlePanel;
use crate::gui::battle::panels::fight_panel::FightPanel;
use crate::battle::battle::Battle;
use crate::util::graphics::draw;
use crate::util::graphics::texture::byte_texture;
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

            background: byte_texture(include_bytes!("../../../../build/assets/gui/battle/panel.png")),
            
            battle_panel: BattlePanel::new(panel_x, panel_y),
            fight_panel: FightPanel::new(panel_x, panel_y),
            
        }
        
	}
    
    pub fn input(&mut self, delta: f32, battle: &mut Battle) {
        if self.battle_panel.in_focus() {
            self.battle_panel.input(delta);
            if input::pressed(crate::io::input::Control::A) {
                match self.battle_panel.cursor_position {
                    0 => {
                        self.battle_panel.next = 1;
                    },
                    1 => {
                        warn!("bag button unimplemented");
                    },
                    2 => {
                        crate::gui::game::pokemon_party_gui::toggle();
                    },
                    3 => {
                        battle.run();
                    },
                    _ => {}
                }
            }        
        } else if self.fight_panel.in_focus() {
            self.fight_panel.input(delta);
            if input::pressed(crate::io::input::Control::A) {
                self.fight_panel.despawn();

                battle.queue_player_move(self.fight_panel.cursor_position as usize);
                battle.queue_opponent_move();
                battle.pmove_queued = true;
                battle.omove_queued = true;
                
            }
        }
    }

    pub fn update_text(&mut self, instance: &BattlePokemon) {
        self.battle_panel.update_text(instance);
        self.fight_panel.update_names(instance);
    }

    pub fn start(&mut self) {
        self.battle_panel.spawn();
        self.battle_panel.focus();
    }

}

impl GuiComponent for PlayerPanel {

	fn update(&mut self, delta: f32) {
        if self.is_alive() {
            if self.battle_panel.next == 1 {
                self.battle_panel.despawn();
                self.fight_panel.spawn();
                self.fight_panel.focus();
            } else if self.fight_panel.next == 1 {
                self.fight_panel.despawn();
                self.battle_panel.spawn();
                self.battle_panel.focus();
            }
            
            if self.battle_panel.in_focus() {
                self.battle_panel.update(delta);
            } else if self.fight_panel.in_focus() {
                self.fight_panel.update(delta);
            }
        }
              
	}

	fn render(&self) {
		if self.is_alive() {
			draw(self.background, self.x as f32, self.y as f32);
            self.battle_panel.render();
            self.fight_panel.render();
		}
	}

	fn update_position(&mut self, x: f32, y: f32) {
        self.fight_panel.update_position(x, y);
	}

}

impl Entity for PlayerPanel {

	fn spawn(&mut self) {
        self.alive = true;
	}

	fn despawn(&mut self) {
		self.alive = false;
        
        self.battle_panel.despawn();
        self.fight_panel.despawn();
	}

	fn is_alive(& self) -> bool {
		self.alive
	}

}