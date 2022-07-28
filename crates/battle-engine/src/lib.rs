pub extern crate firecore_battle as battle;
pub extern crate firecore_pokedex_engine as pokengine;

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    sync::Arc,
};

use rand::Rng;
use serde::{Deserialize, Serialize};

use pokengine::{
    engine::{
        egui,
        graphics::{Color, Draw},
        log::{self, debug, warn},
        math::Vec2,
        text::MessagePage,
        App, Plugins,
    },
    pokedex::{
        item::{usage::ItemExecution, Item, ItemCategory},
        moves::Move,
        pokemon::Pokemon,
        types::Effective,
        Dex, Money,
    },
    PokedexClientData, TrainerGroupId,
};

use battle::{
    endpoint::{MpscClient, MpscEndpoint},
    message::{ClientMessage, ServerMessage},
    moves::{damage::ClientDamage, ClientMove, ClientMoveAction, MoveCancel},
    party::PlayerParty,
    pokemon::Indexed,
    prelude::{
        BattleData, ClientPlayerData, CommandAction, EndMessage, FailedAction, StartableAction,
    },
};

use self::{
    players::{GuiLocalPlayer, GuiRemotePlayers},
    state::BattlePlayerState,
    transition::{closer::BattleCloser, opener::BattleOpener},
    ui::{panels::BattleAction, BattleGui},
    view::PlayerView,
};

mod context;
pub use context::*;

mod ui;
mod view;

mod action;
mod players;
mod state;
mod transition;

use action::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleTrainer {
    pub worth: Money,
    pub texture: TrainerGroupId,
    pub defeat: Vec<MessagePage<[f32; 4], ()>>,
}

pub struct BattlePlayerGui<ID: Eq + Hash + Display + Clone> {
    state: BattlePlayerState<ID>,
    should_select: bool,
    should_end: Option<Option<ID>>,

    opener: BattleOpener,
    // introduction: BattleIntroductionManager,
    closer: BattleCloser,
    gui: BattleGui<ID>,

    local: Option<GuiLocalPlayer<ID>>,
    remotes: GuiRemotePlayers<ID>,

    client: MpscClient<ID, BattleTrainer>,
    endpoint: MpscEndpoint<ID, BattleTrainer>,
}

impl<ID: Clone + Debug + Display + Hash + Eq> BattlePlayerGui<ID> {
    pub fn new(dex: Arc<PokedexClientData>, btl: &BattleGuiData) -> Self
    where
        ID: Default,
    {
        let (client, endpoint) = battle::endpoint::create();

        Self {
            gui: BattleGui::new(dex.clone(), btl),
            opener: BattleOpener::new(dex.clone(), btl),
            // introduction: BattleIntroductionManager::new(btl),
            closer: BattleCloser::new(dex),
            state: BattlePlayerState::WaitToStart,
            should_select: false,
            should_end: None,
            local: None,
            remotes: Default::default(),
            client,
            endpoint,
        }
    }

    pub fn endpoint(&self) -> &MpscEndpoint<ID, BattleTrainer> {
        &self.endpoint
    }

    pub fn winner(&self) -> Option<Option<&ID>> {
        if let BattlePlayerState::End(w) = &self.state {
            Some(w.as_ref())
        } else {
            None
        }
    }

    pub fn battling(&self) -> bool {
        !matches!(
            self.state,
            BattlePlayerState::WaitToStart | BattlePlayerState::Opening
        )
    }

    pub fn start(&mut self, transition: bool) {
        self.state = match transition {
            true => BattlePlayerState::Opening,
            false => BattlePlayerState::WaitToSelect,
        };
    }

    pub fn forfeit(&mut self) {
        self.client.send(ClientMessage::Forfeit);
    }

    fn begin_with<R: Rng>(
        &mut self,
        client: ClientPlayerData<ID, BattleTrainer>,
        random: &mut R,
        pokedex: &Dex<Pokemon>,
        movedex: &Dex<Move>,
        itemdex: &Dex<Item>,
    ) {
        self.remotes = GuiRemotePlayers {
            current: 0,
            players: client
                .remotes
                .into_iter()
                .map(|player| {
                    let player = PlayerParty {
                        id: player.id,
                        name: player.name,
                        active: player.active,
                        pokemon: player
                            .pokemon
                            .into_iter()
                            .map(|u| u.map(|u| u.init(pokedex).unwrap()))
                            .collect(),
                        trainer: player.trainer,
                    };
                    (player.id.clone(), player)
                })
                .collect(),
        };

        let player = battle::party::PlayerParty {
            name: client.local.name,
            id: client.local.id,
            active: client.local.active,
            pokemon: client
                .local
                .pokemon
                .into_iter()
                .flat_map(|p| p.init(random, pokedex, movedex, itemdex))
                .collect(),
            trainer: client.local.trainer,
        };

        let local = GuiLocalPlayer {
            player,
            selecting: None,
            bag: client.bag.init(itemdex).unwrap_or_default(),
            data: client.data,
        };

        self.local = Some(local);
    }

