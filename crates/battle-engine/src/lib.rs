pub extern crate firecore_battle as battle;
pub extern crate firecore_pokedex_engine as pokedex;

use std::{fmt::Debug, hash::Hash, marker::PhantomData, ops::Deref, rc::Rc, sync::Arc};

use bevy_ecs::prelude::*;

use pokedex::{
    engine::{
        log::{debug, warn},
        EngineContext,
    },
    Identifiable, NpcGroupId,
};

use pokedex::{
    engine::{
        graphics::Color,
        math::{vec2, Vec2},
        utils::{Entity, HashMap, Reset},
        Context,
    },
    gui::{bag::BagGui, party::PartyGui},
    item::Item,
    moves::Move,
    pokemon::Pokemon,
    Dex, Initializable, PokedexClientData,
};

use battle::{
    endpoint::{MpscClient, MpscEndpoint},
    message::{ClientMessage, ServerMessage},
    party::PlayerParty,
    pokemon::Indexed,
    prelude::{ClientPlayerData, FailedAction, StartableAction},
};
use view::GuiPokemonView;

use self::{
    ui::{
        view::{GuiLocalPlayer, GuiRemotePlayer},
        BattleGui,
    },
    view::PlayerView,
};

mod context;
mod move_queue;
pub use context::*;

pub mod ui;
pub mod view;

mod action;
mod state;
mod transition;

use action::*;

use self::state::BattlePlayerState;

struct Delta(f32);
const PROCESS: &str = "PROCESS";
const UPDATE: &'static str = "UPDATE";
const DRAW: &'static str = "DRAW";

struct ResourcePtr<R: 'static> {
    r: &'static mut R,
}

struct DexResource<I: Identifiable + 'static, O: Deref<Target = I> + 'static> {
    r: &'static dyn Dex<'static, I, O>,
}

unsafe impl<I: Identifiable + 'static, O: Deref<Target = I> + 'static> Send for DexResource<I, O> {}
unsafe impl<I: Identifiable + 'static, O: Deref<Target = I> + 'static> Sync for DexResource<I, O> {}

impl<R> From<&mut R> for ResourcePtr<R> {
    fn from(r: &mut R) -> Self {
        Self {
            r: unsafe { &mut *(r as *const R as *mut R) },
        }
    }
}

impl<R> Deref for ResourcePtr<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.r
    }
}

impl<R> std::ops::DerefMut for ResourcePtr<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.r
    }
}

impl<'a, I: Identifiable + 'static, O: Deref<Target = I> + 'static> From<&'a dyn Dex<'a, I, O>>
    for DexResource<I, O>
{
    fn from(r: &'a dyn Dex<'a, I, O>) -> Self {
        Self {
            r: unsafe { std::mem::transmute(r) },
        }
    }
}

impl<I: Identifiable + 'static, O: Deref<Target = I> + 'static> DexResource<I, O> {
    fn get(&self) -> &'static dyn Dex<'static, I, O> {
        self.r
    }
}

type Ctx = ResourcePtr<Context>;
type Eng = ResourcePtr<EngineContext>;
type BagRes = ResourcePtr<BagGui>;
type PartyRes = ResourcePtr<PartyGui>;

type Pokedex<P> = DexResource<Pokemon, P>;
type Movedex<M> = DexResource<Move, M>;
type Itemdex<I> = DexResource<Item, I>;

pub struct BattlePlayerGui<
    ID: Eq + Hash + Clone + Debug + Component,
    P: Deref<Target = Pokemon> + Clone,
    M: Deref<Target = Move> + Clone,
    I: Deref<Target = Item> + Clone,
> {
    pub gui: BattleGui<M>,

    p: PhantomData<P>,
    i: PhantomData<I>,

    schedule: Schedule,
    world: World,

    groups: HashMap<ID, NpcGroupId>,

    endpoint: MpscEndpoint<ID>,
}

