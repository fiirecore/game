use hashbrown::HashMap;

use serde::{Deserialize, Serialize};

use super::TrainerError;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrainerPokemon {
    pub ivs: u8,
    pub level: u8,
    pub species: String,
    pub moves: Option<Vec<String>>,
    pub item: Option<String>,
}

enum State {
    File,
    Define,
    Party,
    Pokemon,
}

pub fn parse_parties(file: &str) -> Result<HashMap<String, Vec<TrainerPokemon>>, TrainerError> {
    // let file = if file.contains("#define") {
    //     let mut defines = HashMap::new();
    //     let mut range = None;
    //     let mut removals = Vec::new();
    //     let mut defining = false;
    //     let mut name = None;
    //     let mut lines = String::new();
    //     for (line, text) in file.lines().enumerate() {
    //         if text.starts_with("#define") {
    //             defining = true;
    //             range = Some((line, line));
    //         }
    //         if defining {
    //             let (text, return_) = match text.ends_with('\\') {
    //                 true => (&text[..text.len() - 1], false),
    //                 false => (text, true),
    //             };

    //             let text = match name.is_some() {
    //                 true => {
    //                     text
    //                 }
    //                 false => {
    //                     let mut defline = text.split_ascii_whitespace().skip(1);
    //                     name = Some(defline.next().unwrap());
    //                     defline.next().unwrap_or("")
    //                 }
    //             };
    //             lines.push_str(text);
    //             lines.push('\n');

    //             if return_ {
    //                 defines.insert(name.take().unwrap(), std::mem::take(&mut lines));
    //                 removals.push({
    //                     let (a, b) = range.take().unwrap();
    //                     a..=b
    //                 });
    //                 name = None;
    //                 defining = false;
    //                 continue;
    //             }
    //             let r = range.as_mut().unwrap();
    //             r.1 += 1;
    //         }
    //     }

    //     let mut file = file
    //         .lines()
    //         .enumerate()
    //         .filter(|(l, ..)| !removals.iter().any(|r| r.contains(l)))
    //         .flat_map(|(.., l)| [l, "\n"])
    //         .collect::<String>();

    //     for (key, replacement) in defines {
    //         file = file.replace(&key, &replacement);
    //     }

    //     file
    // } else {
    //     file.to_owned()
    // };

    let mut lines = file.lines().enumerate().peekable();

    let mut state = State::File;
    let mut trainers = HashMap::new();
    let mut mons = Vec::new();
    let mut current_trainer = None;
    let mut current_pokemon = None;
    let mut define = None;
    let mut defined = HashMap::<String, TrainerPokemon>::new();

    // let mut previous = *lines.peek().map(|(.., s)| s).unwrap();

    while let Some((line, text)) = lines.next() {
        let text = text.split_once("//").map(|(l, ..)| l).unwrap_or(text);

        match state {
            State::File => match text.trim() {
                "" => continue,
                text => match text.starts_with('#') {
                    true => {
                        if text.starts_with("#define") {
                            let mut defline = text.split_ascii_whitespace().skip(1);
                            match defline.next() {
                                Some(name) => define = Some(name),
                                None => {
                                    return Err(TrainerError::DefineError(
                                        line,
                                        "No name for define macro",
                                    ))
                                }
                            }
                            state = State::Define;
                        } else {
                            return Err(TrainerError::UnknownMacro(
                                line,
                                text.split_ascii_whitespace()
                                    .next()
                                    .unwrap_or(text)
                                    .to_owned(),
                            ));
                        }
                    }
                    false => {
                        let mut words = text.split_ascii_whitespace().skip(4);
                        current_trainer = words.next().map(|s| s[..s.len() - 2].to_owned());
                        if let Some(def) = text.split_once('=').map(|(.., r)| r.trim()) {
                            party(def, &mut state, &defined, &mut trainers, &mut mons, &mut current_trainer);
                        }
                        if current_trainer.is_some() {
                            state = State::Party;
                        }
                    }
                },
            },
            State::Define => {
                let (text, return_) = match text.ends_with('\\') {
                    true => (&text[..text.len() - 1], false),
                    false => (text, true),
                };

                if let Some(pokemon) =
                    pokemon(line, text, &mut state, State::Define, &mut current_pokemon)?
                {
                    defined.insert(define.take().unwrap().to_owned(), pokemon);
                }

                if return_ {
                    state = State::File;
                }
            }
            State::Party => party(
                text,
                &mut state,
                &defined,
                &mut trainers,
                &mut mons,
                &mut current_trainer,
            ),
            State::Pokemon => {
                if let Some(pokemon) =
                    pokemon(line, text, &mut state, State::Party, &mut current_pokemon)?
                {
                    mons.push(pokemon);
                }
            }
        }
    }

    Ok(trainers)
}

