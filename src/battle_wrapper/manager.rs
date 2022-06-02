use rand::{prelude::SmallRng, RngCore, SeedableRng};
use std::ops::Deref;

use battlecli::battle::{
    default_engine::{scripting::MoveScripts, EngineMoves},
    pokemon::PokemonIdentifier,
    prelude::{
        Battle, BattleAi, BattleData, BattleType, DefaultEngine, EndMessage, PlayerData,
        PlayerSettings,
    },
};
use worldcli::{engine::text::MessagePage, worldlib::character::player::InitPlayerCharacter};

use crate::{
    command::CommandProcessor,
    engine::{
        graphics::{Color, CreateDraw, Graphics},
        App, Plugins,
    },
    pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex},
};

use worldcli::worldlib::map::battle::BattleId;

use crate::pokengine::PokedexClientData;

use firecore_battle_engine::{BattleGuiData, BattlePlayerGui, BattleTrainer};

use super::GameBattleWrapper;

mod command;
pub mod transitions;

use transitions::managers::transition::BattleScreenTransitionManager;

pub struct BattleManager<
    D: Deref<Target = PokedexClientData> + Clone,
    P: Deref<Target = Pokemon> + Clone,
    M: Deref<Target = Move> + Clone,
    I: Deref<Target = Item> + Clone,
> {
    state: BattleManagerState,

    engine: DefaultEngine,

    battle: Option<GameBattleWrapper<P, M, I>>,

    btl: BattleGuiData,

    transition: BattleScreenTransitionManager,

    player: BattlePlayerGui<BattleId, D, P, M, I>,

    seed: u64,
    random: SmallRng,

    commands: CommandProcessor,

    pub finished: bool,
}

#[derive(Debug)]
pub enum TransitionState {
    Begin, // runs on spawn methods
    Run,
    End, // spawns next state and goes back to beginning
}

impl Default for TransitionState {
    fn default() -> Self {
        Self::Begin
    }
}

#[derive(Debug)]
pub enum BattleManagerState {
    Begin,
    Transition,
    Battle,
}

impl Default for BattleManagerState {
    fn default() -> Self {
        Self::Begin
    }
}

