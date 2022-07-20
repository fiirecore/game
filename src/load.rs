use std::{ops::Deref, path::PathBuf};

use crate::{
    pokedex::{item::Item, moves::Move, pokemon::Pokemon, BasicDex},
    engine::{notan::prelude::{AssetList, AssetLoader, Assets}, utils::HashMap},
    pokengine::SerializedPokedexEngine
};

use battlecli::battle::default_engine::{scripting::MoveScripts, EngineMoves};

use worldcli::worldlib::serialized::SerializedWorld;

pub enum LoadData<
    P: Deref<Target = Pokemon> + From<Pokemon>,
    M: Deref<Target = Move> + From<Move>,
    I: Deref<Target = Item> + From<Item>,
> {
    Load(AssetList, HashMap<&'static str, String>),
    Data {
        pokedex: BasicDex<Pokemon, P>,
        movedex: BasicDex<Move, M>,
        itemdex: BasicDex<Item, I>,
    },
}

pub struct Data {
    pub battle: (EngineMoves, MoveScripts),
    pub dex: SerializedPokedexEngine,
    pub world: SerializedWorld,
    pub audio: crate::engine::utils::HashMap<crate::engine::music::MusicId, Vec<u8>>,
}

impl<
        P: Deref<Target = Pokemon> + Clone + From<Pokemon>,
        M: Deref<Target = Move> + Clone + From<Move>,
        I: Deref<Target = Item> + Clone + From<Item>,
    > LoadData<P, M, I>
{
    pub const POKEDEX: &'static str = "pokedex.bin";
    pub const MOVEDEX: &'static str = "movedex.bin";
    pub const ITEMDEX: &'static str = "itemdex.bin";
    pub const WORLD: &'static str = "world.bin";
    pub const BATTLE: &'static str = "battle.bin";
    pub const DEXENGINE: &'static str = "dex_engine.bin";
    pub const AUDIO: &'static str = "audio.bin";

    pub fn load(assets: &mut Assets) -> Result<Self, String> {
        let root = root()
            .ok_or_else(|| String::from("Could not get root assets path!"))?
            .join("assets");

        let mut map2 = HashMap::new();

        let map = &mut map2;

        fn create(
            map: &mut HashMap<&'static str, String>,
            root: &PathBuf,
            path: &'static str,
        ) -> String {
            let output = root.join(path).as_os_str().to_string_lossy().to_string();
            map.insert(path, output.clone());
            output
        }

        let pokedex = create(map, &root, Self::POKEDEX);
        let movedex = create(map, &root, Self::MOVEDEX);
        let itemdex = create(map, &root, Self::ITEMDEX);
        let world = create(map, &root, Self::WORLD);
        let dexengine = create(map, &root, Self::DEXENGINE);
        let battle = create(map, &root, Self::BATTLE);
        let audio = create(map, &root, Self::AUDIO);

        let list = assets.load_list(&[
            &pokedex, &movedex, &itemdex, &world, &dexengine, &battle, &audio,
        ])?;

        Ok(Self::Load(list, map2))

        // info!("Loading dexes...");
        // let (pokedex, movedex, itemdex) = get(path.join("dex.bin"))?;

        // info!("Loading battle data...");
        // let battle = get(path.join("battle.bin"))?;

        // let dex = get(path.join("dex_engine.bin"))?;
        // let world = get(path.join("world.bin"))?;

        //
        // let audio = get(path.join("audio.bin"))?;

        // Ok(Self {
        //     dex,
        //     world,
        //     pokedex,
        //     movedex,
        //     itemdex,
        //     battle,
        //
        //     audio,
        // })
    }

    pub fn is_loaded(&self) -> Option<bool> {
        match self {
            LoadData::Load(list, ..) => Some(list.is_loaded()),
            _ => None,
        }
    }

    pub fn percentage(&self) -> Option<f32> {
        match self {
            LoadData::Load(list, ..) => Some(list.progress()),
            LoadData::Data { .. } => None,
        }
    }

    pub fn finish(&mut self) -> Option<Data> {
        match self.is_loaded() {
            Some(true) => {
                if let LoadData::Load(list, map) = self {
                    fn deser<D: serde::de::DeserializeOwned>(
                        list: &mut AssetList,
                        map: &HashMap<&'static str, String>,
                        id: &str,
                    ) -> Result<D, String> {
                        postcard::from_bytes(
                            &list
                                .take::<Vec<u8>>(
                                    map.get(id)
                                        .ok_or(String::from("Could not get id from map"))?,
                                )?
                                .try_unwrap()?,
                        )
                        .map_err(|err| err.to_string())
                    }

                    let pokedex = deser(list, map, Self::POKEDEX).unwrap();
                    let movedex = deser(list, map, Self::MOVEDEX).unwrap();
                    let itemdex = deser(list, map, Self::ITEMDEX).unwrap();

                    let battle = deser(list, map, Self::BATTLE).unwrap();
                    let dex = deser(list, map, Self::DEXENGINE).unwrap();
                    let world = deser(list, map, Self::WORLD).unwrap();
                    let audio = deser(list, map, Self::AUDIO).unwrap();

                    *self = Self::Data {
                        pokedex,
                        movedex,
                        itemdex,
                    };

                    Some(Data {
                        battle,
                        dex,
                        world,
                        audio,
                    })
                } else {
                    unreachable!()
                }
            }
            _ => None,
        }
    }
}

// pub struct LoadContext {
//     pub pokedex: BasicDex<Pokemon, Rc<Pokemon>>,
//     pub movedex: BasicDex<Move, Rc<Move>>,
//     pub itemdex: BasicDex<Item, Rc<Item>>,
//     pub dex: PokedexClientData,
//     pub battle: (EngineMoves, MoveScripts),
//     pub btl: BattleGuiData,
//     pub world: SerializedWorld,
// }

// impl OpenContext {
//     pub fn load() -> Self {
//         info!("Loading configuration...");
//         let configuration = //engine::notan::prelude::AssetList::
//             // storage::try_load::<RonSerializer, Configuration>(PUBLISHER, APPLICATION)
//             //     .unwrap_or_else(|err| panic!("Cannot load configuration with error {}", err));

//         info!("Loading assets (this may take a while)...");
//         let assets = AssetContext::load()
//             .unwrap_or_else(|err| panic!("Could not load assets with error {}", err));

//         info!("Loading player saves...");
//         // let save = storage::try_load::<RonSerializer, Player>(PUBLISHER, APPLICATION).ok();

//         OpenContext {
//             assets,
//             configuration,
//             save,
//         }
//     }
// }

// impl LoadContext {
//     pub fn load(
//         app: &mut App,
//         plugins: &mut Plugins,
//         gfx: &mut Graphics,
//         OpenContext {
//             assets,
//             configuration,
//             save,
//         }: OpenContext,
//     ) -> Self {
//         info!("Starting {} v{}", TITLE, VERSION);
//         info!("By {}", AUTHORS);

//
//         {
//             info!("Initializing audio...");
//             //Load audio files and setup audio

//             // graphics::draw_text_left(
//             //     ctx,
//             //     eng,
//             //     &0,
//             //     "Loading audio...",
//             //     5.0,
//             //     5.0,
//             //     DrawParams::color(Color::WHITE),
//             // );
//             for (id, data) in assets.audio {
//                 engine::music::add_music(app, plugins, id, data);
//             }
//         }

//         // graphics::clear(ctx, Color::BLACK);

//         info!("Initializing dex textures and audio...");

//         let dex = PokedexClientData::build(app, plugins, gfx, assets.dex)
//             .unwrap_or_else(|err| panic!("Could not initialize dex data with error {}", err));

//         let btl = BattleGuiData::new(gfx)
//             .unwrap_or_else(|err| panic!("Could not initialize battle data with error {}", err));

//         #[cfg(feature = "discord")]
//         use discord_rich_presence::{activity::Activity, new_client, DiscordIpc};

//         #[cfg(feature = "discord")]
//         let mut client = {
//             let mut client = new_client("862413316420665386").unwrap_or_else(|err| {
//                 panic!("Could not create discord IPC client with error {}", err)
//             });
//             client
//                 .connect()
//                 .unwrap_or_else(|err| panic!("Could not connect to discord with error {}", err));
//             client
//                 .set_activity(Activity::new().state("test state").details("test details"))
//                 .unwrap_or_else(|err| panic!("Could not set client activity with error {}", err));
//             client
//         };

//         // {
//         //     if args.contains(&Args::Debug) {
//         //         set_debug(true);
//         //     }

//         //     if is_debug() {
//         //         info!("Running in debug mode");
//         //     }
//         // }

//         info!("Initialized game context!");

//         Self {
//             dex,
//             battle: assets.battle,
//             btl,
//             world: assets.world,
//             pokedex: assets.pokedex,
//             movedex: assets.movedex,
//             itemdex: assets.itemdex,
//         }
//     }
// }

pub fn root() -> Option<PathBuf> {
    {
        #[cfg(not(target_arch = "wasm32"))]
        {
            dirs::data_dir().map(|d| d.join(crate::PUBLISHER).join(crate::APPLICATION))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Some(Default::default())
        }
    }
}

pub fn load(assets: &mut Assets, path: &str) -> Result<String, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let root = dirs::config_dir()
            .ok_or_else(|| {
                String::from("Could not get configuration and saves directory on this OS!")
            })?
            .join(crate::PUBLISHER)
            .join(crate::APPLICATION)
            .join(path);

        std::fs::read_to_string(
            root.as_os_str()
                .to_str()
                .ok_or_else(|| String::from("Could not get a UTF-8 path!"))?,
        )
        .map_err(|err| err.to_string())
    }
    #[cfg(target_arch = "wasm32")]
    {
        let storage = general_storage_web::LocalStorage::new();
        general_storage_web::Storage::load_raw(&storage, path)
            .map_err(|err| format!("{:?}", err))
            .and_then(|v| String::from_utf8(v).map_err(|err| err.to_string()))
    }
}

pub fn asset_loader() -> AssetLoader {
    AssetLoader::new().use_parser(parse).extension("bin")
}

fn parse(_: &str, data: Vec<u8>) -> Result<Vec<u8>, String> {
    Ok(data)
}