    pub fn process<R: Rng>(
        &mut self,
        random: &mut R,
        pokedex: &Dex<Pokemon>,
        movedex: &Dex<Move>,
        itemdex: &Dex<Item>,
    ) {
        while let Ok(message) = self.client.receiver.try_recv() {
            match message {
                ServerMessage::Begin(client) => {
                    self.begin_with(client, random, pokedex, movedex, itemdex)
                }
                message => match self.local.as_mut() {
                    Some(local) => match message {
                        ServerMessage::Start(action) => match action {
                            StartableAction::Selecting => {
                                self.should_select = true;
                                self.gui.panel.despawn();
                            }
                            StartableAction::Turns(queue) => {
                                self.state = BattlePlayerState::Moving(MoveQueue {
                                    actions: queue
                                        .into_iter()
                                        .map(|a| Indexed(a.0, BattleClientGuiAction::Action(a.1)))
                                        .collect(),
                                    current: None,
                                });
                            }
                        },
                        ServerMessage::Replace(Indexed(pokemon, new)) => match &mut self.state {
                            BattlePlayerState::Moving(queue) => {
                                queue.actions.push_back(Indexed(
                                    pokemon,
                                    BattleClientGuiAction::Replace(Some(new)),
                                ));
                            }
                            _ => match pokemon.team() == local.player.id() {
                                true => {
                                    local.player.replace(pokemon.index(), Some(new));
                                }
                                false => {
                                    if let Some(player) =
                                        self.remotes.players.get_mut(pokemon.team())
                                    {
                                        player.replace(pokemon.index(), Some(new));
                                    }
                                }
                            },
                        },
                        ServerMessage::AddRemote(Indexed(target, unknown)) => {
                            if let Some((player, unknown)) = unknown
                                .init(pokedex)
                                .map(|unknown| {
                                    self.remotes
                                        .players
                                        .get_mut(target.team())
                                        .map(|party| (party, unknown))
                                })
                                .flatten()
                            {
                                player.add(target.index(), Some(unknown));
                            } else {
                                warn!("Could not initialize remote pokemon at {:?}", target);
                            }
                        }
                        ServerMessage::Ping(..) => (),
                        ServerMessage::Fail(action) => match action {
                            FailedAction::Move(i) | FailedAction::Switch(i) => match &self.state {
                                BattlePlayerState::Select => {
                                    self.gui.panel.despawn();
                                    local.selecting = Some(0..local.player.active.len());
                                    self.state = BattlePlayerState::Select;
                                }
                                _ => {
                                    local.selecting = Some(i..i + 1);
                                    self.state = BattlePlayerState::Select;
                                }
                            },
                            FailedAction::Replace(index) => {
                                debug!("cannot replace pokemon at active index {}", index);
                            }
                        },
                        ServerMessage::Catch(instance) => {
                            match instance.init(random, pokedex, movedex, itemdex) {
                                Some(instance) => {
                                    local.player.pokemon.push(instance);
                                }
                                None => warn!("Could not initialize caught pokemon."),
                            }
                        }
                        ServerMessage::PlayerEnd(id, message) => match &id == local.player.id() {
                            true => {
                                self.state = match message {
                                    EndMessage::Run => BattlePlayerState::Closing(Some(id)),
                                    message => BattlePlayerState::Lose(message),
                                }
                            }
                            false => {
                                self.remotes.players.remove(&id);
                            }
                        },
                        ServerMessage::GameEnd(winner) => {
                            self.should_end = Some(winner);
                        }
                        ServerMessage::Begin(..) => unreachable!(),
                        ServerMessage::Command(command) => match command {
                            CommandAction::Faint(id) => {
                                let is_local = id.team() == local.player.id();
                                match is_local {
                                    true => {
                                        if let Some(pokemon) =
                                            local.player.pokemon.get_mut(id.index())
                                        {
                                            pokemon.hp = 0;
                                            self.gui.text.on_faint(
                                                local.data.type_.is_wild(),
                                                false,
                                                pokemon.name(),
                                            );
                                            let pid = pokemon.pokemon.id;
                                            drop(pokemon);
                                            if let Some(active) = local.player.index(id.index()) {
                                                self.gui.pokemon.faint((false, active), Some(&pid))
                                            }
                                        }
                                    }
                                    false => {
                                        if let Some(remote) =
                                            self.remotes.players.get_mut(id.team())
                                        {
                                            if let Some(pokemon) = remote
                                                .pokemon
                                                .get_mut(id.index())
                                                .map(Option::as_mut)
                                                .flatten()
                                            {
                                                pokemon.hp = 0.0;
                                                self.gui.text.on_faint(
                                                    local.data.type_.is_wild(),
                                                    false,
                                                    pokemon.name(),
                                                );
                                                let pid = pokemon.pokemon.id;
                                                drop(pokemon);
                                                if let Some(active) = remote.index(id.index()) {
                                                    self.gui
                                                        .pokemon
                                                        .faint((false, active), Some(&pid))
                                                }
                                            }
                                        }
                                        {}
                                    }
                                }
                            }
                        },
                    },
                    None => self.forfeit(),
                },
            }
        }
    }

