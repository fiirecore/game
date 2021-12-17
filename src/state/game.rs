use crate::{
    command::{CommandProcessor, CommandResult},
    saves::PlayerData,
};
use firecore_battle::pokedex::{item::Item, moves::Move, pokemon::Pokemon};
use firecore_battle_gui::{context::BattleGuiContext, pokedex::context::PokedexClientData};
use std::rc::Rc;
use worldlib::serialized::SerializedWorld;

use crate::{split, Receiver, Sender};

use crate::{
    engine::{
        error::ImageError,
        input::keyboard::{down as is_key_down, Key},
        Context,
    },
    game::battle_glue::BattleEntry,
    pokedex::gui::{bag::BagGui, party::PartyGui},
};

use crate::engine::log::warn;

use crate::{battle::BattleManager, world::manager::WorldManager};

use super::{MainStates, StateMessage};

pub enum GameStates {
    World,
    Battle,
}

impl Default for GameStates {
    fn default() -> Self {
        Self::World
    }
}

#[derive(Clone)]
pub enum GameActions {
    Battle(BattleEntry),
    Save,
    Exit,
}

pub struct GameStateManager {
    state: GameStates,

    dex: Rc<PokedexClientData>,

    world: WorldManager,
    battle: BattleManager<&'static Pokemon, &'static Move, &'static Item>,

    pub save: Option<PlayerData>,

    sender: Sender<StateMessage>,
    receiver: Receiver<GameActions>,
}

impl GameStateManager {
    pub(crate) fn new(
        ctx: &mut Context,
        dex: PokedexClientData,
        btl: BattleGuiContext,
        wrld: SerializedWorld,
        sender: Sender<StateMessage>,
    ) -> Result<Self, ImageError> {
        let dex = Rc::new(dex);
        let party = Rc::new(PartyGui::new(&dex));
        let bag = Rc::new(BagGui::new(&dex));

        let (actions, receiver) = split();

        let mut world = WorldManager::new(
            ctx,
            dex.clone(),
            party.clone(),
            bag.clone(),
            actions.clone(),
        )?;

        world.load(ctx, wrld);

        Ok(Self {
            state: GameStates::default(),

            world,
            battle: BattleManager::new(ctx, btl, dex.clone(), party, bag),

            dex,

            save: None,

            sender,
            receiver,
        })
    }

    pub fn seed(&mut self, seed: u64) {
        self.world.seed(seed);
    }
}

impl GameStateManager {
    pub fn start(&mut self, ctx: &mut Context) {
        if let Some(save) = self.save.as_mut() {
            match self.state {
                GameStates::World => self.world.start(ctx, &mut save.character, &mut save.party),
                GameStates::Battle => (),
            }
        }
        // Ok(())
    }

    pub fn end(&mut self) {
        if let Some(save) = self.save.take() {
            self.sender.send(StateMessage::UpdateSave(save));
        }
        // Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context, delta: f32) {
        // Speed game up if spacebar is held down

        let delta = delta
            * if is_key_down(ctx, Key::Space) {
                4.0
            } else {
                1.0
            };

        if let Some(save) = self.save.as_mut() {
            for action in self.receiver.try_iter() {
                match action {
                    GameActions::Battle(entry) => match self.state {
                        GameStates::World => {
                            if self.battle.battle(
                                crate::pokedex(),
                                crate::movedex(),
                                crate::itemdex(),
                                save,
                                entry,
                            ) {
                                self.state = GameStates::Battle;
                            }
                        }
                        GameStates::Battle => warn!("Cannot start new battle, already in one!"),
                    },
                    GameActions::Save => self.sender.send(StateMessage::UpdateSave(save.clone())),
                    GameActions::Exit => self.sender.send(StateMessage::Goto(MainStates::Menu)),
                }
            }

            match self.state {
                GameStates::World => {
                    self.world
                        .update(ctx, delta, &mut save.character, &mut save.party, &save.bag);
                }
                GameStates::Battle => {
                    self.battle.update(
                        ctx,
                        crate::pokedex(),
                        crate::movedex(),
                        crate::itemdex(),
                        delta,
                        save,
                    );
                    if self.battle.finished {
                        save.character.input_frozen = false;
                        save.character.unfreeze();
                        if let Some(winner) = self.battle.winner() {
                            let winner = winner.0.as_ref() == Some(&save.id);
                            let trainer = self.battle.update_data(winner, &mut save.character);
                            self.world.update_world(
                                &mut save.character,
                                &mut save.party,
                                winner,
                                trainer,
                            );
                        }
                        self.state = GameStates::World;
                        self.world.start(ctx, &mut save.character, &mut save.party);
                    }
                }
            }
        } else if self.sender.0.is_empty() {
            self.sender.send(StateMessage::Goto(MainStates::Menu));
        }
        // Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        if let Some(save) = self.save.as_ref() {
            match self.state {
                GameStates::World => self.world.draw(ctx, save),
                GameStates::Battle => {
                    if self.battle.world_active() {
                        self.world.draw(ctx, save);
                    }
                    self.battle.draw(ctx, save);
                }
            }
        }

        // Ok(())
    }
}

impl CommandProcessor for GameStateManager {
    fn process(&mut self, command: CommandResult) {
        match self.state {
            GameStates::World => self.world.process(command),
            GameStates::Battle => (), //self.battle.process(result),
        }
    }
}
