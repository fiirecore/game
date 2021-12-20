use std::hash::Hash;

use assets::AssetContext;
use battlelib::{
    default_engine::{scripting::MoveScripts, EngineMoves},
    pokedex::{item::Item, moves::Move, pokemon::Pokemon, BasicDex},
};
pub(crate) use firecore_battle_gui::pokedex;
pub(crate) use pokedex::engine;
use worldlib::serialized::SerializedWorld;

// mod args;
mod assets;
mod battle;
mod command;
mod game;
mod saves;
mod state;
mod world;

extern crate firecore_storage as storage;

use rand::prelude::{SeedableRng, SmallRng};

use crate::saves::PlayerSaves;
use game::config::Configuration;

use firecore_battle_gui::{
    context::BattleGuiData,
    pokedex::engine::{
        graphics::{self, Color, DrawParams},
        utils::{HEIGHT, WIDTH, HashSet},
        ContextBuilder,
    },
};

use crate::engine::log::info;
use pokedex::PokedexClientData;
use state::StateManager;

extern crate firecore_battle as battlelib;
extern crate firecore_world as worldlib;

const TITLE: &str = "Pokemon FireRed";
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");

const PUBLISHER: Option<&str> = Some("fiirecore");
const APPLICATION: &str = env!("CARGO_PKG_NAME");

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
            info!("Loading configuration...");
            let configuration = storage::try_load(PUBLISHER, APPLICATION)
                .await
                .unwrap_or_else(|err| panic!("Cannot load configuration with error {}", err));

            info!("Loading assets (this may take a while)...");
            let assets = AssetContext::load()
                .await
                .unwrap_or_else(|err| panic!("Could not load assets with error {}", err));

            info!("Loading player saves...");
            let saves = PlayerSaves::load()
                .await
                .unwrap_or_else(|err| panic!("Could not load player saves with error {}", err));

            OpenContext {
                assets,
                configuration,
                saves,
            }
        },
        move |ctx,
              OpenContext {
                  assets,
                  configuration,
                  saves,
              }| {
            info!("Starting {} v{}", TITLE, VERSION);
            info!("By {}", AUTHORS);

            unsafe {
                POKEDEX = Some(assets.pokedex);

                MOVEDEX = Some(assets.movedex);

                ITEMDEX = Some(assets.itemdex);
            }

            info!("Initializing fonts...");

            for font in assets.fonts {
                engine::text::insert_font(ctx, &font).unwrap();
            }

            #[cfg(feature = "audio")]
            {
                info!("Initializing audio...");
                //Load audio files and setup audio

                graphics::draw_text_left(
                    ctx,
                    &0,
                    "Loading audio...",
                    5.0,
                    5.0,
                    DrawParams::color(Color::WHITE),
                );
                for (id, data) in assets.audio {
                    engine::audio::add_music(ctx, id, data);
                }
            }

            graphics::clear(ctx, Color::BLACK);

            let random = SmallRng::seed_from_u64(engine::utils::seed());

            info!("Initializing dex textures and audio...");

            let dex = PokedexClientData::new(ctx, assets.dex)
                .unwrap_or_else(|err| panic!("Could not initialize dex data with error {}", err));

            let btl = BattleGuiData::new(ctx).unwrap_or_else(|err| {
                panic!("Could not initialize battle data with error {}", err)
            });

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
                battle: assets.battle,
                btl,
                world: assets.world,
                random,
            }
        },
        StateManager::new,
    );

    #[cfg(feature = "discord")]
    client.close().unwrap();
}

struct OpenContext {
    assets: AssetContext,
    configuration: Configuration,
    saves: PlayerSaves,
}

pub(crate) struct LoadContext {
    pub configuration: Configuration,
    pub saves: PlayerSaves,
    pub dex: PokedexClientData,
    pub battle: (EngineMoves, MoveScripts),
    pub btl: BattleGuiData,
    pub world: SerializedWorld,
    pub random: SmallRng,
}

// pub(crate) struct EventReciver<T: Eq + Hash>(pub std::rc::Rc<std::cell::RefCell<EventReciverInner<T>>>);

// impl<T: Eq + Hash> Default for EventReciver<T> {
//     fn default() -> Self {
//         Self(Default::default())
//     }
// }

// impl<T: Eq + Hash> Clone for EventReciver<T> {
//     fn clone(&self) -> Self {
//         Self(self.0.clone())
//     }
// }




pub(crate) struct EventReceiver<T: Eq + Hash> {
    pub long: HashSet<T>,
    pub short: HashSet<T>,
}


impl<T: Eq + Hash> EventReceiver<T> {
    pub fn flush(&mut self) {
        self.short.clear();
    }

    pub fn contains(&self, x: &T) -> bool
    {
        self.short.contains(x) || self.long.contains(x)
    }

}

impl<T: Eq + Hash> Default for EventReceiver<T> {
    fn default() -> Self {
        Self {
            long: Default::default(),
            short: Default::default(),
        }
    }
}
