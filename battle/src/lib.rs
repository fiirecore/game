// #![feature(map_into_keys_values)] // for move queue fn

extern crate firecore_dependencies as deps;
extern crate firecore_pokedex as pokedex;

use {
    deps::{
        log::{debug, info, warn},
        random::{Random, RandomState, GLOBAL_STATE},
        rhai::Engine,
    },
    pokedex::{
        item::ItemUseType,
        moves::{
            instance::MoveInstance,
            target::MoveTargetInstance,
            usage::{MoveResult, PokemonTarget},
        },
        pokemon::{instance::PokemonInstance, Experience, Health},
        types::Effective,
    },
};

use pokedex::{
    battle::{ActionInstance, ActivePokemon, BattleMove, PokemonIndex},
    moves::usage::{DamageResult, NoHitResult},
};

use crate::{
    data::*,
    message::{ClientMessage, ServerMessage},
    player::BattlePlayer,
    state::BattleState,
    client::{BattleClientMove, BattleClientAction},
};

pub mod data;
pub mod state;

pub mod player;

pub mod client;
pub mod message;

pub static BATTLE_RANDOM: Random = Random::new(RandomState::Static(&GLOBAL_STATE));

pub struct Battle<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq + Ord> {
    ////////////// if using hashmap, only remaining player should be winner
    state: BattleState<ID>,

    data: BattleData,