    pub fn update(
        &mut self,
        app: &mut App,
        plugins: &mut Plugins,
        pokedex: &Dex<Pokemon>,
        movedex: &Dex<Move>,
        itemdex: &Dex<Item>,
        delta: f32,
    ) {
        match &mut self.state {
            BattlePlayerState::WaitToStart => (),
            state => match self.local.as_mut() {
                Some(local) => match state {
                    BattlePlayerState::WaitToStart => unreachable!(),
                    BattlePlayerState::Opening => {
                        match self.opener.alive() {
                            true => {
                                if self.opener.update(app, &mut self.gui, local, &self.remotes) {
                                    self.opener.reset();
                                    self.state = BattlePlayerState::WaitToSelect;
                                    // self.update(app, plugins, dex, pokedex, movedex, itemdex, delta);
                                }
                            }
                            false => self.opener.spawn(&mut self.gui, local, &self.remotes),
                        }
                    }
                    BattlePlayerState::WaitToSelect => {
                        let active = local
                            .player
                            .active_iter()
                            .find(|(.., p)| p.fainted())
                            .map(|(a, ..)| a);
                        match active {
                            Some(active) => match local.player.any_inactive() {
                                true => match self.gui.panel.alive() {
                                    true => {
                                        if !self.gui.actions.is_empty() {
                                            if let BattleAction::Action(
                                                battle::moves::BattleMove::Switch(index),
                                            ) = self.gui.actions.remove(0)
                                            {
                                                self.client.send(ClientMessage::ReplaceFaint(
                                                    active, index,
                                                ));
                                                local.player.replace(active, Some(index));
                                            }
                                        }
                                    }
                                    false => self.gui.panel.spawn(true),
                                },
                                false => local.player.replace(active, None),
                            },
                            None => {
                                if self.should_select {
                                    self.should_select = false;
                                    local.selecting = Some(0..local.player.active.len());
                                    self.state = BattlePlayerState::Select;
                                }
                            }
                        }
                    }
                    BattlePlayerState::Select => {
                        self.gui.bounce.update(delta);
                        if !self.gui.panel.alive() {
                            self.gui.panel.spawn(false);
                        }
                        match local.selecting.as_mut() {
                            Some(range) => {
                                let current = range.start;
                                if !self.gui.actions.is_empty() {
                                    let action = self.gui.actions.remove(0);
                                    match action {
                                        BattleAction::Action(action) => {
                                            self.client.send(ClientMessage::Move(current, action));
                                        }
                                        BattleAction::Forfeit => {
                                            self.client.send(ClientMessage::Forfeit);
                                        }
                                    }
                                    range.next();
                                }
                            }
                            None => unreachable!(),
                        }
                    }
                    BattlePlayerState::Moving(queue) => {
                        match &mut queue.current {
                            None => {
                                match queue.actions.pop_front() {
                                    None => {
                                        self.state = match Option::take(&mut self.should_end) {
                                            Some(winner) => BattlePlayerState::Closing(winner),
                                            None => BattlePlayerState::WaitToSelect,
                                        };
                                    }
                                    Some(Indexed(user_id, action)) => {
                                        let user = match user_id.team() == local.player.id() {
                                            true => {
                                                Some(&mut local.player as &mut dyn PlayerView<ID>)
                                            }
                                            false => self
                                                .remotes
                                                .players
                                                .get_mut(user_id.team())
                                                .map(|p| p as _),
                                        };

                                        match user {
                                            Some(user) => {
                                                if user.active(user_id.index()).is_some() {
                                                    if let Some(action) = match action {
                                                        BattleClientGuiAction::Action(action) => {
                                                            match action {
                                                                ClientMove::<ID>::Move(
                                                                    pokemon_move,
                                                                    pp,
                                                                    targets,
                                                                ) => {
                                                                    // log::trace!("set current: client move");

                                                                    match movedex
                                                                        .try_get(&pokemon_move)
                                                                    {
                                                                        Some(pokemon_move) => {
                                                                            {
                                                                                if let Some(
                                                                                    user_active,
                                                                                ) = user
                                                                                    .active_mut(
                                                                                        user_id
                                                                                            .index(
                                                                                            ),
                                                                                    )
                                                                                {
                                                                                    user_active
                                                                                        .decrement_pp(
                                                                                            &pokemon_move,
                                                                                            pp,
                                                                                        );

                                                                                    self.gui.text.on_move(
                                                                                        &pokemon_move,
                                                                                        user_active.name(),
                                                                                    );
                                                                                }

                                                                                // user_active.decrement_pp(pp);
                                                                            }

                                                                            drop(user);

                                                                            for Indexed(
                                                                                target_id,
                                                                                action,
                                                                            ) in &targets
                                                                            {
                                                                                let is_local =
                                                                                    target_id
                                                                                        .team()
                                                                                        == local
                                                                                            .player
                                                                                            .id();
                                                                                let target = match is_local {
                                                                                true => local.player.active_mut(target_id.index()).map(|p| p as &mut dyn view::GuiPokemonView),
                                                                                false => self.remotes.players.get_mut(target_id.team()).map(|remote| {
                                                                                    remote.active_mut(target_id.index()).map(|p| p as _)
                                                                                }).flatten(),
                                                                            };

                                                                                if let Some(
                                                                                    target,
                                                                                ) = target
                                                                                {
                                                                                    match *action {
                                                                                        ClientMoveAction::SetHP(result) => {
                                                                                            target.set_hp(result.damage());
                                                                                            if let ClientDamage::Result(result) = result {
                                                                                                match result.damage > 0.0 {
                                                                                                    true => self.gui.pokemon.flicker((is_local, target_id.index())),
                                                                                                    false => {
                                                                                                        self.gui.text.on_faint(local.data.type_.is_wild(), is_local, target.name());
                                                                                                        self.gui.pokemon.faint((
                                                                                                            is_local,
                                                                                                            target_id.index(),
                                                                                                        ), Some(&target.base().pokemon().id))
                                                                                                    },
                                                                                                }
                                                                                                if result.effective != Effective::Effective {
                                                                                                    self.gui.text.on_effective(&result.effective)
                                                                                                }
                                                                                                if result.crit {
                                                                                                    self.gui.text.on_crit();
                                                                                                }
                                                                                            }
                                                                                        },
                                                                                        ClientMoveAction::Error => self.gui.text.on_fail(vec![format!("{} cannot use move", target.name()), format!("{}, as there was an error.", pokemon_move.name)]),
                                                                                        ClientMoveAction::Miss => self.gui.text.on_miss(target.name()),
                                                                                        ClientMoveAction::SetExp(experience, level) => {
                                                                                            let previous = target.level();
                                                                                            target.set_level(level);
                                                                                            target.set_exp(experience);
                                                                                            if let Some(user_pokemon) = target.instance() {
                                                                                                let moves = user_pokemon.on_level_up(movedex, previous).flat_map(|id| movedex.try_get(id)).cloned().collect();
                                                                                                queue.actions.push_front(Indexed(target_id.clone(), BattleClientGuiAction::SetExp(previous, experience, moves)));
                                                                                            }
                                                                                        }
                                                                                        ClientMoveAction::Cancel(reason) => match reason {
                                                                                            MoveCancel::Flinch => self.gui.text.on_flinch(target.name()),
                                                                                            MoveCancel::Sleep => self.gui.text.on_sleep(target.name()),
                                                                                            MoveCancel::Paralysis => self.gui.text.on_paralysis(target.name()),
                                                                                        } ,
                                                                                        ClientMoveAction::AddStat(stat, stage) => self.gui.text.on_stat_stage(target.name(), stat, stage),
                                                                                        ClientMoveAction::Ailment(ailment) => {
                                                                                            target.set_ailment(ailment);
                                                                                            self.gui.text.on_status(target.name(), ailment.map(|a| a.ailment));
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }

                                                                            Some(BattleClientGuiCurrent::Move(targets))
                                                                        }
                                                                        None => None,
                                                                    }
                                                                }
                                                                ClientMove::UseItem(Indexed(
                                                                    target,
                                                                    item,
                                                                )) => {
                                                                    if let Some(item) =
                                                                        itemdex.try_get(&item)
                                                                    {
                                                                        if let Some(pokemon) = match item
                                                                            .category
                                                                        {
                                                                            ItemCategory::Pokeballs => self
                                                                                .remotes.players
                                                                                .get(target.team())
                                                                                .map(|p| {
                                                                                    p.active(
                                                                                        target.index(),
                                                                                    )
                                                                                })
                                                                                .flatten()
                                                                                .map(|p| p as _),
                                                                            _ => match &item.usage.execute {
                                                                                ItemExecution::Actions(
                                                                                    ..,
                                                                                ) => user
                                                                                    .active(target.index()),
                                                                                ItemExecution::None => None,
                                                                            },
                                                                        } {
                                                                            if let ItemCategory::Pokeballs =
                                                                                &item.category
                                                                            {
                                                                                // self.messages.push(ClientMessage::RequestPokemon(index));
                                                                                queue.actions.push_front(Indexed(target.clone(), BattleClientGuiAction::Catch));
                                                                            }
                                                                            self.gui.text.on_item(
                                                                                pokemon.name(),
                                                                                &item,
                                                                            );
                                                                        }
                                                                        Some(
                                                                            BattleClientGuiCurrent::UseItem(
                                                                                target,
                                                                            ),
                                                                        )
                                                                    } else {
                                                                        None
                                                                    }
                                                                }
                                                                ClientMove::Switch(index) => {
                                                                    let coming = user
                                                                        .pokemon(index)
                                                                        .map(|v| v.name())
                                                                        .unwrap_or("Unknown");
                                                                    self.gui.text.on_switch(
                                                                        user.active(
                                                                            user_id.index(),
                                                                        )
                                                                        .map(|v| v.name())
                                                                        .unwrap_or("Unknown"),
                                                                        coming,
                                                                    );
                                                                    Some(BattleClientGuiCurrent::Switch(
                                                                        index,
                                                                    ))
                                                                }
                                                            }
                                                        }
                                                        BattleClientGuiAction::Catch => {
                                                            match self
                                                                .remotes
                                                                .players
                                                                .get_mut(user_id.team())
                                                            {
                                                                Some(remote) => {
                                                                    if let Some(pokemon) = remote
                                                                        .active(user_id.index())
                                                                    {
                                                                        self.gui.text.on_catch(
                                                                            pokemon
                                                                                .as_ref()
                                                                                .map(|p| p.name())
                                                                                .unwrap_or(
                                                                                    "Unknown",
                                                                                ),
                                                                        );
                                                                    }
                                                                    // if let Some(pokemon) = pokemon {
                                                                    remote.replace(
                                                                        user_id.index(),
                                                                        None,
                                                                    );
                                                                    // }
                                                                    Some(BattleClientGuiCurrent::Catch)
                                                                }
                                                                None => None,
                                                            }
                                                        }
                                                        BattleClientGuiAction::Replace(new) => {
                                                            self.gui.text.on_replace(
                                                                user.name(),
                                                                new.map(|index| {
                                                                    user.pokemon(index)
                                                                        .map(|v| v.name())
                                                                })
                                                                .flatten(),
                                                            );
                                                            user.replace(user_id.index(), new);
                                                            Some(BattleClientGuiCurrent::Replace(
                                                                user_id.index(),
                                                                false,
                                                            ))
                                                        }
                                                        // To - do: experience spreading
                                                        BattleClientGuiAction::SetExp(
                                                            previous,
                                                            experience,
                                                            moves,
                                                        ) => match user.active_mut(user_id.index())
                                                        {
                                                            Some(pokemon) => {
                                                                self.gui.text.on_gain_exp(
                                                                    pokemon.name(),
                                                                    experience,
                                                                    pokemon.level(),
                                                                );
                                                                queue.actions.push_front(Indexed(
                                                                    user_id.clone(),
                                                                    BattleClientGuiAction::LevelUp(
                                                                        moves,
                                                                    ),
                                                                ));
                                                                Some(BattleClientGuiCurrent::SetExp)
                                                            }
                                                            None => None,
                                                        },
                                                        BattleClientGuiAction::LevelUp(moves) => {
                                                            match user
                                                                .active_mut(user_id.index())
                                                                .map(|v| v.instance())
                                                                .flatten()
                                                            {
                                                                Some(instance) => {
                                                                    match moves.is_empty() {
                                                                        false => {
                                                                            // **IMPORTANT**
                                                                            // self.gui
                                                                            //     .level_up
                                                                            //     .spawn(instance, moves);
                                                                            Some(BattleClientGuiCurrent::LevelUp)
                                                                        }
                                                                        true => None,
                                                                    }
                                                                }
                                                                None => None,
                                                            }
                                                        } // ClientMove::Catch(index) => {
                                                          //     if let Some(target) = match index.team {
                                                          //         Team::Player => &user.active[index.active],
                                                          //         Team::Opponent => &other.active[index.active],
                                                          //     }.pokemon.as_ref() {
                                                          //         ui::text::on_catch(text, target);
                                                          //     }
                                                          // }
                                                    } {
                                                        queue.current =
                                                            Some(Indexed(user_id, action));
                                                    } else {
                                                        self.update(
                                                            app, plugins, pokedex, movedex,
                                                            itemdex, delta,
                                                        );
                                                    }
                                                }
                                            }
                                            None => log::warn!("cannot get user {:?}", user_id),
                                        }
                                    }
                                }
                            }
                            Some(Indexed(user_id, action)) => {
                                // log::trace!("update current");

                                let user = if user_id.team() == local.player.id() {
                                    Some(&mut local.player as &mut dyn PlayerView<ID>)
                                } else {
                                    self.remotes.players.get_mut(user_id.team()).map(|p| p as _)
                                };

                                match user {
                                    Some(user) => match action {
                                        BattleClientGuiCurrent::Move(targets) => {
                                            if !self.gui.text.alive()
                                            /*if self.gui.text.page() > 0 || self.gui.text.waiting() */
                                            {
                                                match self.gui.pokemon.finished() {
                                                    true => queue.current = None,
                                                    false => {
                                                        self.gui.pokemon.update(app, plugins);
                                                    }
                                                }
                                            }
                                        }
                                        BattleClientGuiCurrent::Switch(new) => {
                                            match self.gui.text.alive() {
                                                true => {
                                                    if self.gui.text.page() == Some(1)
                                                        && !user
                                                            .active_eq(user_id.index(), Some(*new))
                                                    {
                                                        user.replace(user_id.index(), Some(*new));
                                                    }
                                                }
                                                false => queue.current = None,
                                            }
                                        }
                                        BattleClientGuiCurrent::UseItem(target) => {
                                            if !self.gui.text.alive() {
                                                queue.current = None;
                                            }
                                        }
                                        BattleClientGuiCurrent::Replace(index, replaced) => {
                                            if self.gui.text.waiting()
                                                || !self.gui.text.alive() && !*replaced
                                            {
                                                *replaced = true;
                                            }
                                            if !self.gui.text.alive() {
                                                queue.current = None;
                                            }
                                        }
                                        BattleClientGuiCurrent::Catch => {
                                            if !self.gui.text.alive() {
                                                queue.current = None;
                                            }
                                        }
                                        BattleClientGuiCurrent::SetExp => {
                                            queue.current = None;
                                        }
                                        BattleClientGuiCurrent::LevelUp => {
                                            queue.current = None;
                                        }
                                    },
                                    None => queue.current = None,
                                }
                            }
                        }
                    }
                    BattlePlayerState::Lose(..) => {
                        if let Some(winner) = Option::take(&mut self.should_end) {
                            self.state = BattlePlayerState::Closing(winner);
                        }
                    }
                    BattlePlayerState::Closing(winner) => match self.closer.alive() {
                        false => {
                            self.closer.reset();
                            self.closer.spawn::<ID>(
                                local,
                                &self.remotes,
                                Option::as_ref(winner),
                                self.gui.text.state_mut(),
                            );
                        }
                        true => {
                            self.closer.update(app, self.gui.text.state());
                            if self.closer.finished() {
                                self.closer.reset();
                                self.state = BattlePlayerState::End(winner.clone());
                            }
                        }
                    },
                    BattlePlayerState::End(..) => (),
                },
                None => todo!(),
            },
        }
    }

    pub fn ui(&mut self, app: &mut App, plugins: &mut Plugins, egui: &egui::Context) {
        self.gui.text.ui(app, plugins, egui);
        match &self.state {
            BattlePlayerState::Select => {
                if let Some(local) = self.local.as_mut() {
                    if let Some(action) = self.gui.panel.ui::<ID>(app, egui, local, &self.remotes) {
                        self.gui.actions.push(action);
                    }
                }
            }
            BattlePlayerState::Lose(message) => match message {
                EndMessage::Run => (),
                _ => {
                    egui::Window::new("Leave")
                        .title_bar(false)
                        .show(egui, |ui| {
                            if ui.button("Leave Game").clicked() {
                                self.state = BattlePlayerState::Closing(None);
                            }
                        });
                }
            },
            _ => (),
        }
        if matches!(
            &self.state,
            BattlePlayerState::Moving(..)
                | BattlePlayerState::Select
                | BattlePlayerState::WaitToSelect
        ) {
            let mut index = 0;
            if let Some(local) = self.local.as_ref() {
                for (.., pokemon) in local.player.active_iter() {
                    ui::PokemonStatus::status(egui, index, pokemon);
                    index += 1;
                }
            }
            if let Some((.., remote)) = self.remotes.players.get_index(self.remotes.current) {
                for (.., pokemon) in remote.active_iter() {
                    ui::PokemonStatus::status(egui, index, pokemon);
                    index += 1;
                }
            }
        }
    }

    pub fn draw(&self, draw: &mut Draw) {
        match &self.state {
            BattlePlayerState::WaitToStart => (),
            BattlePlayerState::Opening => {
                self.opener.draw::<ID>(draw, &self.gui, &self.remotes);
            }
            BattlePlayerState::Select => {
                self.gui.background.draw(draw, Default::default());
                self.gui.pokemon.draw_remotes(
                    draw,
                    &self.remotes,
                    Default::default(),
                    Color::WHITE,
                );
                if let Some(local) = self.local.as_ref() {
                    if let Some(selecting) = local.selecting.as_ref().map(|r| r.start) {
                        for (index, pokemon) in local.player.active_iter() {
                            self.gui.pokemon.draw(
                                (true, index),
                                local.player.active.len(),
                                &pokemon.pokemon.id,
                                pokemon.fainted(),
                                draw,
                                if selecting == index {
                                    Vec2::new(0.0, self.gui.bounce.offset)
                                } else {
                                    Default::default()
                                },
                                Color::WHITE,
                            );
                        }
                    }
                }
                self.gui.draw_panel(draw);
            }
            BattlePlayerState::WaitToSelect | BattlePlayerState::Moving(..) => {
                self.gui.background.draw(draw, Default::default());
                if let Some(local) = &self.local {
                    self.gui
                        .pokemon
                        .draw_local(draw, local, Default::default(), Color::WHITE);
                }
                self.gui.pokemon.draw_remotes(
                    draw,
                    &self.remotes,
                    Default::default(),
                    Color::WHITE,
                );
                self.gui.draw_panel(draw);
            }
            BattlePlayerState::Lose(..) => {
                self.gui.pokemon.draw_remotes(
                    draw,
                    &self.remotes,
                    Default::default(),
                    Color::WHITE,
                );
                self.gui.draw_panel(draw);
            }
            BattlePlayerState::Closing(..) => {
                self.closer.draw(draw, &self.gui, self.local.as_ref())
            }
            BattlePlayerState::End(..) => {}
        }
    }

    pub fn world_active(&self) -> bool {
        self.closer.world_active()
    }

    pub fn finished(&self) -> bool {
        matches!(self.state, BattlePlayerState::End(..))
    }

    pub fn data(&self) -> Option<&BattleData> {
        self.local.as_ref().map(|l| &l.data)
    }

    pub fn reset(&mut self) {
        self.reset_gui();
        self.remotes.players.clear();
        self.local = None;
        self.remotes.current = 0;
    }

    pub fn reset_gui(&mut self) {
        self.gui.reset();
    }
}