fn party(
    text: &str,
    state: &mut State,
    defined: &HashMap<String, TrainerPokemon>,
    trainers: &mut HashMap<String, Vec<TrainerPokemon>>,
    mons: &mut Vec<TrainerPokemon>,
    current_trainer: &mut Option<String>,
) {
    match text.trim() {
        "};" => {
            let pokemon = std::mem::take(mons);
            let name = current_trainer.take().unwrap();

            println!("inserting {}", name);
            trainers.insert(name, pokemon);

            *state = State::File;
        }
        text => {
            if text.starts_with('{') && text.ends_with("};") {
                let def = text;
                println!("{}", def);
                let def = &def[1..def.len() - 2];
                for def in def.split(", ") {
                    if let Some(pokemon) = defined.get(def) {
                        mons.push(pokemon.clone());
                    }
                }
                let pokemon = std::mem::take(mons);
                let name = current_trainer.take().unwrap();
                println!("inserting {}", name);
                trainers.insert(name, pokemon);
                
            } else {
                *state = State::Pokemon;
            }
        }
    }
}

fn pokemon(
    line: usize,
    text: &str,
    state: &mut State,
    superceding: State,
    current_pokemon: &mut Option<TrainerPokemon>,
) -> Result<Option<TrainerPokemon>, TrainerError> {
    let pokemon = current_pokemon.get_or_insert(TrainerPokemon::default());
    match text.trim() {
        "{" => (),
        "}," | "}" => {
            *state = superceding;
            if let Some(current) = current_pokemon.take() {
                return Ok(Some(current));
            }
        }
        _ => {
            let (left, right) = text
                .split_once('=')
                .ok_or_else(|| TrainerError::FieldParse(line, text.to_owned()))?;
            let (left, right) = (left.trim(), right.trim());
            let right = &right[..right.len() - 1];
            match left {
                ".iv" => {
                    pokemon.ivs = right
                        .parse()
                        .map_err(|err| TrainerError::NumParse(line, "ivs", err))?
                }
                ".lvl" => {
                    pokemon.level = right
                        .parse()
                        .map_err(|err| TrainerError::NumParse(line, "level", err))?
                }
                ".species" => pokemon.species = right.to_owned(),
                ".moves" => {
                    let (lb, ..) = right
                        .char_indices()
                        .find(|(.., c)| c == &'{')
                        .ok_or_else(|| TrainerError::BracketParse(line, "moves"))?;
                    let (rb, ..) = right
                        .char_indices()
                        .find(|(.., c)| c == &'}')
                        .ok_or_else(|| TrainerError::BracketParse(line, "moves"))?;
                    let array = &right[lb + 1..rb];
                    pokemon.moves =
                        Some(array.split(',').map(str::trim).map(str::to_owned).collect());
                }
                ".heldItem" => pokemon.item = Some(right.to_owned()),
                field => return Err(TrainerError::UnknownField(line, field.to_owned())),
            }
        }
    }
    Ok(None)
}