impl<
        D: Deref<Target = PokedexClientData> + Clone,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > BattleManager<D, P, M, I>
{
    pub fn new(
        gfx: &mut Graphics,
        btl: BattleGuiData,
        dex: D,
        data: (EngineMoves, MoveScripts),
        commands: CommandProcessor,
    ) -> Result<Self, String> {
        let mut engine = DefaultEngine::new::<BattleId, SmallRng>();

        engine.moves = data.0;

        engine.scripting.moves = data.1;

        Ok(Self {
            state: BattleManagerState::default(),

            battle: None,

            transition: BattleScreenTransitionManager::new(gfx)?,

            player: BattlePlayerGui::new(dex.clone(), &btl),

            engine,

            seed: 0,

            random: SmallRng::seed_from_u64(0),

            commands,

            finished: false,
            btl,
        })
    }

    pub fn battle(
        &mut self,
        pokedex: &impl Dex<Pokemon, Output = P>,
        movedex: &impl Dex<Move, Output = M>,
        itemdex: &impl Dex<Item, Output = I>,
        player: &InitPlayerCharacter<P, M, I>,
    ) -> bool {
        // add battle type parameter
        self.finished = false;
        self.state = BattleManagerState::default();
        self.player.reset();
        let entry = match player.state.battle.battling.as_ref() {
            Some(entry) => entry,
            None => return false,
        };
        let empty = player.trainer.party.is_empty() || entry.party.is_empty();
        (!(empty ||
            // Checks if player has any pokemon in party that aren't fainted (temporary)
            !player
                .trainer
                .party
                .iter()
                .any(|pokemon| !(pokemon.fainted()))))
        .then(|| {
            let player = PlayerData {
                id: BattleId::Player,
                name: Some(player.character.name.clone()),
                party: player
                    .trainer
                    .party
                    .iter()
                    .cloned()
                    .map(|p| p.uninit())
                    .collect(),
                bag: player
                    .trainer
                    .bag
                    .iter()
                    .cloned()
                    .map(|i| i.uninit())
                    .collect::<Vec<_>>()
                    .into(),
                settings: PlayerSettings { gains_exp: true },
                endpoint: Box::new(self.player.endpoint().clone()),
                trainer: None,
            };

            let ai = BattleAi::new(SmallRng::seed_from_u64(self.random.next_u64()));

            let ai_player = PlayerData {
                id: entry.id,
                name: entry.trainer.as_ref().map(|trainer| trainer.name.clone()),
                party: entry.party.clone(),
                bag: entry
                    .trainer
                    .as_ref()
                    .map(|trainer| trainer.bag.clone())
                    .unwrap_or_default(),
                trainer: entry.trainer.as_ref().map(|t| BattleTrainer {
                    worth: t.worth,
                    texture: t.sprite,
                    defeat: t
                        .defeat
                        .iter()
                        .map(|m| MessagePage {
                            lines: m.lines.clone(),
                            wait: m.wait,
                            color: m.color.map(Into::into),
                        })
                        .collect(),
                }),
                settings: PlayerSettings { gains_exp: false },
                endpoint: Box::new(ai.endpoint().clone()),
            };

            self.battle = Some(GameBattleWrapper {
                battle: Battle::new(
                    BattleData {
                        type_: entry
                            .trainer
                            .as_ref()
                            .map(|trainer| {
                                if trainer.badge.is_some() {
                                    BattleType::GymLeader
                                } else {
                                    BattleType::Trainer
                                }
                            })
                            .unwrap_or(BattleType::Wild),
                        settings: Default::default(),
                    },
                    &mut self.random,
                    entry.active,
                    pokedex,
                    movedex,
                    itemdex,
                    std::iter::once(player).chain(std::iter::once(ai_player)),
                ),
                trainer: entry.trainer.clone(),
                ai,
            });
        });
        self.battle.is_some()
    }

    pub fn update(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        player: &mut InitPlayerCharacter<P, M, I>,
        pokedex: &impl Dex<Pokemon, Output = P>,
        movedex: &impl Dex<Move, Output = M>,
        itemdex: &impl Dex<Item, Output = I>,
    ) {
        if app
            .keyboard
            .was_pressed(worldcli::engine::notan::prelude::KeyCode::F1)
        {
            // exit shortcut
            self.end();
            self.player.reset();
            player.state.battle.battling = None;
            player.character.locked.clear();
            player.character.input_lock.clear();
            return;
        }

        while let Some(command) = self.commands.commands.borrow_mut().pop() {
            match Self::process(command) {
                Ok(command) => match command {
                    command::BattleCommand::Faint(id, index) => {
                        if let Some(battle) = self.battle.as_mut().map(|b| &mut b.battle) {
                            match index {
                                Some(index) => battle.faint(PokemonIdentifier(id, index)),
                                None => battle.remove(&id, EndMessage::Lose),
                            }
                        }
                    }
                    command::BattleCommand::End => todo!(),
                },
                Err(_) => todo!(),
            }
        }
        if let Some(battle) = &mut self.battle {
            self.player
                .process(&mut self.random, &self.btl, pokedex, movedex, itemdex);
            match self.state {
                BattleManagerState::Begin => {
                    self.player.reset_gui();
                    self.state = BattleManagerState::Transition;
                    self.transition.state = TransitionState::Begin;

                    battle.battle.begin();

                    // self.player.on_begin(dex);

                    self.update(app, plugins, player, pokedex, movedex, itemdex);
                }
                BattleManagerState::Transition => match self.transition.state {
                    TransitionState::Begin => {
                        self.transition.begin(
                            app,
                            plugins,
                            self.player.data().unwrap().type_,
                            &battle.trainer,
                        );
                        self.update(app, plugins, player, pokedex, movedex, itemdex);
                    }
                    TransitionState::Run => self.transition.update(app.timer.delta_f32()),
                    TransitionState::End => {
                        self.transition.end();
                        self.state = BattleManagerState::Battle;
                        self.player.start(true);
                        self.update(app, plugins, player, pokedex, movedex, itemdex);
                    }
                },
                BattleManagerState::Battle => {
                    if !battle.battle.finished() {
                        battle.update(
                            &mut self.random,
                            &mut self.engine,
                            // app.timer.delta_f32(),
                            pokedex,
                            movedex,
                            itemdex,
                        );
                    }

                    self.player.update(
                        app,
                        plugins,
                        pokedex,
                        movedex,
                        itemdex,
                        app.timer.delta_f32(),
                    );

                    if self.player.finished() {
                        self.finished = true;
                    }
                }
            }
        }
    }

    pub fn winner(&self) -> Option<&BattleId> {
        self.battle.as_ref().and_then(|b| b.battle.winner())
    }

    pub fn world_active(&self) -> bool {
        matches!(self.state, BattleManagerState::Transition) || self.player.world_active()
    }

    pub fn seed(&mut self, seed: u64) {
        self.seed = seed;
        self.random = SmallRng::seed_from_u64(seed);
    }

    pub fn end(&mut self) {
        self.finished = true;
        match self.state {
            BattleManagerState::Begin => (),
            BattleManagerState::Transition => self.transition.state = TransitionState::Begin,
            BattleManagerState::Battle => {
                if let Some(battle) = self.battle.as_mut() {
                    battle.battle.end(None);
                } else {
                    self.player.forfeit();
                }
            }
        }
    }

    pub fn ui(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        egui: &crate::engine::notan::egui::Context,
    ) {
        if self.battle.is_some() {
            match self.state {
                BattleManagerState::Begin | BattleManagerState::Transition => (),
                BattleManagerState::Battle => self.player.ui(app, plugins, egui),
            }
        }
    }

    pub fn draw(&self, gfx: &mut Graphics) {
        let mut draw = gfx.create_draw();
        let draw = &mut draw;
        draw.set_size(crate::WIDTH, crate::HEIGHT);
        draw.clear(Color::BLACK);
        if self.battle.is_some() {
            match self.state {
                BattleManagerState::Begin => (),
                BattleManagerState::Transition => self.transition.draw(draw),
                BattleManagerState::Battle => self.player.draw(draw),
            }
        }
        gfx.render(draw);
    }
}
