use std::sync::Arc;

use battle::{
    ai::BattleAi,
    data::{BattleData, VersusType},
    endpoint::{create, MpscConnection},
    host::{Battle, PlayerData},
    message::{ClientMessage, ServerMessage},
    moves::BattleMove,
    party::PlayerParty,
    pokemon::remote::UnknownPokemon,
};
use bevy::prelude::*;

use firecore_battle_client::{resources::BattleBackground, *};
use firecore_battle_engine::{moves::EngineMove, DefaultEngine};
use hashbrown::HashMap;
use iyes_loopless::state::NextState;
use pokengine::{
    dex::BevyDex,
    engine::text::MessagePage,
    pokedex::{
        item::Item,
        moves::{Move, MoveId},
        pokemon::{owned::SavedPokemon, Pokemon, PokemonId},
        trainer::TrainerGroupId,
        Dex,
    },
};
use rand::{rngs::SmallRng, SeedableRng};

use firecore_battle_client::resources::PlayerChannel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(pokengine::engine::FirecorePlugins)
        .add_plugin(firecore_pokedex_client::PokedexClientPlugin)
        .add_plugin(BattleClientPlugin::<u8>::default())
        .insert_resource(BattleRandom(rand::rngs::SmallRng::seed_from_u64(0)))
        .add_startup_system(setup)
        .add_startup_system(load_resources)
        .add_system(update)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>, mut random: ResMut<BattleRandom>) {
    let mut pokedex =
        BevyDex(firecore_storage::from_bytes(include_bytes!("../../../data/pokedex.bin")).unwrap());
    let mut movedex =
        BevyDex(firecore_storage::from_bytes(include_bytes!("../../../data/movedex.bin")).unwrap());
    let mut itemdex = BevyDex(Dex::new(Default::default()));

    let mut ai = BattleAi::<u8, BattleTrainer>::new();

    let (a, b) = create();

    let mut engine = DefaultEngine::new::<SmallRng>();

    let bmoves = firecore_storage::from_bytes::<Vec<BattleMove>>(include_bytes!(
        "../../../data/battle_moves.bin"
    ))
    .unwrap();

    let mut move_execs = firecore_storage::from_bytes::<HashMap<MoveId, _>>(include_bytes!(
        "../../../data/battle_move_execution.bin"
    ))
    .unwrap();

    let mut moves = HashMap::new();

    for m in bmoves {
        let id = m.id;
        moves.insert(id, EngineMove {
            data: m,
            usage: move_execs.remove(&id).unwrap(),
        });
    }

    engine.moves = moves;

    let mut players = [
        PlayerData {
            id: 0,
            name: Some("Player".into()),
            party: vec![
                SavedPokemon {
                    pokemon: PokemonId(1),
                    level: 20,
                    ..Default::default()
                }
                .init(&mut random.0, &pokedex, &movedex, &itemdex)
                .unwrap(),
                SavedPokemon {
                    pokemon: PokemonId(2),
                    level: 30,
                    ..Default::default()
                }
                .init(&mut random.0, &pokedex, &movedex, &itemdex)
                .unwrap(),
            ],
            bag: Default::default(),
            trainer: Some(BattleTrainer {
                worth: 500,
                texture: TrainerGroupId("".parse().unwrap()),
                defeat: vec![MessagePage {
                    lines: vec!["Player defeat message".into(), "Line 2".into()],
                    ..Default::default()
                }],
            }),
            settings: Default::default(),
            endpoint: Arc::new(a),
        },
        PlayerData {
            id: 1,
            name: Some("Rival".into()),
            party: vec![
                SavedPokemon {
                    pokemon: PokemonId(2),
                    level: 20,
                    ..Default::default()
                }
                .init(&mut random.0, &pokedex, &movedex, &itemdex)
                .unwrap(),
                SavedPokemon {
                    pokemon: PokemonId(1),
                    level: 30,
                    ..Default::default()
                }
                .init(&mut random.0, &pokedex, &movedex, &itemdex)
                .unwrap(),
            ],
            trainer: Some(BattleTrainer {
                worth: 20,
                texture: TrainerGroupId("rival".parse().unwrap()),
                defeat: vec![MessagePage {
                    lines: vec!["Bruh".into()],
                    ..Default::default()
                }],
            }),
            bag: Default::default(),
            settings: Default::default(),
            endpoint: Arc::new(ai.endpoint().clone()),
        },
    ];

    assert!(players[0].party[0].moves.len() > 0);

    for p in &mut players {
        for p in &mut p.party {
            for m in p.moves.iter_mut() {
                m.pp = m.m.pp;
            }
        }
    }

    let battle = Battle::<_, _, DefaultEngine<_, _>>::new(
        BattleData {
            versus: VersusType::Trainer,
            settings: Default::default(),
            active: 1,
        },
        players,
    );

    commands.insert_resource(Ai(ai));
    commands.insert_resource(pokedex);
    commands.insert_resource(movedex);
    commands.insert_resource(itemdex);
    commands.insert_resource(Host(battle));
    commands.insert_resource(BattleEngine(engine));
    commands.insert_resource(PlayerChannel(Arc::new(b)));

    commands.insert_resource(NextState(BattleClientState::start()));
}

fn update(
    mut ai: ResMut<Ai>,
    engine: Res<BattleEngine>,
    mut battle: ResMut<Host>,
    pokedex: Res<BevyDex<Pokemon>>,
    movedex: Res<BevyDex<Move>>,
    itemdex: Res<BevyDex<Item>>,
    mut random: ResMut<BattleRandom>,
) {
    battle.0.update(&mut random.0, &*engine, &movedex).unwrap();

    ai.update(&mut random.0, &pokedex, &movedex, &itemdex)
        .unwrap();
    println!("{:?}", ai.0.party());
}

#[derive(Resource, Deref, DerefMut)]
struct Ai(BattleAi<u8, BattleTrainer>);

#[derive(Resource, Deref)]
struct BattleEngine(DefaultEngine<u8, BattleTrainer>);

#[derive(Resource)]
struct Host(Battle<u8, BattleTrainer, DefaultEngine<u8, BattleTrainer>>);

#[derive(Resource, Deref, DerefMut)]
pub struct BattleMessageChannel<ID, T>(pub MpscConnection<ClientMessage<ID>, ServerMessage<ID, T>>);
