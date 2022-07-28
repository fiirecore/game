use battle::pokedex::item::{Item, ItemCategory, ItemId, Price};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;

const ITEMS: &str = "https://raw.githubusercontent.com/pret/pokefirered/master/src/data/items.json";

const ICONS: &str =
    "https://raw.githubusercontent.com/pret/pokefirered/master/graphics/items/icons";

pub type ItemTextures = hashbrown::HashMap<ItemId, Vec<u8>>;

#[derive(Deserialize)]
struct JsonItems {
    items: Vec<JsonItem>,
}

#[derive(Deserialize)]
struct JsonItem {
    #[serde(rename = "english")]
    name: String,
    #[serde(rename = "itemId")]
    item_id: String,
    price: Price,

    // hold_effect: String,
    // hold_effect_param: isize,
    #[serde(rename = "description_english")]
    description: String,

    pocket: String,
}

pub fn generate() -> Vec<Item> {
    let items = || {
        Some(
            attohttpc::get(ITEMS)
                .send()
                .ok()?
                .json::<JsonItems>()
                .unwrap(),
        )
    };

    let items = (items)().map(|i| i.items).unwrap_or_default();

    items
        .into_par_iter()
        .flat_map(|item| {
            if item
                .name
                .get(0..1)
                .map(|s| s.eq_ignore_ascii_case("?"))
                .unwrap_or(true)
            {
                return None;
            }

            println!("Creating item entry for {}", item.name);

            let idstr = item.item_id[5..].to_ascii_lowercase();
            let id = match idstr.parse() {
                Ok(id) => id,
                Err(err) => {
                    eprintln!("Cannot parse item id {} with error {}", idstr, err);
                    return None;
                }
            };

            let mut name = item.name.to_ascii_lowercase();

            unsafe {
                let bytes = name.as_bytes_mut();

                let str = std::str::from_utf8_unchecked_mut(bytes);

                str[0..1].make_ascii_uppercase();

                for (index, c) in str.char_indices() {
                    if c == ' ' {
                        let str = &mut *(str as *const str as *mut str);
                        if let Some(x) = str.get_mut(index + 1..index + 2) {
                            x.make_ascii_uppercase();
                        }
                    }
                }
            }

            Some(Item {
                id,
                name,
                description: item.description,
                category: match item.pocket.as_str() {
                    "POCKET_POKE_BALLS" => ItemCategory::Pokeballs,
                    "POCKET_KEY_ITEMS" => ItemCategory::KeyItems,
                    _ => ItemCategory::Items,
                },
                price: item.price,
                stackable: Default::default(),
                consume: true,
                usage: Default::default(),
            })
        })
        .collect()
}

#[cfg(feature = "client-data")]
pub fn generate_client(client: std::sync::Arc<pokerust::Client>) -> ItemTextures {
    let items = || {
        Some(
            attohttpc::get(ITEMS)
                .send()
                .ok()?
                .json::<JsonItems>()
                .unwrap(),
        )
    };

    let items = (items)().map(|i| i.items).unwrap_or_default();

    items
        .into_par_iter()
        .flat_map(|item| {
            if item
                .name
                .get(0..1)
                .map(|s| s.eq_ignore_ascii_case("?"))
                .unwrap_or(true)
            {
                return None;
            }

            println!("Creating item entry for {}", item.name);

            let idstr = item.item_id[5..].to_ascii_lowercase();
            let id = match idstr.parse() {
                Ok(id) => id,
                Err(err) => {
                    eprintln!("Cannot parse item id {} with error {}", idstr, err);
                    return None;
                }
            };

            let mut name = item.name.to_ascii_lowercase();

            unsafe {
                let bytes = name.as_bytes_mut();

                let str = std::str::from_utf8_unchecked_mut(bytes);

                str[0..1].make_ascii_uppercase();

                for (index, c) in str.char_indices() {
                    if c == ' ' {
                        let str = &mut *(str as *const str as *mut str);
                        if let Some(x) = str.get_mut(index + 1..index + 2) {
                            x.make_ascii_uppercase();
                        }
                    }
                }
            }

            let url = format!("{}/{}.png", ICONS, idstr);

            let mut texture = match attohttpc::get(url).send().ok()?.bytes() {
                Ok(bytes) => bytes,
                Err(err) => {
                    eprintln!("Could not get texture for item {} with error {}", name, err);
                    return None;
                }
            };

            if texture == "404: Not Found".as_bytes() {
                let url = idstr.replace('_', "-");
                match client.get::<pokerust::Item, &str>(&url) {
                    Ok(item) => match attohttpc::get(item.sprites.default).send().ok()?.bytes() {
                        Ok(bytes) => texture = bytes,
                        Err(err) => {
                            eprintln!("Could not get texture for item {} with error {}", name, err);
                            return None;
                        }
                    },
                    Err(err) => {
                        eprintln!("Cannot get item {} from PokeAPI with error {}", name, err);
                        return None;
                    }
                }
            }

            Some((id, texture))
        })
        .collect()
}
