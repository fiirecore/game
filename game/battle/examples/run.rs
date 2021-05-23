extern crate firecore_game as game;

use std::collections::BTreeMap;

use crate::{
    util::{
        WIDTH,
        HEIGHT,
        smallvec::SmallVec,
    },
    pokedex::pokemon::{
        PokemonId,
        Pokemon,
        saved::{SavedPokemon, PokemonData},
        data::StatSet,
        instance::PokemonInstance,
        GeneratePokemon,
        texture::PokemonTexture,
    },
    battle::BattleData,
    gui::{
        party::PartyGui,
        bag::BagGui,
    },
    textures::pokemon_texture,
    macroquad::{
        self,
        camera::{
            set_camera,
            Camera2D,
        },
        prelude::{
            Conf,
            Rect,
            clear_background,
            BLACK,
            draw_text,
            WHITE,
            get_frame_time,
            next_frame,
            coroutines::{
                start_coroutine,
                stop_coroutine,
            },
            vec2,
            is_key_down,
            KeyCode,
            Texture2D,
            screen_width,
            screen_height,
        },
        ui::{
            hash,
            root_ui,
            widgets::{self, Button},
        }
    }
};

use firecore_battle::manager::BattleManager;

const AUDIO_PATH: &str = "examples/audio.bin";
const DEX_PATH: &str = "../build/data/dex.bin";
const FONTS_PATH: &str = "../build/data/fonts.bin";

#[macroquad::main(config)]
async fn main() {

    let loading = start_coroutine(async {
        loop {
            clear_background(BLACK);
            draw_text("Loading...", 300.0, 210.0, 20.0, WHITE);
            next_frame().await;
        }
    });

    game::input::keyboard::load(game::input::keyboard::default_key_map());

    game::init::text(
        postcard::from_bytes(
            &game::macroquad::prelude::load_file(
                FONTS_PATH,
            ).await.unwrap()
        ).unwrap()
    );

    game::init::audio(
        postcard::from_bytes(
            &game::macroquad::prelude::load_file(
                AUDIO_PATH,
            ).await.unwrap()
        ).unwrap()
    );
    
    game::init::pokedex(
        postcard::from_bytes(
            &game::macroquad::prelude::load_file(
                DEX_PATH,
            ).await.unwrap()
        ).unwrap()
    );

    stop_coroutine(loading);

    let w = 80.0;
    let origin = vec2(w, 0.0);
    let size = vec2(screen_width() - w * 2.0, screen_height());
    let mut player: Vec<&Pokemon> = Vec::with_capacity(6);
    let mut opponent: Vec<&Pokemon> = Vec::with_capacity(6);
    let pokemon: BTreeMap<PokemonId, (&Pokemon, Texture2D)> = game::pokedex::pokedex().iter().map(|(id, pokemon)| (*id, (pokemon, pokemon_texture(id, PokemonTexture::Icon)))).collect();
    let mut finished = false;

    loop {

        clear_background(BLACK);

        widgets::Window::new(hash!(), vec2(0.0, 0.0), vec2(w, screen_height()))
            .movable(false)
            .label("Player party")
            .ui(&mut root_ui(), |ui| {
                if Button::new("Start").position(vec2(10.0, 5.0)).size(vec2(60.0, 20.0)).ui(ui) {
                    finished = true;
                }
                for id in 0..player.len() {
                    widgets::Group::new(hash!("playerparty", id), vec2(60.0, 60.0))
                    .position(vec2(10.0, 30.0 + id as f32 * 70.0)).ui(ui, |ui| {
                        if let Some(pokemon) = player.get(id) {
                            ui.label(vec2(10.0, 10.0), &pokemon.data.name);
                        }
                        if ui.button(vec2(20.0, 30.0), "Remove") {
                            player.remove(id);
                        }
                    });
                }
            });

        widgets::Window::new(hash!(), vec2(screen_width() - w, 0.0), vec2(w, screen_height()))
            .movable(false)
            .label("Opponent party")
            .ui(&mut root_ui(), |ui| {
                for id in 0..opponent.len() {
                    widgets::Group::new(hash!("opponentparty", id), vec2(60.0, 60.0))
                    .ui(ui, |ui| {
                        ui.label(vec2(10.0, 10.0), &opponent[id].data.name);
                        if ui.button(vec2(20.0, 30.0), "Remove") {
                            opponent.remove(id);
                        }
                    });
                }
            });

        widgets::Window::new(hash!(), origin, size)
            .movable(false)
            .label("Choose Pokemon")
            .ui(&mut root_ui(), |ui| {
                for id in pokemon.keys() {
                    widgets::Group::new(hash!("pokemon", id), vec2(200.0, 100.0)).ui(ui, |ui| {
                        ui.label(vec2(10.0, 10.0), &pokemon.get(id).unwrap().0.data.name);
                        if ui.button(vec2(10.0, 30.0), "Add to player party") {
                            if player.len() < 6 {
                                player.push(pokemon.get(id).unwrap().0)
                            }
                        }
                        if ui.button(vec2(10.0, 50.0), "Add to opponent party") {
                            if opponent.len() < 6 {
                                opponent.push(pokemon.get(id).unwrap().0)
                            }
                        }
                    });
                }
            });

        if finished {
            break;
        }

        next_frame().await;

    }
    
    set_camera(Camera2D::from_display_rect(Rect::new(0.0, 0.0, WIDTH, HEIGHT)));

    let mut party_gui = PartyGui::new();
    let mut bag_gui = BagGui::new();
    let mut manager = BattleManager::new();

    let player: SmallVec<[SavedPokemon; 6]> = player.into_iter().map(|pokemon| {
        SavedPokemon {
            id: pokemon.data.id,
            data: PokemonData {
                nickname: None,
                level: 30,
                gender: pokemon.generate_gender(),
                status: None,
                ivs: StatSet::uniform(15),
                evs: StatSet::default(),
                experience: 500,
                friendship: 70,
            },
            item: None,
            moves: None,
            current_hp: None,
            owned_data: None,
        }
    }).collect();

    let opponent: SmallVec<[PokemonInstance; 6]> = opponent.into_iter().map(|pokemon| {
        PokemonInstance::generate_with_level(pokemon.data.id, 30, None)
    }).collect();

    manager.battle(&player, 
    BattleData {
        party: opponent,
        trainer: None,
    });

    while !manager.finished() {

        clear_background(BLACK);

        let delta = if is_key_down(KeyCode::Space) { get_frame_time() * 8.0 } else { get_frame_time() };        

        if party_gui.alive() {
            party_gui.input();
        } else if bag_gui.alive() {
            bag_gui.input(&mut party_gui);
        } else {
            manager.input(&mut party_gui, &mut bag_gui);
        }

        manager.update(delta, &mut party_gui, &mut bag_gui);
        manager.render();
        
        party_gui.update(delta);
        party_gui.render();

        bag_gui.update(&mut party_gui);
        party_gui.render();

        next_frame().await;

    }

}

fn config() -> Conf {
    Conf {
        window_title: "Battle Test".to_owned(),
        window_width: (game::util::WIDTH * 3.0) as i32,
        window_height: (game::util::HEIGHT * 3.0) as i32,
        ..Default::default()
    }
}