impl<
        ID: Eq + Hash + Clone + Debug + Component,
        P: Deref<Target = Pokemon> + Clone + 'static,
        M: Deref<Target = Move> + Clone + 'static,
        I: Deref<Target = Item> + Clone + 'static,
    > BattlePlayerGui<ID, P, M, I>
{
    pub fn new(ctx: &mut Context, dex: &PokedexClientData, btl: &BattleGuiData) -> Self
    where
        ID: Default,
    {
        let (client, endpoint) = battle::endpoint::create();

        let mut schedule = Schedule::default();
        let mut world = World::new();

        let party = PartyGui::new(dex);
        let bag = BagGui::new(dex);

        Self {
            gui: BattleGui::new(ctx, btl),
            groups: Default::default(),
            endpoint,
            schedule,
            world,
            p: Default::default(),
            i: Default::default(),
        }
    }

    pub fn endpoint(&self) -> &MpscEndpoint<ID> {
        &self.endpoint
    }

    // pub fn winner(&self) -> Option<Option<&ID>> {
    //     if let BattlePlayerState::GameEnd(w) = &self.state {
    //         Some(w.as_ref())
    //     } else {
    //         None
    //     }
    // }

    // pub fn battling(&self) -> bool {
    //     !matches!(
    //         self.state,
    //         BattlePlayerState::WaitToStart
    //             | BattlePlayerState::Opening(..)
    //             | BattlePlayerState::Introduction(..)
    //     )
    // }

    // pub fn start(&mut self, transition: bool) {
    //     self.state = match transition {
    //         true => BattlePlayerState::Opening(TransitionState::default()),
    //         false => BattlePlayerState::WaitToSelect,
    //     };
    // }

    pub fn set_next_groups(&mut self, groups: HashMap<ID, NpcGroupId>) {
        self.groups = groups;
    }

    pub fn forfeit(&mut self) {
        if let Some(client) = self.world.get_resource::<MpscClient<ID>>() {
            client.send(ClientMessage::Forfeit);
        }
    }

    pub fn update<'d>(
        &mut self,
        ctx: &mut Context,
        eng: &mut EngineContext,
        dex: &PokedexClientData,
        pokedex: &'d dyn Dex<'d, Pokemon, P>,
        movedex: &'d dyn Dex<'d, Move, M>,
        itemdex: &'d dyn Dex<'d, Item, I>,
        delta: f32,
    ) {
        match self.schedule.get_stage_mut::<SystemStage>(&PROCESS) {
            Some(stage) => stage.run(&mut self.world),
            None => todo!(),
        }

        match self.schedule.get_stage_mut::<SystemStage>(&UPDATE) {
            Some(stage) => stage.run(&mut self.world),
            None => todo!(),
        }

        // match &mut self.state {
        //     BattlePlayerState::WaitToStart => (),
        //     state => match self.local.as_mut() {
        //         Some(local) => match state {
        //             BattlePlayerState::PlayerEnd | BattlePlayerState::GameEnd(..) => (),
        //             BattlePlayerState::Closing(_state) => (),
        //         },
        //         None => todo!(),
        //     },
        // }
    }

    pub fn draw(&mut self, ctx: &mut Context, eng: &EngineContext, dex: &PokedexClientData) {
        match self.schedule.get_stage_mut::<SystemStage>(&DRAW) {
            Some(stage) => stage.run(&mut self.world),
            None => todo!(),
        };
        if !matches!(self.state, BattlePlayerState::WaitToStart) {
            self.gui.background.draw(ctx, 0.0);
            self.remotes.values().for_each(|remote| {
                remote
                    .renderer
                    .iter()
                    .for_each(|active| active.draw(ctx, eng))
            });
            if let Some(local) = &self.local {
                match &self.state {
                    BattlePlayerState::WaitToStart => unreachable!(),
                    BattlePlayerState::Opening(..) => {
                        self.gui
                            .background
                            .draw(ctx, self.gui.opener.offset::<ID, P, M, I>());
                        self.gui.opener.draw_below_panel::<ID, P, M, I>(
                            ctx,
                            &local.renderer,
                            &self.remotes.values().next().unwrap().renderer,
                        );
                        self.gui.trainer.draw(ctx);
                        self.gui.draw_panel(ctx);
                        self.gui.opener.draw::<ID, P, M, I>(ctx);
                    }
                    BattlePlayerState::Introduction(..) => {
                        self.gui.background.draw(ctx, 0.0);
                        self.gui.introduction.draw::<ID, P, M, I>(
                            ctx,
                            eng,
                            &local.renderer,
                            &self.remotes.values().next().unwrap().renderer,
                        );
                        self.gui.trainer.draw(ctx);
                        self.gui.draw_panel(ctx);
                        self.gui.text.draw(ctx, eng);
                    }
                    // BattlePlayerState::Faint(..) => if self.gui.party.alive() {
                    //     self.gui.party.draw(ctx)
                    // },
                    BattlePlayerState::WaitToSelect | BattlePlayerState::Moving(..) => {
                        local
                            .renderer
                            .iter()
                            .for_each(|active| active.draw(ctx, eng));
                        self.gui.draw_panel(ctx);
                        self.gui.text.draw(ctx, eng);
                        self.gui.level_up.draw(ctx, eng);
                        if self.gui.party.alive() {
                            self.gui.party.draw(ctx, eng)
                        }
                    }
                    BattlePlayerState::GameEnd(..) | BattlePlayerState::PlayerEnd => {
                        local
                            .renderer
                            .iter()
                            .for_each(|active| active.draw(ctx, eng));
                        self.gui.draw_panel(ctx);
                        self.gui.text.draw(ctx, eng);
                    }
                    BattlePlayerState::Closing(..) => {
                        self.gui.background.draw(ctx, 0.0);
                        // self.gui.introduction.draw::<ID, P, M, I>(
                        //     ctx,
                        //     &local.renderer,
                        //     &self.remotes.values().next().unwrap().renderer,
                        // );
                        self.gui.trainer.draw(ctx);
                        self.gui.draw_panel(ctx);
                        self.gui.text.draw(ctx, eng);
                    }
                }
            }
        }
    }
    pub fn process(
        mut commands: Commands,
        random: &mut impl rand::Rng,
        dex: NonSend<Rc<PokedexClientData>>,
        btl: NonSend<Rc<BattleGuiData>>,
        pokedex: Res<crate::Pokedex<P>>,
        movedex: Res<crate::Movedex<M>>,
        itemdex: Res<crate::Itemdex<I>>,
        mut local: NonSendMut<GuiLocalPlayer<ID, P, M, I>>,
        client: Res<MpscClient<ID>>,
    ) {
        while let Ok(message) = client.receiver.try_recv() {
            match message {
                ServerMessage::Begin(client) => {
                    let mut remotes = client
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
                            };
                            (
                                player.id.clone(),
                                GuiRemotePlayer {
                                    renderer: GuiRemotePlayer::create(&player, btl, dex),
                                    player,
                                    npc: None,
                                },
                            )
                        })
                        .collect::<HashMap<_, _>>();

                    let groups = std::mem::take(&mut self.groups);

                    for (id, group) in groups.into_iter() {
                        if let Some(remote) = remotes.get_mut(&id) {
                            remote.npc = Some(group);
                        }
                    }

                    self.world.insert_non_send(remotes);

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
                    };

                    let local = GuiLocalPlayer {
                        renderer: GuiLocalPlayer::create(&player, btl, dex),
                        bag: client.bag.init(itemdex).unwrap_or_default(),
                        data: client.data,
                        player,
                    };

                    commands.insert_resource(local);
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
                                self.gui.text.pages.clear();
                                self.gui.text.spawn();
                            }
                        },
                        ServerMessage::Replace(Indexed(pokemon, new)) => match &mut self.state {
                            BattlePlayerState::Moving(queue) => {
                                queue.actions.push_back(Indexed(
                                    pokemon,
                                    BattleClientGuiAction::Replace(Some(new)),
                                ));
                            }
                            _ => {
                                if let Some((renderer, pokemon)) = match pokemon.team()
                                    == local.player.id()
                                {
                                    true => {
                                        local.player.replace(pokemon.index(), Some(new));
                                        let renderer = &mut local.renderer[pokemon.index()];
                                        let pokemon = local.player.active(pokemon.index());
                                        let id = pokemon.map(|p| p.pokemon.id);
                                        renderer.status.update_gui(pokemon, None, true);
                                        Some((renderer, id))
                                    }
                                    false => {
                                        if let Some(remote) = self.remotes.get_mut(pokemon.team()) {
                                            remote.player.replace(pokemon.index(), Some(new));
                                            let renderer = &mut remote.renderer[pokemon.index()];
                                            let pokemon = remote
                                                .player
                                                .active(pokemon.index())
                                                .map(|u| u as &dyn GuiPokemonView<P, M, I>);
                                            let id = pokemon.map(|v| v.pokemon().id);
                                            renderer.status.update_gui_view(pokemon, None, true);
                                            Some((renderer, id))
                                        } else {
                                            None
                                        }
                                    }
                                } {
                                    renderer.pokemon.new_pokemon(dex, pokemon);
                                }
                            }
                        },
                        ServerMessage::AddRemote(Indexed(target, unknown)) => {
                            if let Some(party) = self.remotes.get_mut(target.team()) {
                                party.player.add(target.index(), unknown.init(pokedex));
                            }
                        }
                        // ServerMessage::Winner(player) => {
                        //     self.state = BattlePlayerState::Winner(player);
                        //     for (index, pokemon) in self.local.party.pokemon.iter().enumerate() {
                        //         party[index] = pokemon.clone();
                        //     }
                        // }
                        ServerMessage::Ping(..) => (),
                        ServerMessage::Fail(action) => match action {
                            FailedAction::Move(i) | FailedAction::Switch(i) => match &self.state {
                                BattlePlayerState::Select(..) => {
                                    self.gui.panel.despawn();
                                    self.state =
                                        BattlePlayerState::Select(0, local.player.active.len());
                                }
                                _ => self.state = BattlePlayerState::Select(i, i + 1),
                            },
                            FailedAction::Replace(index) => {
                                debug!("cannot replace pokemon at active index {}", index);
                            }
                        },
                        ServerMessage::Catch(instance) => {
                            match instance.init(random, pokedex, movedex, itemdex) {
                                Some(instance) => {
                                    if let Ok(_) = local.player.pokemon.try_push(instance) {}
                                }
                                None => warn!("Could not initialize caught pokemon."),
                            }
                        }
                        ServerMessage::PlayerEnd(..) => {
                            self.state = BattlePlayerState::PlayerEnd;
                        }
                        ServerMessage::GameEnd(winner) => {
                            self.state = BattlePlayerState::GameEnd(winner);
                        }
                        ServerMessage::Begin(..) => unreachable!(),
                    },
                    None => todo!(),
                },
            }
        }
    }
}

impl<
        ID: Eq + Hash + Clone + Component + Debug,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
    > Reset for BattlePlayerGui<ID, P, M, I>
{
    fn reset(&mut self) {
        self.gui.reset();
        self.remotes.clear();
    }
}
