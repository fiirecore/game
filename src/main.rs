extern crate firecore_saves as saves;
extern crate firecore_storage as storage;

use battlelib::pokedex::{item::Item, moves::Move, pokemon::Pokemon, BasicDex};
pub(crate) use firecore_battle_gui::pokedex;
pub(crate) use pokedex::engine;

pub mod args;
pub mod battle;
pub mod game;
pub mod state;
pub mod world;

use std::{collections::HashMap, ops::{Deref, DerefMut}};

use game::{config::Configuration, init};
use rand::prelude::{SeedableRng, SmallRng};
use saves::PlayerSaves;

use firecore_battle_gui::{context::BattleGuiContext, pokedex::engine::{Context, ContextBuilder, audio::MusicId, util::{HEIGHT, WIDTH}, graphics::{self, Color, DrawParams}}};

use crate::engine::log::info;
use pokedex::context::PokedexClientData;
use state::StateManager;

extern crate firecore_battle as battlelib;
extern crate firecore_world as worldlib;

pub const TITLE: &str = "Pokemon FireRed";
pub const DEBUG_NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DEFAULT_SCALE: f32 = 3.0;

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
    init::logger();

    info!("Starting {} v{}", TITLE, VERSION);
    info!("By {}", AUTHORS);

    let args = args();

    #[cfg(debug_assertions)]
    if !args.contains(&Args::NoSeed) {
        // init::seed_random(engine::util::date() % 1000000)
    }

    #[cfg(feature = "discord")]
    use discord_rich_presence::{activity::Activity, new_client, DiscordIpc};

    #[cfg(feature = "discord")]
    let mut client = {
        let mut client = new_client("862413316420665386")
            .unwrap_or_else(|err| panic!("Could not create discord IPC client with error {}", err));
        client
            .connect()
            .unwrap_or_else(|err| panic!("Could not connect to discord with error {}", err));
        client
            .set_activity(Activity::new().state("test state").details("test details"))
            .unwrap_or_else(|err| panic!("Could not set client activity with error {}", err));
        client
    };

    let debug = cfg!(debug_assertions);
    let save_locally = cfg!(debug_assertions);

    #[cfg(feature = "audio")]
    let audio = !args.contains(&Args::DisableAudio);

    engine::run(
        ContextBuilder::new(
            TITLE,
            (WIDTH * DEFAULT_SCALE) as _,
            (HEIGHT * DEFAULT_SCALE) as _,
        ), // .resizable(true)
        // .show_mouse(true)
        move |mut ctx| async move {
            

            info!("Loading configuration...");

            let configuration = Configuration::load(&mut ctx, save_locally)
                .await
                .unwrap_or_else(|err| panic!("Cannot load configuration with error {}", err));

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
            if audio {
                info!("Loading audio...");
                //Load audio files and setup audio
                match bincode::deserialize::<HashMap<MusicId, Vec<u8>>>(include_bytes!("../build/data/audio.bin")) {
                    Ok(audio_data) => {
                        graphics::draw_text_left(&mut ctx, &0, "Loading audio...", 5.0, 5.0, DrawParams::color(Color::WHITE));
                        for (id, data) in audio_data {
                            engine::audio::add_music(&mut ctx, id, data).await;
                        }
                        
                    }
                    Err(err) => engine::log::error!("Could not read sound file with error {}", err),
                }
            } else {
                info!("Skipping audio loading...");
            }

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
                .await
                .unwrap_or_else(|err| panic!("Could not initialize dex engine with error {}", err));

            let btl = BattleGuiContext::new(&mut ctx).unwrap();

            let mut saves = PlayerSaves::load(save_locally)
                .await
                .unwrap_or_else(|err| panic!("Could not load player saves with error {}", err));

            saves.select_first_or_default(
                save_locally,
                &mut random,
                crate::pokedex(),
                crate::movedex(),
                crate::itemdex(),
            );

            info!("Initialized game context!");

            GameContext {
                engine: ctx,
                random,
                dex,
                btl,
                configuration,
                saves,
                save_locally,
                debug,
            }
        },
        |ctx| StateManager::new(ctx, args),
    );

    #[cfg(feature = "discord")]
    client.close().unwrap();
}

pub struct GameContext {
    pub engine: Context,
    pub random: SmallRng,
    pub dex: PokedexClientData,
    pub btl: BattleGuiContext,
    pub configuration: Configuration,
    pub saves: PlayerSaves,
    pub save_locally: bool,
    pub debug: bool,
}

impl Deref for GameContext {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.engine
    }
}

impl DerefMut for GameContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine
    }
}

#[derive(PartialEq)]
pub enum Args {
    DisableAudio,
    Debug,
    #[cfg(debug_assertions)]
    NoSeed,
}

pub fn args() -> Vec<Args> {
    let mut list = Vec::new();

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut args = pico_args::Arguments::from_env();

        if args.contains("-a") {
            list.push(Args::DisableAudio);
        }

        if args.contains("-d") {
            list.push(Args::Debug);
        }

        #[cfg(debug_assertions)]
        if args.contains("-s") {
            list.push(Args::NoSeed);
        }
    }

    list
}
