use crate::battle::battle::Battle;
use crate::gui::dynamic_text::DynamicText;
use firecore_util::text::MessageSet;
use firecore_util::text::TextColor;
use firecore_util::Entity;

use super::battle_gui::BattleGui;
use super::pokemon_gui::PokemonGui;

#[deprecated(note = "fix the timer so it uses \"wait_for_input = false\" timer instead")]
pub fn pmove(delta: f32, battle: &mut Battle, battle_gui: &mut BattleGui) {
    if battle.pmove_queued {
        if battle_gui.battle_text.is_alive() {
            if battle_gui.battle_text.can_continue {
                if !battle_gui.opponent_pokemon_gui.health_bar.is_moving()
                    && battle_gui.battle_text.timer.is_finished()
                {
                    battle.pmove_queued = false;
                    battle_gui.battle_text.despawn();

                    if battle.opponent().faint() {
                        battle.faint_queued = true;
                        battle.omove_queued = false;
                    }
                } else if !battle_gui.battle_text.timer.is_alive() {
                    battle_gui.battle_text.timer.spawn();
                    battle_gui
                        .opponent_pokemon_gui
                        .health_bar
                        .update_bar(battle.opponent().current_hp, battle.opponent().base.hp);
                }
                battle_gui.battle_text.timer.update(delta);
            }
        } else {
            battle.player_move();
            battle_gui.battle_text.spawn();
            update_text(
                &mut battle_gui.battle_text,
                &battle.player().pokemon.data.name,
                &battle.player_move.name,
            );
        }
    } else if battle.faint_queued {
        faint_queued(delta, battle, battle_gui);
    } else if battle.omove_queued {
        omove(delta, battle, battle_gui);
    }
}

pub fn omove(delta: f32, battle: &mut Battle, battle_gui: &mut BattleGui) {
    if battle.omove_queued {
        if battle_gui.battle_text.is_alive() {
            if battle_gui.battle_text.can_continue {
                if !battle_gui.player_pokemon_gui.health_bar.is_moving()
                    && battle_gui.battle_text.timer.is_finished()
                {
                    battle.omove_queued = false;
                    battle_gui.battle_text.despawn();

                    if battle.player().faint() {
                        battle.faint_queued = true;
                        battle.pmove_queued = false;
                    }
                } else if !battle_gui.battle_text.timer.is_alive() {
                    battle_gui.battle_text.timer.spawn();
                    battle_gui
                        .player_pokemon_gui
                        .update_hp(battle.player().current_hp, battle.player().base.hp);
                }
                battle_gui.battle_text.timer.update(delta);
            }
        } else {
            battle.opponent_move();
            battle_gui.battle_text.spawn();
            update_text(
                &mut battle_gui.battle_text,
                &battle.opponent().pokemon.data.name,
                &battle.opponent_move.name,
            );
        }
    } else if battle.faint_queued {
        faint_queued(delta, battle, battle_gui);
    } else if battle.pmove_queued {
        pmove(delta, battle, battle_gui);
    }
}

fn faint_queued(delta: f32, battle: &mut Battle, battle_gui: &mut BattleGui) {
    if battle.player().faint() {
        if battle_gui.battle_text.is_alive() {
            if battle_gui.battle_text.can_continue {
                if battle_gui.battle_text.timer.is_finished() {
                    battle_gui.battle_text.despawn();
                    battle.faint_queued = false;
                    battle.faint = true;
                } else if !battle_gui.battle_text.timer.is_alive() {
                    battle_gui.battle_text.timer.spawn();
                }
                battle_gui.battle_text.timer.update(delta);
            }
        } else {
            battle_gui.battle_text.spawn();
            update_faint(
                &mut battle_gui.battle_text,
                &battle.player().pokemon.data.name,
            );
        }
    } else {
        if battle_gui.battle_text.is_alive() {
            if battle_gui.battle_text.can_continue {
                if battle_gui.battle_text.timer.is_finished() {
                    battle.faint_queued = false;
                    battle_gui.battle_text.despawn();
                    battle.faint = true;
                } else if !battle_gui.battle_text.timer.is_alive() {
                    battle_gui.battle_text.timer.spawn();
                }
                battle_gui.battle_text.timer.update(delta);
            }
        } else {
            battle_gui.battle_text.spawn();
            update_faint(
                &mut battle_gui.battle_text,
                &battle.opponent().pokemon.data.name,
            );
        }
    }
}

fn update_text(text: &mut DynamicText, pokemon: &String, pmove: &String) {
    text.text = MessageSet::new(
        1,
        TextColor::White,
        vec![vec![pokemon.clone() + " used " + pmove.as_str() + "!"]],
    );
}

fn update_faint(text: &mut DynamicText, pokemon: &String) {
    text.text = MessageSet::new(
        1,
        TextColor::White,
        vec![vec![pokemon.clone() + " fainted!"]],
    );
}
