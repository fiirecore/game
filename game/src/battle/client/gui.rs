use std::rc::Rc;

use deps::{log::warn, tetra::Context};
use util::Entity;
use pokedex::{item::ItemUseType, moves::target::{MoveTarget, MoveTargetInstance, Team}};

use crate::{battle::{BattleData, BattleType, pokemon::{ActivePokemonIndex, BattleMove, BattleParty, BattlePartyPlayerView, BattlePartyView}, ui::panels::{BattlePanel, BattlePanels}}, gui::{bag::BagGui, party::PartyGui}};

use super::BattleClient;

pub struct BattlePlayerGui {

    party: Rc<PartyGui>,
    bag: Rc<BagGui>,

	panel: BattlePanel,

    state: Option<BattlePlayerState>,

    is_wild: bool,

    user: BattlePartyPlayerView,

    moves: Vec<BattleMove>,

    faint: Option<usize>,


}

#[derive(Clone, Copy)]
enum BattlePlayerState {
    Select(usize, bool), // usize = active pokemon num, bool = can run
    Faint(usize),
}

impl BattlePlayerGui {

    pub fn new(ctx: &mut Context, party: Rc<PartyGui>, bag: Rc<BagGui>) -> Self {
        Self {
            party,
            bag,
            panel: BattlePanel::new(ctx),
            state: None,
            is_wild: false,
            user: BattlePartyPlayerView::default(),
            moves: Vec::with_capacity(3),
            faint: None,
        }
    }

    pub fn update(&mut self, ctx: &Context, delta: f32) {
        if let Some(state) = &mut self.state {
            match state {
                BattlePlayerState::Select(active_index, wild) => {
                    match self.user.active.get_mut(*active_index) {
                        Some(pokemon) => match pokemon.as_mut() {
                            Some(pokemon) => match self.panel.alive {
                                true => match self.moves.len() <= *active_index {
                                    true => {
            
                                        // Checks if a move is queued from an action done in the GUI
            
                                        if self.bag.alive() {
                                            self.bag.input(ctx);
                                            if let Some(item) = self.bag.take_selected_despawn() {
                                                let target = match &item.value().usage {
                                                    ItemUseType::Pokeball => ActivePokemonIndex { team: Team::Opponent, active: crate::battle::BATTLE_RANDOM.gen_range(0, self.panel.fight.targets.names.len()) },
                                                    ItemUseType::Script(..) => todo!("user targeting"),
                                                    ItemUseType::None => todo!("make item unusable"),
                                                    // MoveTarget::Opponents => todo!("make none"),
                                                };
                                                self.moves.push(BattleMove::UseItem(item, target));
                                            }
                                        } else if self.party.alive() {
                                            self.party.input(ctx);
                                            self.party.update(delta);
                                            if let Some(selected) = self.party.take_selected() {
                                                self.party.despawn();
                                                self.moves.push(BattleMove::Switch(selected));
                                            }
                                        } else {
                                            if let Some(panels) = self.panel.input(ctx, pokemon) {
                                                match panels {
                                                    BattlePanels::Main => {
                                                        match self.panel.battle.cursor {
                                                            0 => self.panel.active = BattlePanels::Fight,
                                                            1 => self.bag.spawn(),
                                                            2 => crate::battle::ui::battle_party_player_view_gui(&self.party, &self.user, true),
                                                            3 => if *wild {
                                                                // closer.spawn(self, &mut gui.text);
                                                                todo!()
                                                                // self.state = BattleState::End; // To - do: "Got away safely!" - run text and conditions
                                                            },
                                                            _ => unreachable!(),
                                                        }
                                                    }
                                                    BattlePanels::Fight => match pokemon.moves.get(self.panel.fight.moves.cursor) {
                                                        Some(instance) => match instance.get() {
                                                            Some(move_ref) => self.moves.push(BattleMove::Move(
                                                                self.panel.fight.moves.cursor,
                                                                match move_ref.value().target {
                                                                    MoveTarget::User => MoveTargetInstance::User,
                                                                    MoveTarget::Opponent => MoveTargetInstance::Opponent(self.panel.fight.targets.cursor),
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
                                        self.panel.despawn();
                                    }
                                }
                                false => {
                                    self.panel.user(pokemon);
                                }
                            },
                            None => *active_index += 1,
                        },
                        None => {
                            self.panel.despawn();
                        },
                    }
                },
                BattlePlayerState::Faint(active) => match self.party.alive() {
                    true => {
                        self.party.input(ctx);
                        self.party.update(delta);
                        if let Some(selected) = self.party.take_selected() {
                            if self.user.pokemon[selected].as_ref().map(|instance| !instance.value().fainted()).unwrap_or_default() {
                                // user.queue_replace(index, selected);
                                self.party.despawn();
                                self.faint = Some(selected);
                            }
                        }
                    },
                    false => crate::battle::ui::battle_party_player_view_gui(&self.party, &self.user, false)
                }
            }
        }
    }

}

impl BattleClient for BattlePlayerGui {

    fn begin(&mut self, data: &BattleData) {
        self.is_wild = data.battle_type == BattleType::Wild;
    }

    fn start_moves(&mut self, user: BattlePartyPlayerView, targets: BattlePartyView) {
        self.user = user;
        self.state = Some(BattlePlayerState::Select(0, self.is_wild));
        self.panel.target(&targets);
    }

    fn wait_moves(&mut self) -> Option<Vec<BattleMove>> {
        self.state.map(|state| match state {
            BattlePlayerState::Select(index, _) => (index >= self.user.active.len()).then(|| {
                self.state = None;
                self.moves.drain(0..self.moves.len()).collect()
            }),
            _ => unreachable!(),
        }).flatten()
    }

    fn start_faint(&mut self, active: usize) {
        if let Some(pokemon) = self.user.active[active].as_mut() {
            pokemon.current_hp = 0;
        }
        self.state = Some(BattlePlayerState::Faint(active));
    }

    fn wait_faint(&mut self) -> Option<usize> {
        self.faint.take()
    }

    fn draw(&self, ctx: &mut Context) {
        if self.party.alive() {
            self.party.draw(ctx);
        } else if self.bag.alive() {
            self.bag.draw(ctx);
        } else {
            self.panel.draw(ctx);
        }
    }

}