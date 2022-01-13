use std::rc::Rc;

use crate::pokedex::{item::Item, moves::Move, pokemon::Pokemon};

use battlecli::battle::default_engine::{scripting::MoveScripts, EngineMoves};

use crate::engine::music;

use crate::pokengine::{
    PokedexClientData,
};

use battlecli::BattleGuiData;

use worldcli::{
    battle::{BattleMessage, BattleId},
    worldlib::{events::*, serialized::SerializedWorld},
};

use crate::{
    command::{CommandProcessor, CommandResult},
    engine::{
        error::ImageError,
        input::keyboard::{down as is_key_down, Key},
        Context, EngineContext,
    },
    saves::Player,
};

use crate::engine::log::warn;

use crate::battle_wrapper::BattleManager;
use crate::world_wrapper::WorldWrapper;

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
    Battle(BattleMessage),
    CommandError(&'static str),
    Save,
    Exit,
}

pub(super) struct GameStateManager {
    state: GameStates,

    world: WorldWrapper,
    battle: BattleManager<&'static Pokemon, &'static Move, &'static Item>,

    pub save: Option<Player>,

    sender: Sender<StateMessage>,
    receiver: Receiver<GameActions>,
}

impl GameStateManager {
    pub fn new(
        ctx: &mut Context,
        dex: PokedexClientData,
        btl: BattleGuiData,
        wrld: SerializedWorld,
        battle: (EngineMoves, MoveScripts),
        sender: Sender<StateMessage>,
    ) -> Result<Self, ImageError> {
        let dex = Rc::new(dex);

        let (actions, receiver) = split();

        let world = WorldWrapper::new(ctx, dex.clone(), actions, wrld)?;

        Ok(Self {
            state: GameStates::default(),

            world,
            battle: BattleManager::new(ctx, btl, dex, battle),

            save: None,

            sender,
            receiver,
        })
    }

    pub fn seed(&mut self, seed: u64) {
        self.world.seed(seed);
        self.battle.seed(seed);
    }
}

impl GameStateManager {
    pub fn start(&mut self, _ctx: &mut Context) {
        if let Some(save) = self.save.as_mut() {
            save.player.create(self.world.manager.spawn());
            match self.state {
                GameStates::World => self.world.manager.start(save.player.unwrap()),
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

    pub fn update(
        &mut self,
        ctx: &mut Context,
        eng: &mut EngineContext,
        delta: f32,
    ) {
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
                                crate::dex::pokedex(),
                                crate::dex::movedex(),
                                crate::dex::itemdex(),
                                save.player.unwrap(),
                                entry,
                            ) {
                                self.state = GameStates::Battle;
                            }
                        }
                        GameStates::Battle => warn!("Cannot start new battle, already in one!"),
                    },
                    GameActions::Save => self.sender.send(StateMessage::UpdateSave(save.clone())),
                    GameActions::Exit => {
                        music::stop_music(ctx, eng);
                        self.sender.send(StateMessage::Goto(MainStates::Menu));
                    }
                    GameActions::CommandError(error) => {
                        self.sender.send(StateMessage::CommandError(error))
                    }
                }
            }

            match self.state {
                GameStates::World => {
                    self.world
                        .update(ctx, eng, delta, save.player.unwrap());
                }
                GameStates::Battle => {
                    self.battle.update(
                        ctx,
                        eng,
                        crate::dex::pokedex(),
                        crate::dex::movedex(),
                        crate::dex::itemdex(),
                        delta,
                    );
                    if self.battle.finished {
                        let player = save.player.unwrap();
                        if let Some(winner) = self.battle.winner() {
                            let winner = winner == &BattleId::Player;
                            self.world.manager.post_battle(player, winner);
                        }
                        self.state = GameStates::World;
                        self.world.manager.start(player);
                    }
                }
            }
        } else if self.sender.is_empty() {
            self.sender.send(StateMessage::Goto(MainStates::Menu));
        }
        // Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, eng: &mut EngineContext) {
        match self.state {
            GameStates::World => {
                if let Some(player) = self.save.as_ref().map(|p| p.player.as_ref()).flatten() {
                    self.world.draw(ctx, eng, player);
                }
            }
            GameStates::Battle => {
                if self.battle.world_active() {
                    if let Some(player) = self.save.as_ref().map(|p| p.player.as_ref()).flatten() {
                        self.world.draw(ctx, eng, player);
                    }
                }
                self.battle.draw(ctx, eng);
            }
        }
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
