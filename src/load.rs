use crate::engine::{
    graphics::{self, Color, DrawParams},
    log::info,
    self, Context, EngineContext,
};
use crate::{APPLICATION, AUTHORS, PUBLISHER, TITLE, VERSION};
use storage::RonSerializer;

use crate::{config::Configuration, saves::Player};

use crate::pokengine::PokedexClientData;

use crate::battle::default_engine::{scripting::MoveScripts, EngineMoves};

use battlecli::BattleGuiData;

use worldlib::serialized::SerializedWorld;

mod assets;
use assets::*;

pub struct OpenContext {
    assets: AssetContext,
    configuration: Configuration,
    save: Option<Player>,
}

pub struct LoadContext {
    pub configuration: Configuration,
    pub save: Option<Player>,
    pub dex: PokedexClientData,
    pub battle: (EngineMoves, MoveScripts),
    pub btl: BattleGuiData,
    pub world: SerializedWorld,
}

impl OpenContext {
    pub async fn load() -> Self {
        info!("Loading configuration...");
        let configuration =
            storage::try_load::<RonSerializer, Configuration>(PUBLISHER, APPLICATION)
                .await
                .unwrap_or_else(|err| panic!("Cannot load configuration with error {}", err));

        info!("Loading assets (this may take a while)...");
        let assets = AssetContext::load()
            .await
            .unwrap_or_else(|err| panic!("Could not load assets with error {}", err));

        info!("Loading player saves...");
        let save = storage::try_load::<RonSerializer, Player>(PUBLISHER, APPLICATION)
            .await
            .ok();

        OpenContext {
            assets,
            configuration,
            save,
        }
    }
}

impl LoadContext {
    pub fn load(
        ctx: &mut Context,
        eng: &mut EngineContext,
        OpenContext {
            assets,
            configuration,
            save,
        }: OpenContext,
    ) -> Self {
        info!("Starting {} v{}", TITLE, VERSION);
        info!("By {}", AUTHORS);

        unsafe {
            crate::dex::POKEDEX = Some(assets.pokedex);

            crate::dex::MOVEDEX = Some(assets.movedex);

            crate::dex::ITEMDEX = Some(assets.itemdex);
        }

        info!("Initializing fonts...");

        for font in assets.fonts {
            engine::text::insert_font(ctx, eng, &font).unwrap();
        }

        #[cfg(feature = "audio")]
        {
            info!("Initializing audio...");
            //Load audio files and setup audio

            graphics::draw_text_left(
                ctx,
                eng,
                &0,
                "Loading audio...",
                5.0,
                5.0,
                DrawParams::color(Color::WHITE),
            );
            for (id, data) in assets.audio {
                engine::music::add_music(ctx, eng, id, data);
            }
        }

        graphics::clear(ctx, Color::BLACK);

        info!("Initializing dex textures and audio...");

        let dex = PokedexClientData::new(ctx, eng, assets.dex)
            .unwrap_or_else(|err| panic!("Could not initialize dex data with error {}", err));

        let btl = BattleGuiData::new(ctx)
            .unwrap_or_else(|err| panic!("Could not initialize battle data with error {}", err));

        #[cfg(feature = "discord")]
        use discord_rich_presence::{activity::Activity, new_client, DiscordIpc};

        #[cfg(feature = "discord")]
        let mut client = {
            let mut client = new_client("862413316420665386").unwrap_or_else(|err| {
                panic!("Could not create discord IPC client with error {}", err)
            });
            client
                .connect()
                .unwrap_or_else(|err| panic!("Could not connect to discord with error {}", err));
            client
                .set_activity(Activity::new().state("test state").details("test details"))
                .unwrap_or_else(|err| panic!("Could not set client activity with error {}", err));
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

        Self {
            configuration,
            save,
            dex,
            battle: assets.battle,
            btl,
            world: assets.world,
        }
    }
}
