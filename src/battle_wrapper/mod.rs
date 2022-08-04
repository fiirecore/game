use rand::{prelude::SmallRng, RngCore, SeedableRng};
use std::sync::Arc;

use battlecli::battle::{
    pokemon::PokemonIdentifier,
    prelude::{
        Battle, BattleAi, BattleData, BattleType, DefaultEngine, EndMessage, PlayerData,
        PlayerSettings,
    },
};
use worldcli::{
    engine::text::MessagePage,
    pokedex::trainer::InitTrainer,
    worldlib::{character::player::PlayerCharacter, map::battle::BattleEntry},
};

use crate::{
    command::CommandProcessor,
    engine::{
        graphics::{Color, CreateDraw, Graphics},
        App, Plugins,
    },
    pokedex::{item::Item, moves::Move, pokemon::Pokemon, Dex},
    random::GamePseudoRandom,
};

use worldcli::worldlib::map::battle::BattleId;

use firecore_battle_engine::{
    pokengine::texture::{ItemTextures, PokemonTextures, TrainerGroupTextures},
    BattlePlayerGui, BattleTrainer, InitBattleGuiTextures,
};

mod command;
pub mod transition;

use transition::manager::BattleScreenTransitionManager;

use self::transition::transitions::BattleTransitions;

pub struct BattleWrapper {
    state: BattleManagerState,
    engine: DefaultEngine,
    battle: Option<Battle<BattleId, BattleTrainer>>,
    player: BattlePlayerGui<BattleId>,
    ai: Vec<BattleAi<BattleId, SmallRng, BattleTrainer>>,
    transition: BattleScreenTransitionManager,
    random: GamePseudoRandom,
    commands: CommandProcessor,
    pokedex: Arc<Dex<Pokemon>>,
    movedex: Arc<Dex<Move>>,
    itemdex: Arc<Dex<Item>>,
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

impl BattleWrapper {
    pub fn new(
        gfx: &mut Graphics,
        pokedex: Arc<Dex<Pokemon>>,
        movedex: Arc<Dex<Move>>,
        itemdex: Arc<Dex<Item>>,
        pokemon: Arc<PokemonTextures>,
        items: Arc<ItemTextures>,
        trainers: Arc<TrainerGroupTextures>,
        btl: InitBattleGuiTextures,
        engine: DefaultEngine,
        commands: CommandProcessor,
    ) -> Result<Self, String> {
        Ok(Self {
            state: Default::default(),
            battle: Default::default(),
            ai: Default::default(),
            transition: BattleScreenTransitionManager::new(gfx)?,
            player: BattlePlayerGui::new(pokemon, items, trainers, &btl),
            engine,
            random: Default::default(),
            commands,
            pokedex,
            movedex,
            itemdex,
        })
    }

    pub fn seed(&mut self, seed: u64) {
        self.random.seed(seed);
    }

    pub fn try_battle(&mut self, player: &PlayerCharacter, trainer: &InitTrainer) -> bool {
        let entry = match player.battle.battling.as_ref() {
            Some(entry) => entry,
            None => return false,
        };

        match trainer.party.iter().any(|pokemon| !(pokemon.fainted())) || entry.party.is_empty() {
            true => {
                self.battle(&player.name, trainer, entry);
                true
            }
            false => false,
        }
    }

    pub fn battle(&mut self, name: &str, trainer: &InitTrainer, entry: &BattleEntry) {
        // add battle type parameter
        self.state = BattleManagerState::default();
        self.player.reset();
        let player = PlayerData {
            id: BattleId::Player,
            name: Some(name.to_owned()),
            party: trainer.party.iter().cloned().map(|p| p.uninit()).collect(),
            bag: trainer
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
                        theme: (),
                    })
                    .collect(),
            }),
            settings: PlayerSettings { gains_exp: false },
            endpoint: Box::new(ai.endpoint().clone()),
        };

        self.battle = Some(Battle::new(
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
            &self.pokedex,
            &self.movedex,
            &self.itemdex,
            [player, ai_player].into_iter(),
        ));

        self.transition.current = entry
            .trainer
            .as_ref()
            .map(|e| BattleTransitions::from(e.transition))
            .unwrap_or_default();
    }

    pub fn update(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        player: &mut PlayerCharacter,
    ) -> bool {
        if app
            .keyboard
            .was_pressed(worldcli::engine::notan::prelude::KeyCode::F1)
        {
            // exit shortcut
            self.end();
            self.player.reset();
            player.battle.battling = None;
            player.character.locked.clear();
            player.character.input_lock.clear();
            return true;
        }

        while let Some(command) = self.commands.commands.borrow_mut().pop() {
            match Self::process(command) {
                Ok(command) => match command {
                    command::BattleCommand::Faint(id, index) => {
                        if let Some(battle) = self.battle.as_mut() {
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
            self.player.process(
                &mut self.random,
                &self.pokedex,
                &self.movedex,
                &self.itemdex,
            );
            match self.state {
                BattleManagerState::Begin => {
                    self.player.reset_gui();

                    battle.begin();

                    self.state = BattleManagerState::Transition;

                    self.transition
                        .begin(app, plugins, self.player.data().unwrap().type_);

                    // self.player.on_begin(dex);

                    self.update(app, plugins, player);
                }
                BattleManagerState::Transition => {
                    if self.transition.update(app.timer.delta_f32()) {
                        self.state = BattleManagerState::Battle;
                        self.player.start(true);
                        self.update(app, plugins, player);
                    }
                }
                BattleManagerState::Battle => {
                    if !battle.finished() {
                        battle.update(
                            &mut self.random,
                            &mut self.engine,
                            &self.movedex,
                            &self.itemdex,
                        );
                    }

                    self.player.update(
                        app,
                        plugins,
                        &self.pokedex,
                        &self.movedex,
                        &self.itemdex,
                        app.timer.delta_f32(),
                    );

                    return self.player.finished();
                }
            }
        }
        false
    }

    pub fn winner(&self) -> Option<&BattleId> {
        self.battle.as_ref().and_then(|b| b.winner())
    }

    pub fn world_active(&self) -> bool {
        matches!(self.state, BattleManagerState::Transition) || self.player.world_active()
    }

    pub fn end(&mut self) {
        match self.state {
            BattleManagerState::Begin => (),
            BattleManagerState::Transition => (),
            BattleManagerState::Battle => {
                if let Some(battle) = self.battle.as_mut() {
                    battle.end(None);
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
    ) -> bool {
        if self.battle.is_some() {
            match self.state {
                BattleManagerState::Begin | BattleManagerState::Transition => (),
                BattleManagerState::Battle => self.player.ui(app, plugins, egui),
            }
        }
        self.battle.is_none()
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
