use std::{cell::RefCell, rc::Rc};

use deps::{log::warn, tetra::Context};
use firecore_util::Entity;
use pokedex::{item::ItemUseType, moves::target::{MoveTarget, MoveTargetInstance, Team}};

use crate::{battle::{BattleData, BattleType, pokemon::{ActivePokemonArray, ActivePokemonIndex, BattleMove, BattleParty}, ui::panels::{BattlePanel, BattlePanels}}, gui::{bag::BagGui, party::PartyGui}};

use super::BattlePlayerAction;

#[derive(Clone)]
pub struct BattlePlayerGui {

    pub party: Rc<PartyGui>,
    pub bag: Rc<BagGui>,
	pub panel: Rc<RefCell<BattlePanel>>,

}

impl BattlePlayerAction for BattlePlayerGui {

    fn moves(&mut self, ctx: &Context, delta: f32, data: &BattleData, active_index: &mut usize, user: &mut BattleParty, target: &ActivePokemonArray) -> bool {
        let mut panel = self.panel.borrow_mut();
        match user.active.get_mut(*active_index) {
            Some(active) => match active.pokemon.as_mut() {
                Some(pokemon) => match panel.alive {
                    true => match active.queued_move.is_none() {
                        true => {

                            // Checks if a move is queued from an action done in the GUI

                            if self.bag.alive() {
                                self.bag.input(ctx);
                                if let Some(item) = self.bag.take_selected_despawn() {
                                    let target = match &item.unwrap().usage {
                                        ItemUseType::Pokeball => ActivePokemonIndex { team: Team::Opponent, active: crate::battle::BATTLE_RANDOM.gen_range(0, target.len()) },
                                        ItemUseType::Script(..) => todo!("user targeting"),
                                        ItemUseType::None => todo!("make item unusable"),
                                        // MoveTarget::Opponents => todo!("make none"),
                                    };
                                    active.queued_move = Some(BattleMove::UseItem(item, target));
                                }
                            } else if self.party.alive() {
                                self.party.input(ctx);
                                self.party.update(delta);
                                if let Some(selected) = self.party.take_selected() {
                                    self.party.despawn();
                                    active.queued_move = Some(BattleMove::Switch(selected));
                                }
                            } else {
                                if let Some(panels) = panel.input(ctx, pokemon) {
                                    match panels {
                                        BattlePanels::Main => {
                                            match panel.battle.cursor {
                                                0 => panel.active = BattlePanels::Fight,
                                                1 => self.bag.spawn(),
                                                2 => crate::battle::ui::battle_party_gui(&self.party, user, true),
                                                3 => if data.battle_type == BattleType::Wild {
                                                    // closer.spawn(self, &mut gui.text);
                                                    todo!()
                                                    // self.state = BattleState::End; // To - do: "Got away safely!" - run text and conditions
                                                },
                                                _ => unreachable!(),
                                            }
                                        }
                                        BattlePanels::Fight => match pokemon.moves.get(panel.fight.moves.cursor) {
                                            Some(instance) => match instance.get() {
                                                Some(move_ref) => active.queued_move = Some(BattleMove::Move(
                                                    panel.fight.moves.cursor,
                                                    match move_ref.unwrap().target {
                                                        MoveTarget::User => MoveTargetInstance::User,
                                                        MoveTarget::Opponent => MoveTargetInstance::Opponent(panel.fight.targets.cursor),
                                                        MoveTarget::AllButUser => MoveTargetInstance::AllButUser,
                                                        MoveTarget::Opponents => MoveTargetInstance::Opponents,
                                                    }
                                                )),
                                                None => warn!("Pokemon is out of Power Points for this move!")
                                            }
                                            None => warn!("Could not get move at cursor!"),
                                        }
                                    }
                                }
                            }
                        }
                        false => {
                            *active_index += 1;
                            panel.despawn();
                        }
                    }
                    false => {
                        panel.run(&mut active.last_move, pokemon, target);
                    }
                },
                None => *active_index += 1,
            }
            None => {
                panel.despawn();
                return true;
            }
        }
        false
    }

    fn faint(&mut self, ctx: &Context, delta: f32, data: &BattleData, index: usize, user: &mut BattleParty) -> bool {
        match self.party.alive() {
            true => {
                self.party.input(ctx);
                self.party.update(delta);
                if let Some(selected) = self.party.take_selected() {
                    if user.pokemon[selected].as_ref().map(|instance| !instance.value().fainted()).unwrap_or_default() {
                        user.queue_replace(index, selected);
                        self.party.despawn();
                        return true;
                    }
                }
            },
            false => match user.active[index].pokemon.is_active() {
                true => if user.any_inactive() {
                    crate::battle::ui::battle_party_gui(&self.party, user, false);
                } else {
                    user.remove_pokemon(index);
                    return true;
                },
                false => return true,
            } 
        }
        false
    }

    fn draw(&self, ctx: &mut Context) {
        if self.party.alive() {
            self.party.draw(ctx);
        } else if self.bag.alive() {
            self.bag.draw(ctx);
        } else {
            self.panel.borrow().draw(ctx);
        }
    }

}