    player1: BattlePlayer<ID>,
    player2: BattlePlayer<ID>,
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq + Ord> Battle<ID> {
    pub fn new(data: BattleData, player1: BattlePlayer<ID>, player2: BattlePlayer<ID>) -> Self {
        Self {
            state: BattleState::default(),
            data,
            player1,
            player2,
        }
    }

    pub fn data(&self) -> &BattleData {
        &self.data
    }

    pub fn begin(&mut self) {
        self.player1.client.send(ServerMessage::User(
            self.data.clone(),
            self.player1.party.as_known(),
        ));
        self.player2.client.send(ServerMessage::User(
            self.data.clone(),
            self.player2.party.as_known(),
        ));

        self.player1.party.reveal_active();
        self.player2.party.reveal_active();

        self.player1
            .client
            .send(ServerMessage::Opponents(self.player2.party.as_unknown()));
        self.player2
            .client
            .send(ServerMessage::Opponents(self.player1.party.as_unknown()));

        self.state = BattleState::SELECTING_START;
    }

    fn _receive(&mut self, player1: bool) {
        let (user, other) = match player1 {
            true => (&mut self.player1, &mut self.player2),
            false => (&mut self.player2, &mut self.player1),
        };
        while let Some(message) = user.client.receive() {
            match message {
                ClientMessage::Move(active, bmove) => {
                    if let Some(pokemon) = user.party.active.get_mut(active) {
                        match pokemon {
                            ActivePokemon::Some(.., queued_move) => {
                                *queued_move = Some(bmove);
                                // party.client.confirm_move(active);
                            }
                            _ => warn!(
                                "Party {} could not add move #{:?} to pokemon #{}",
                                user.party.id, bmove, active
                            ),
                        }
                    }
                }
                ClientMessage::FaintReplace(active, index) => {
                    debug!(
                        "faint replace: {} replacing {} with {}",
                        user.party.name(),
                        active,
                        index
                    );
                    if !user.party.active_contains(index) {
                        if if let Some(pokemon) = user.party.pokemon.get(index) {
                            if !pokemon.pokemon.fainted() {
                                user.party.active[active] = ActivePokemon::Some(index, None);
                                if let Some(pokemon) = user.party.know(index) {
                                    other.client.send(ServerMessage::AddUnknown(index, pokemon));
                                }
                                other.client.send(ServerMessage::FaintReplace(
                                    PokemonIndex {
                                        team: user.party.id,
                                        index: active,
                                    },
                                    Some(index),
                                ));
                                false
                            } else {
                                true
                            }
                        // party.client.confirm_replace(active, index);
                        } else {
                            true
                        } {
                            warn!("Player {} tried to replace a fainted pokemon with an missing/fainted pokemon", user.party.name());
                        }
                    } else {
                        warn!("Player {} tried to replace a pokemon with a pokemon that is already active.", user.party.name());
                    }
                }
                ClientMessage::RequestPokemon(request) => {
                    if let Some(pokemon) = other.party.pokemon.get(request) {
                        if matches!(self.data.type_, BattleType::Wild)
                            || pokemon.requestable
                            || (pokemon.pokemon.fainted() && pokemon.known)
                        {
                            user.client.send(ServerMessage::PokemonRequest(
                                request,
                                pokemon.pokemon.cloned(),
                            ));
                        }
                    }
                }
                ClientMessage::Forfeit => self.state = BattleState::End(false, other.party.id),
                ClientMessage::FinishedTurnQueue => (),
            }
        }
    }

    pub fn update(&mut self, engine: &Engine) {
        self._receive(true);
        self._receive(false);

        match &mut self.state {
            BattleState::Setup => self.begin(),

            BattleState::StartWait => (),
            // Select pokemon moves / items / party switches
            BattleState::Selecting(started) => match *started {
                false => {
                    self.player1.client.send(ServerMessage::StartSelecting);
                    self.player2.client.send(ServerMessage::StartSelecting);
                    *started = true;
                }
                true => {
                    if self.player1.party.ready_to_move() && self.player2.party.ready_to_move() {
                        self.state = BattleState::MOVE_START;
                    }
                }
            },
            BattleState::Moving(waiting) => match waiting {
                false => {
                    *waiting = true;

                    let queue = pokedex::battle::move_queue(
                        &mut self.player1.party,
                        &mut self.player2.party,
                    );

                    let player_queue = self.client_queue(engine, queue);

                    // end queue calculations

                    self.player2
                        .client
                        .send(ServerMessage::TurnQueue(player_queue.clone()));
                    self.player1
                        .client
                        .send(ServerMessage::TurnQueue(player_queue));

                    self.player1
                        .client
                        .send(ServerMessage::AskFinishedTurnQueue);
                    self.player2
                        .client
                        .send(ServerMessage::AskFinishedTurnQueue);
                }
                true => {
                    if self.player1.client.finished_turn() && self.player2.client.finished_turn() {
                        if self.player2.party.all_fainted() {
                            debug!("{} won!", self.player1.party.name());
                            self.state = BattleState::End(false, self.player1.party.id);
                        } else if self.player1.party.all_fainted() {
                            debug!("{} won!", self.player2.party.name());
                            self.state = BattleState::End(false, self.player2.party.id);
                        } else if !self.player1.party.needs_replace()
                            && !self.player2.party.needs_replace()
                        {
                            self.state = BattleState::SELECTING_START;
                        }
                    }
                }
            },
            BattleState::End(started, winner) => {
                if !*started {
                    self.player1.client.send(ServerMessage::Winner(*winner));
                    self.player2.client.send(ServerMessage::Winner(*winner));
                    *started = true;
                }
            }
        }
    }

    pub fn finished(&self) -> bool {
        matches!(self.state, BattleState::End(true, ..))
    }

    pub fn winner(&self) -> Option<&ID> {
        match &self.state {
            BattleState::End(.., winner) => Some(winner),
            _ => None,
        }
    }

    pub fn client_queue(
        &mut self,
        engine: &Engine,
        queue: Vec<ActionInstance<ID, BattleMove>>,
    ) -> Vec<ActionInstance<ID, BattleClientAction<ID>>> {
        let mut player_queue = Vec::with_capacity(queue.len());

        for instance in queue {
            let (user, other) = match instance.pokemon.team == self.player1.party.id {
                true => (&mut self.player1, &mut self.player2),
                false => (&mut self.player2, &mut self.player1),
            };
            if user.party.active(instance.pokemon.index).is_some() {
                match instance.action {
                    BattleMove::Move(move_index, targets) => {
                        let user_pokemon = user.party.active(instance.pokemon.index).unwrap();

                        let targets = targets
                            .into_iter()
                            .flat_map(|target| {
                                match target {
                                    MoveTargetInstance::Opponent(index) => {
                                        other.party.active(index)
                                    }
                                    MoveTargetInstance::Team(index) => user.party.active(index),
                                    MoveTargetInstance::User => Some(user_pokemon),
                                }
                                .map(|i| (target, i))
                            })
                            .map(|(target, pokemon)| PokemonTarget {
                                pokemon: &pokemon.pokemon,
                                active: target,
                            })
                            .collect();

                        let turn = user_pokemon
                            .pokemon
                            .use_own_move(engine, move_index, targets);

                        let mut target_results = Vec::with_capacity(turn.results.len());

                        for (target_instance, result) in turn.results {
                            let mut client_results = Vec::new();

                            {
                                let user = match instance.pokemon.team == self.player1.party.id {
                                    true => &mut self.player1,
                                    false => &mut self.player2,
                                };

                                let user = &mut user
                                    .party
                                    .active_mut(instance.pokemon.index)
                                    .unwrap()
                                    .pokemon;

                                for result in &result {
                                    match result {
                                        MoveResult::Drain(_, heal, ..) => {
                                            user.current_hp =
                                                (user.current_hp + *heal).min(user.base.hp());
                                            client_results.push(BattleClientMove::UserHP(
                                                user.current_hp as f32 / user.max_hp() as f32,
                                            ));
                                        }
                                        _ => (),
                                    }
                                }
                            }

                            for result in result {
                                let (user, other) =
                                    match instance.pokemon.team == self.player1.party.id {
                                        true => (&mut self.player1, &mut self.player2),
                                        false => (&mut self.player2, &mut self.player1),
                                    };

                                let (target_party, index) = match target_instance {
                                    MoveTargetInstance::Opponent(index) => (other, index),
                                    MoveTargetInstance::User => (user, instance.pokemon.index),
                                    MoveTargetInstance::Team(index) => (user, index),
                                };

                                let target = target_party.party.active_mut(index).unwrap();

                                fn on_damage<
                                    ID: Sized
                                        + Copy
                                        + core::fmt::Debug
                                        + core::fmt::Display
                                        + PartialEq,
                                >(
                                    pokemon: &mut PokemonInstance,
                                    results: &mut Vec<BattleClientMove<ID>>,
                                    result: DamageResult<Health>,
                                ) {
                                    pokemon.current_hp =
                                        pokemon.current_hp.saturating_sub(result.damage);
                                    results.push(BattleClientMove::TargetHP(
                                        pokemon.hp() as f32 / pokemon.max_hp() as f32,
                                    ));
                                    if result.crit {
                                        results.push(BattleClientMove::Critical);
                                    }
                                    if !matches!(result.effective, Effective::Effective) {
                                        results.push(BattleClientMove::Effective(result.effective));
                                    }
                                }

                                match result {
                                    MoveResult::Damage(result) => {
                                        on_damage(&mut target.pokemon, &mut client_results, result)
                                    }
                                    MoveResult::Status(effect) => {
                                        target.pokemon.status = Some(effect)
                                    }
                                    MoveResult::Drain(result, ..) => {
                                        on_damage(&mut target.pokemon, &mut client_results, result)
                                    }
                                    MoveResult::StatStage(stat, stage) => {
                                        target.pokemon.base.change_stage(stat, stage);
                                        client_results
                                            .push(BattleClientMove::StatStage(stat, stage));
                                    }
                                    MoveResult::Flinch => target.flinch = true,
                                    MoveResult::NoHit(result) => match result {
                                        NoHitResult::Ineffective => client_results.push(
                                            BattleClientMove::Effective(Effective::Ineffective),
                                        ),
                                        NoHitResult::Miss => {
                                            client_results.push(BattleClientMove::Miss);
                                            continue;
                                        }
                                        NoHitResult::Todo => {
                                            client_results.push(BattleClientMove::Fail)
                                        }
                                    },
                                }

                                if target.pokemon.fainted() {

                                    let experience = (target.pokemon.exp_from() as f32
                                    * match matches!(self.data.type_, BattleType::Wild) {
                                        true => 1.5,
                                        false => 1.0,
                                    }
                                    * 7.0)
                                    as Experience;

                                    client_results.push(BattleClientMove::Faint(PokemonIndex {
                                        team: target_party.party.id,
                                        index,
                                    }));

                                    let inactive = target_party.party.any_inactive();
                                    if let Some(active) = target_party.party.active.get_mut(index) {
                                        if inactive {
                                            *active = ActivePokemon::ToReplace;
                                        } else {
                                            *active = ActivePokemon::None;
                                        }
                                    }

                                    if target_party.settings.gains_exp {

                                        let user = match instance.pokemon.team == self.player1.party.id
                                        {
                                            true => (&mut self.player1),
                                            false => (&mut self.player2),
                                        };
                                        

                                        client_results.push(BattleClientMove::GainExp(experience)); // .send(ServerMessage::GainExp(instance.pokemon.index, experience));

                                        let pokemon = &mut user
                                            .party
                                            .active_mut(instance.pokemon.index)
                                            .unwrap()
                                            .pokemon;

                                        let level = pokemon.level;

                                        // To - do: send opponent level ups

                                        pokemon.add_exp(experience);

                                        if !pokemon.moves.is_full() {
                                            let mut moves = pokemon.moves_from(level..pokemon.level);
                                            while let Some(moves) = moves.pop() {
                                                if let Err(_) =
                                                    pokemon.moves.try_push(MoveInstance::new(moves))
                                                {
                                                    break;
                                                }
                                            }
                                        }

                                    }

                                }
                            }

                            target_results.push((target_instance, client_results));
                        }

                        player_queue.push(ActionInstance {
                            pokemon: instance.pokemon,
                            action: BattleClientAction::Move(turn.pokemon_move, target_results),
                        });
                    }
                    BattleMove::UseItem(item, target) => {
                        if match &item.usage {
                            ItemUseType::Script(script) => {
                                user.party
                                    .active_mut(instance.pokemon.index)
                                    .unwrap()
                                    .pokemon
                                    .execute_item_script(script);
                                true
                            }
                            ItemUseType::Pokeball => match self.data.type_ {
                                BattleType::Wild => {
                                    let (target, active) = match target {
                                        MoveTargetInstance::Opponent(index) => (other, index),
                                        _ => unreachable!(),
                                    };
                                    if let Some(active) = target.party.active.get_mut(active) {
                                        let active = std::mem::replace(active, ActivePokemon::None);
                                        if let Some(index) = active.index() {
                                            if target.party.pokemon.len() > index {
                                                let pokemon = target.party.pokemon.remove(index);
                                                user.client.send(ServerMessage::PokemonRequest(
                                                    index,
                                                    pokemon.pokemon.owned(),
                                                ));
                                            }
                                        }
                                    }
                                    true
                                }
                                _ => {
                                    info!("Cannot use pokeballs in trainer battles!");
                                    false
                                }
                            },
                            ItemUseType::None => true,
                        } {
                            player_queue.push(ActionInstance {
                                pokemon: instance.pokemon,
                                action: BattleClientAction::UseItem(item, target),
                            });
                        }
                    }
                    BattleMove::Switch(new) => {
                        user.party.replace(instance.pokemon.index, Some(new));
                        let unknown = user
                            .party
                            .active_index(instance.pokemon.index)
                            .map(|index| user.party.know(index))
                            .flatten();
                        player_queue.push(ActionInstance {
                            pokemon: instance.pokemon,
                            action: BattleClientAction::Switch(new, unknown),
                        });
                    }
                }
            }
        }
        player_queue
    }
}
