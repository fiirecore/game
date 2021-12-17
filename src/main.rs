use battlelib::pokedex::{item::Item, moves::Move, pokemon::Pokemon, BasicDex};
pub(crate) use firecore_battle_gui::pokedex;
pub(crate) use pokedex::engine;
use worldlib::serialized::SerializedWorld;

// mod args;
mod battle;
mod game;
mod saves;
mod state;
mod world;
mod command;
mod storage;

use std::collections::HashMap;

use rand::prelude::{SeedableRng, SmallRng};

use crate::saves::PlayerSaves;
use game::config::Configuration;

use firecore_battle_gui::{
    context::BattleGuiContext,
    pokedex::engine::{
        audio::MusicId,
        graphics::{self, Color, DrawParams},
        util::{HEIGHT, WIDTH},
        ContextBuilder,
    },
};

use crate::engine::log::info;
use pokedex::context::PokedexClientData;
use state::StateManager;

extern crate firecore_battle as battlelib;
extern crate firecore_world as worldlib;

const TITLE: &str = "Pokemon FireRed";
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const SCALE: f32 = 3.0;

static mut POKEDEX: Option<BasicDex<Pokemon>> = None;
static mut MOVEDEX: Option<BasicDex<Move>> = None;
static mut ITEMDEX: Option<BasicDex<Item>> = None;

fn pokedex() -> &'static BasicDex<Pokemon> {
    unsafe { POKEDEX.as_ref().unwrap() }
}

fn movedex() -> &'static BasicDex<Move> {
    unsafe { MOVEDEX.as_ref().unwrap() }
}

fn itemdex() -> &'static BasicDex<Item> {
    unsafe { ITEMDEX.as_ref().unwrap() }
}

fn main() {
    engine::run(
        ContextBuilder::new(TITLE, (WIDTH * SCALE) as _, (HEIGHT * SCALE) as _), // .resizable(true)
        // .show_mouse(true)
        async {
            let save_locally = cfg!(debug_assertions);

            let configuration = Configuration::load(save_locally)
                .await
                .unwrap_or_else(|err| panic!("Cannot load configuration with error {}", err));

            let saves = PlayerSaves::load(save_locally)
                .await
                .unwrap_or_else(|err| panic!("Could not load player saves with error {}", err));

            OpenContext {
                configuration,
                saves,
            }
        },
        move |mut ctx,
              OpenContext {
                  configuration,
                  saves,
              }| {
            info!("Starting {} v{}", TITLE, VERSION);
            info!("By {}", AUTHORS);

            let debug = cfg!(debug_assertions);

            // #[cfg(feature = "audio")]
            // let audio = !args.contains(&Args::DisableAudio);

            info!("Loading configuration...");

            // Load dexes;

            info!("Loading dexes...");

            let (pokedex, movedex, itemdex) = bincode::deserialize(include_bytes!(
                "../build/data/dex.bin"
            ))
            .unwrap_or_else(|err| panic!("Could not deserialize pokedex with error {}", err));

            unsafe {
                POKEDEX = Some(pokedex);

                MOVEDEX = Some(movedex);

                ITEMDEX = Some(itemdex);
            }

            info!("Loading fonts...");

            // Loads configuration, sets up controls

            let fonts: Vec<engine::text::FontSheet<Vec<u8>>> =
                bincode::deserialize(include_bytes!("../build/data/fonts.bin"))
                    .unwrap_or_else(|err| panic!("Could not load font sheets with error {}", err));

            for font in fonts {
                engine::text::insert_font(&mut ctx, &font).unwrap();
            }

            #[cfg(feature = "audio")]
            /* if audio */
            {
                info!("Loading audio...");
                //Load audio files and setup audio
                match bincode::deserialize::<HashMap<MusicId, Vec<u8>>>(include_bytes!(
                    "../build/data/audio.bin"
                )) {
                    Ok(audio_data) => {
                        graphics::draw_text_left(
                            &mut ctx,
                            &0,
                            "Loading audio...",
                            5.0,
                            5.0,
                            DrawParams::color(Color::WHITE),
                        );
                        for (id, data) in audio_data {
                            engine::audio::add_music(&mut ctx, id, data);
                        }
                    }
                    Err(err) => engine::log::error!("Could not read sound file with error {}", err),
                }
            } // else {
              //     info!("Skipping audio loading...");
              // }

            graphics::clear(&mut ctx, Color::BLACK);

            let mut random = SmallRng::seed_from_u64(engine::util::seed());

            info!("Loading dex textures and audio...");

            let dex_engine = bincode::deserialize(include_bytes!("../build/data/dex_engine.bin"))
                .unwrap_or_else(|err| {
                    panic!(
                        "Could not deserialize pokedex engine data with error {}",
                        err
                    )
                });

            let dex = PokedexClientData::new(&mut ctx, dex_engine)
                .unwrap_or_else(|err| panic!("Could not initialize dex data with error {}", err));

            let btl = BattleGuiContext::new(&mut ctx).unwrap_or_else(|err| {
                panic!("Could not initialize battle data with error {}", err)
            });

            let world = bincode::deserialize(include_bytes!("../build/data/world.bin")).unwrap_or_else(|err| panic!("Could not load world data with error {}", err));

            #[cfg(feature = "discord")]
            use discord_rich_presence::{activity::Activity, new_client, DiscordIpc};

            #[cfg(feature = "discord")]
            let mut client = {
                let mut client = new_client("862413316420665386").unwrap_or_else(|err| {
                    panic!("Could not create discord IPC client with error {}", err)
                });
                client.connect().unwrap_or_else(|err| {
                    panic!("Could not connect to discord with error {}", err)
                });
                client
                    .set_activity(Activity::new().state("test state").details("test details"))
                    .unwrap_or_else(|err| {
                        panic!("Could not set client activity with error {}", err)
                    });
                client
            };

            // {
            //     if args.contains(&Args::Debug) {
            //         set_debug(true);
            //     }

            //     if is_debug() {
            //         info!("Running in debug mode");
            //     }
            // }

            info!("Initialized game context!");

            LoadContext {
                configuration,
                saves,
                dex,
                btl,
                world,
                random,
            }
        },
        StateManager::new,
    );

    #[cfg(feature = "discord")]
    client.close().unwrap();
}

struct OpenContext {
    configuration: Configuration,
    saves: PlayerSaves,
}

pub(crate) struct LoadContext {
    pub configuration: Configuration,
    pub saves: PlayerSaves,
    pub dex: PokedexClientData,
    pub btl: BattleGuiContext,
    pub world: SerializedWorld,
    pub random: SmallRng,
}

pub(crate) use crossbeam_channel::Receiver;

pub(crate) fn split<T>() -> (Sender<T>, Receiver<T>) {
    let (x, y) = crossbeam_channel::unbounded();
    (Sender(x), y)
}

#[derive(Clone)]
pub(crate) struct Sender<T>(crossbeam_channel::Sender<T>);

impl<T> Sender<T> {
    pub fn send(&self, msg: T) {
        if let Err(err) = self.0.try_send(msg) {
            engine::log::warn!(
                "Could not send message {} with error {}",
                std::any::type_name::<T>(),
                err
            );
        }
    }
}
