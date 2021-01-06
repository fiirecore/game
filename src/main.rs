//#![windows_subsystem = "windows"]

extern crate piston;
extern crate piston_window;
extern crate opengl_graphics;
extern crate sdl2_window;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate image;
extern crate oorandom;
extern crate music;
extern crate log;
extern crate simplelog;

//#[macro_use]
//extern crate include_dir;
//pub static ASSET_DIR: include_dir::Dir = include_dir!("./assets");

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

pub mod util {
    pub mod file_util;
    pub mod image_util;
    pub mod render_util;
    pub mod texture_util;
    pub mod map_util;
    pub mod traits;
    pub mod map_traits;
    pub mod timer;
}

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

    pub mod util {
        pub mod mob_coordinates;
        pub mod direction;
    }

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

pub mod io {

    pub mod app_console;

    pub mod data {
        pub mod configuration;
        pub mod player_data;
        pub mod game_data;
        pub mod pokemon_party;
        pub mod saved_pokemon;
        pub mod saved_pokemon_move;
        pub mod trainer;
        //pub mod saved_pokemon_move_set;
    }

    pub mod map {

        pub mod map_serializable;
        pub mod npc_serializable;

        pub mod map_loader;
        pub mod jigsaw_map_loader;
        pub mod warp_map_loader;

        pub mod warp_loader;
        pub mod npc_loader;
        pub mod encounter_loader;

        pub mod gba_map;
        pub mod json_map {
            pub mod v1;
        }
    }
}

pub mod game {

    pub mod game_manager;

    pub mod world {

        pub mod world_manager;
        pub mod world_manager_load;
        pub mod world_manager_input;
        pub mod world_manager_warp;
    
        pub mod world_map {
            pub mod world_map;
            pub mod world_map_piece;
            pub mod world_map_manager;
        }
    
        pub mod warp_map {
            pub mod warp_map_set;
            pub mod warp_map;
            pub mod warp_map_manager;
        }

        pub mod pokemon {
            pub mod wild_pokemon_encounter;
            pub mod wild_pokemon_table;
            pub mod original_wild_pokemon_table;
            pub mod random_wild_pokemon_table;
        }

        pub mod gui {
            pub mod map_window_manager;
            pub mod player_world_gui;
        }    
        
    }

    pub mod warp {
        pub mod warp_entry;
        pub mod warp_transition;
    }

    pub mod battle {
        pub mod battle;
        pub mod battle_manager;
        pub mod battle_pokemon;
        pub mod battle_info;
        pub mod battle_context;

        pub mod transitions {

            pub mod battle_transition_traits;

            pub mod managers {
                pub mod battle_screen_transition_manager;
                pub mod battle_opener_manager;
                pub mod battle_introduction_manager;
                pub mod battle_closer_manager;
            }
            
            pub mod screen_transitions {
                pub mod flash_battle_screen_transition;
                pub mod trainer_battle_screen_transition;
                pub mod vertical_close_battle_screen_transition;
            }
            
            pub mod openers {             
                pub mod trainer_battle_opener;
                pub mod wild_battle_opener;
            }

            pub mod introductions {
                pub mod trainer_battle_introduction;
                pub mod basic_battle_introduction;
                pub mod util {
                    pub mod player_intro;
                    pub mod intro_text;
                }
            }
            
            pub mod closers {
                pub mod basic_battle_closer;
            }

        }

    }

    pub mod pokedex {
        pub mod pokedex;
        pub mod pokemon {
            pub mod pokemon;
            pub mod pokemon_instance;
            pub mod pokemon_owned;
            pub mod pokemon_toml;
            pub mod stat_set;
        }
        pub mod pokemon_move {
            pub mod move_category;
            pub mod pokemon_move;
            pub mod move_instance;
            pub mod move_toml;
        }
    }

    pub mod npc {
        pub mod npc;
    }

}

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
}


