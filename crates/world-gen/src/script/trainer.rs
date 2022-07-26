use std::{num::ParseIntError, str::ParseBoolError};

use hashbrown::HashMap;

use serde::{Deserialize, Serialize};

pub mod party;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Trainer {
    pub party_flags: String,
    pub class: String,
    pub music: String,
    pub pic: String,
    pub name: Option<String>,
    pub items: Vec<String>,
    pub double_battle: bool,
    /// bitflag
    pub ai_flags: Vec<String>,
    /// alias
    pub party: String,
}

pub fn parse_trainers(file: &str) -> Result<HashMap<String, Trainer>, TrainerError> {
    let mut lines = file.lines().enumerate().peekable();

    lines.next();

    let mut id = None;
    let mut current = None;

    let mut trainers = HashMap::new();

    while let Some((line, text)) = lines.next() {
        match text.trim() {
            "};" => break,
            _ => match id.is_some() {
                false => {
                    let (lb, ..) = text
                        .char_indices()
                        .find(|(.., c)| c == &'[')
                        .ok_or_else(|| TrainerError::BracketParse(line, "id"))?;
                    let (rb, ..) = text
                        .char_indices()
                        .find(|(.., c)| c == &']')
                        .ok_or_else(|| TrainerError::BracketParse(line, "id"))?;
                    id = Some(text[lb + 1..rb].to_owned());
                }
                true => match text.trim() {
                    "}," | "}" => {
                        let id = id.take().unwrap();
                        let trainer = current.take().unwrap();
                        trainers.insert(id, trainer);
                    }
                    _ => {
                        let trainer = current.get_or_insert(Trainer::default());
                        let (left, right) = text
                            .split_once('=')
                            .ok_or_else(|| TrainerError::FieldParse(line, text.to_owned()))?;
                        let (left, right) = (left.trim(), right.trim());
                        let right = &right[..right.len() - 1];
                        match left {
                            ".partyFlags" => {
                                trainer.party_flags = right.to_owned();
                            }
                            ".trainerClass" => trainer.class = right.to_owned(),
                            ".encounterMusic_gender" => trainer.music = right.to_owned(),
                            ".trainerPic" => trainer.pic = right.to_owned(),
                            // To - do: trainer name
                            ".trainerName" => {
                                let mut split = right.split('"').skip(1);
                                let name = split.next().ok_or_else(|| {
                                    TrainerError::FieldParse(line, right.to_owned())
                                })?;
                                if name.len() != 0 {
                                    trainer.name = Some(name.to_owned());
                                }
                            }
                            // To - do: items
                            ".items" => {
                                let (lb, ..) = right
                                    .char_indices()
                                    .find(|(.., c)| c == &'{')
                                    .ok_or_else(|| TrainerError::BracketParse(line, "items"))?;
                                let (rb, ..) = right
                                    .char_indices()
                                    .find(|(.., c)| c == &'}')
                                    .ok_or_else(|| TrainerError::BracketParse(line, "items"))?;
                                let array = &right[lb + 1..rb];
                                trainer.items = array.split(',').map(str::to_owned).collect();
                            }
                            ".doubleBattle" => {
                                let right = right.to_ascii_lowercase();
                                trainer.double_battle = right.parse::<bool>().map_err(|err| {
                                    TrainerError::BoolParse(line, "double_battle", err)
                                })?
                            }
                            ".aiFlags" => {
                                trainer.ai_flags =
                                    right.split('|').map(str::trim).map(str::to_owned).collect()
                            }
                            ".partySize" => (),
                            ".party" => {
                                let (lb, ..) =
                                    right.char_indices().find(|(.., c)| c == &'{').ok_or_else(
                                        || TrainerError::BracketParse(line, "party (left"),
                                    )?;
                                let rb = right
                                    .char_indices()
                                    .find(|(.., c)| c == &'}')
                                    .ok_or_else(|| TrainerError::BracketParse(line, "party"))
                                    .map(|(rb, ..)| rb)
                                    .unwrap_or(right.len());
                                let party = &right[lb + 1..rb];
                                if party.len() != 0 {
                                    let (.., id) = party.split_once('=').ok_or_else(|| {
                                        TrainerError::FieldParse(line, party.to_owned())
                                    })?;
                                    let id = id.trim();
                                    trainer.party = id.to_owned();
                                }
                            }
                            field => {
                                return Err(TrainerError::UnknownField(line, field.to_owned()))
                            }
                        }
                    }
                },
            },
        }
    }

    Ok(trainers)
}

#[derive(Debug)]
pub enum TrainerError {
    BracketParse(usize, &'static str),
    FieldParse(usize, String),
    NumParse(usize, &'static str, ParseIntError),
    BoolParse(usize, &'static str, ParseBoolError),
    UnknownField(usize, String),
    UnknownMacro(usize, String),
    DefineError(usize, &'static str),
}

impl std::error::Error for TrainerError {}

impl std::fmt::Display for TrainerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrainerError::BracketParse(line, field) => {
                write!(
                    f,
                    "Could not parse field {} at line {} with bracket error",
                    field, line
                )
            }
            TrainerError::FieldParse(line, text) => write!(
                f,
                "Could not parse field at line {} with text: {}",
                line, text
            ),
            TrainerError::NumParse(line, field, err) => write!(
                f,
                "Could not parse number for field \"{}\" at line {} with error {}",
                field, line, err
            ),
            TrainerError::BoolParse(line, field, err) => write!(
                f,
                "Could not parse boolean for field \"{}\" at line {} with error {}",
                field, line, err
            ),
            TrainerError::UnknownField(line, field) => {
                write!(f, "Found unknown field \"{}\" at line {}", field, line)
            }
            TrainerError::UnknownMacro(line, macro_) => {
                write!(f, "Found unknown macro \"{}\" at line {}", macro_, line)
            }
            TrainerError::DefineError(line, error) => write!(f, "Cannot parse define macro at {} with error \"{}\"", line, error),
        }
    }
}
