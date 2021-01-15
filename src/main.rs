fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut game = crate::app::Game::new();

    game.run()?;

    Ok(())
    
}

pub mod app;

pub mod engine {

    pub mod engine;
    pub mod game_context;

    pub mod text;
    pub mod text_renderer_load;

}

pub mod util;

pub mod audio {
    pub mod music;
    pub mod sound;
}

pub mod scene {
    pub mod scene;
    pub mod scene_manager;
    pub mod scenes {
        pub mod first_scene;
        pub mod character_creation_scene;
        pub mod firsttime_scenes;
        pub mod game_scene;
        pub mod loading_scenes;
        pub mod main_menu_scene;
        pub mod title_scene;
    }
}

pub mod entity {

    pub mod entity;

    pub mod util;

    pub mod entities {
        pub mod player;
    }

    pub mod texture {
        pub mod still_texture_manager;
        pub mod movement_texture;
        pub mod movement_texture_manager;
        pub mod texture_manager;
        pub mod four_way_texture;
        pub mod three_way_texture;
    }
}

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


