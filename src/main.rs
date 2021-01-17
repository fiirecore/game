use std::sync::Mutex;

use io::data::configuration::Configuration;

pub static TITLE: &str =  env!("CARGO_PKG_NAME");
pub static AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static BASE_WIDTH: u32 = 240;
pub static BASE_HEIGHT: u32 = 160;

lazy_static::lazy_static! {
    pub static ref CONFIGURATION: Mutex<Configuration> = Mutex::new(Configuration::new());
    pub static ref WINDOW_SCALE: Mutex<u8> = Mutex::new(Configuration::get().scale);
    // pub static ref ASSET_CACHE: AssetCache = AssetCache::new("assets").expect("Error creating asset cache!");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut game = crate::app::Game::new();

    game.run()?;

    Ok(())
    
}

pub mod app;

pub mod util;

pub mod audio {
    pub mod music;
    pub mod sound;
}

pub mod scene;

pub mod entity;

pub mod io;

pub mod world;

pub mod game {

    pub mod game_manager;

    pub mod player_data_container;

    pub mod pokedex {
        pub mod pokedex;
        pub mod pokemon {
            pub mod pokemon_instance;
            pub mod pokemon_owned;
        }
        pub mod move_instance;
    }

}

pub mod battle;

mod gui {

    pub mod gui;

    pub mod basic_button;

    pub mod battle {

        pub mod battle_gui;

        pub mod battle_background;
        pub mod health_bar;
        pub mod pokemon_gui;
        pub mod battle_text;
        pub mod player_bounce;

        pub mod panels {
            pub mod player_panel;
            pub mod battle_panel;
            pub mod fight_panel;
            pub mod move_panel;
        }
    
    }

    pub mod game {
        pub mod pokemon_party_gui;
    }

}


