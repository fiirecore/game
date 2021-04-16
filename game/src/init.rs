use data::{store, get, configuration::Configuration, player::PlayerSaves};
use pokedex::{
    POKEDEX,
    MOVEDEX,
    ITEMDEX,
    serialize::SerializedDex,
};
use audio::{
    add_sound,
    SerializedSoundData,
    Sound
};
use firecore_font_lib::SerializedFonts;
use macroquad::prelude::{warn, Texture2D, Image, Rect};

use crate::graphics::text::{TextRenderer, TEXT_RENDERER, Font};
use crate::graphics::{image_texture, byte_texture};
use crate::textures::{PokemonTextures, POKEMON_TEXTURES, ITEM_TEXTURES};
use util::hash::HashMap;

pub fn seed_randoms(seed: u64) {
    pokedex::pokemon::POKEMON_RANDOM.seed(seed);
}

pub async fn data() {
    store::<Configuration>().await;
    store::<PlayerSaves>().await;

    {

        let config = get::<Configuration>().expect("Could not get configuration!");

        input::keyboard::load(config.controls.clone());

        if config.touchscreen {
            input::touchscreen::touchscreen(true);
        }

    }

}

pub fn pokedex(dex: SerializedDex) {

	let mut textures = PokemonTextures::default();

	pokedex::new();

	let pokedex = unsafe { POKEDEX.as_mut().unwrap() };

	for pokemon in dex.pokemon {
		
		textures.front.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.front_png));
		textures.back.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.back_png));
		textures.icon.insert(pokemon.pokemon.data.id, byte_texture(&pokemon.icon_png));

		if !pokemon.cry_ogg.is_empty() {
			if let Err(_) = add_sound(
				SerializedSoundData {
					bytes: pokemon.cry_ogg,
					sound: Sound {
						name: String::from("Cry"),
						variant: Some(pokemon.pokemon.data.id),
					}
				}
			) {
				// warn!("Error adding pokemon cry: {}", err);
			}
		}
		
		pokedex.insert(pokemon.pokemon.data.id, pokemon.pokemon);
	}

	let movedex = unsafe { MOVEDEX.as_mut().unwrap() };

	for pokemon_move in dex.moves {
		movedex.insert(pokemon_move.id, pokemon_move);
	}

    let itemdex = unsafe { ITEMDEX.as_mut().unwrap() };

    let mut item_textures = HashMap::with_capacity(dex.items.len());

    for item in dex.items {
        item_textures.insert(item.item.id, byte_texture(&item.texture));
        itemdex.insert(item.item.id, item.item.item);
    }

    unsafe { ITEM_TEXTURES = Some(item_textures); }

	unsafe { POKEMON_TEXTURES = Some(textures); }

}

#[cfg(feature = "audio")]
pub fn audio(audio: audio::SerializedAudio) {
    use macroquad::prelude::error;

    if let Err(err) = audio::create() {
        error!("{}", err);
    } else {
        #[cfg(not(target = "wasm32"))] {
            std::thread::spawn( || {
                if let Err(err) = audio::load(audio) {
                    error!("Could not load audio files with error {}", err);
                }
            });
        }
    
        #[cfg(target = "wasm32")] {
            if let Err(err) = audio::load(audio) {
                error!("Could not load audio files with error {}", err);
            }
        }
    }    
}

pub fn text(font_sheets: SerializedFonts) {
	let mut text_renderer = TextRenderer::new();

    for font_sheet in font_sheets.fonts {
        text_renderer.fonts.insert(
            font_sheet.data.id, 
            Font {
                font_width: font_sheet.data.width,
                font_height: font_sheet.data.height,
                chars: iterate_fontsheet(
                    font_sheet.data.chars, 
                    font_sheet.data.width, 
                    font_sheet.data.height, 
                    font_sheet.data.custom, 
                    Image::from_file_with_format(&font_sheet.image, None)
                ),
            }
        );
    }

	unsafe { TEXT_RENDERER = Some(text_renderer); }
}

fn iterate_fontsheet(chars: String, font_width: u8, font_height: u8, custom: Vec<firecore_font_lib::CustomChar>, sheet: Image) -> HashMap<char, Texture2D> {

    let mut customchars: HashMap<char, (u8, Option<u8>)> = custom.into_iter().map(|cchar| (cchar.id, (cchar.width, cchar.height))).collect();

    let chars: Vec<char> = chars.chars().collect();
    let sheet_width = sheet.width() as f32;
    let sheet_height = sheet.height() as f32;// - font_height as u32;

    let mut charmap = HashMap::with_capacity(chars.len());

    let mut counter: usize = 0;
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;

    'yloop: while y < sheet_height {
        while x < sheet_width {
            if let Some(cchar) = customchars.remove(&chars[counter]) {
                charmap.insert(chars[counter], image_texture(&sheet.sub_image(Rect::new(x, y, cchar.0 as f32, cchar.1.unwrap_or(font_height) as f32))));
            } else {
                charmap.insert(chars[counter], image_texture(&sheet.sub_image(Rect::new(x, y, font_width as f32, font_height as f32))));
            }
            x += font_width as f32;
            counter+=1;
            if counter >= chars.len() {
                break 'yloop;
            }
        }
        x = 0.0;
        y += font_height as f32;
    }

    charmap
}