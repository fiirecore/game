use std::marker::PhantomData;
use core::fmt::Formatter;
use serde::de::Error;
use serde::__private::de::missing_field;
use serde::de::MapAccess;
use serde::de::SeqAccess;

use crate::item::ItemRef;
use crate::moves::instance::MoveInstanceSet;
use crate::pokemon::Level;
use crate::pokemon::data::Gender;
use crate::pokemon::instance::Nickname;
use crate::pokemon::stat::BaseStatSet;
use crate::pokemon::instance::data::PokemonData;
use crate::pokemon::Health;
use crate::pokemon::PokemonRef;

use super::PokemonInstance;

const STRUCT: &str = "PokemonInstance";

const ELEMENTS: &str = "struct PokemonInstance with 8 elements";

// Fields

const ID: &str = "id";
const ID_BYTES: &[u8] = ID.as_bytes();

const NICKNAME: &str = "nickname";
const NICKNAME_BYTES: &[u8] = NICKNAME.as_bytes();

const LEVEL: &str = "level";
const LEVEL_BYTES: &[u8] = LEVEL.as_bytes();

const GENDER: &str = "gender";
const GENDER_BYTES: &[u8] = GENDER.as_bytes();

const DATA: &str = "data";
const DATA_BYTES: &[u8] = DATA.as_bytes();

const MOVES: &str = "moves";
const MOVES_BYTES: &[u8] = MOVES.as_bytes();

const ITEM: &str = "item";
const ITEM_BYTES: &[u8] = ITEM.as_bytes();

const CURRENT_HP: &str = "current_hp";
const CURRENT_HP_BYTES: &[u8] = CURRENT_HP.as_bytes();

impl<'de> serde::Deserialize<'de> for PokemonInstance {

    fn deserialize<D: serde::Deserializer<'de>,>(deserializer: D) -> Result<Self, D::Error> {
        enum Field {
            Id,
            Nickname,
            Level,
            Gender,
            Data,
            Moves,
            Item,
            CurrentHp,
            Ignore,
        }

        struct FieldVisitor;

        impl<'de> serde::de::Visitor<'de> for FieldVisitor {

            type Value = Field;

            fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
                formatter.write_str("field identifier")
            }

            fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
                match value {
                    0 => Ok(Field::Id),
                    1 => Ok(Field::Nickname),
                    2 => Ok(Field::Level),
                    3 => Ok(Field::Gender),
                    4 => Ok(Field::Data),
                    5 => Ok(Field::Moves),
                    6 => Ok(Field::Item),
                    7 => Ok(Field::CurrentHp),
                    _ => Ok(Field::Ignore),
                }
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                match value {
                    ID => Ok(Field::Id),
                    NICKNAME => Ok(Field::Nickname),
                    LEVEL => Ok(Field::Level),
                    GENDER => Ok(Field::Gender),
                    DATA => Ok(Field::Data),
                    MOVES => Ok(Field::Moves),
                    ITEM => Ok(Field::Item),
                    CURRENT_HP => Ok(Field::CurrentHp),
                    _ => Ok(Field::Ignore),
                }
            }

            fn visit_bytes<E: Error>(self, value: &[u8]) -> Result<Self::Value, E> {
                match value {
                    ID_BYTES => Ok(Field::Id),
                    NICKNAME_BYTES => Ok(Field::Nickname),
                    LEVEL_BYTES => Ok(Field::Level),
                    GENDER_BYTES => Ok(Field::Gender),
                    DATA_BYTES => Ok(Field::Data),
                    MOVES_BYTES => Ok(Field::Moves),
                    ITEM_BYTES => Ok(Field::Item),
                    CURRENT_HP_BYTES => Ok(Field::CurrentHp),
                    _ => Ok(Field::Ignore),
                }
            }

        }

        impl<'de> serde::Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct Visitor<'de> {
            marker: PhantomData<PokemonInstance>,
            lifetime: PhantomData<&'de ()>,
        }

        impl<'de> serde::de::Visitor<'de> for Visitor<'de> {

            type Value = PokemonInstance;

            fn expecting(&self, formatter: &mut Formatter) -> core::fmt::Result {
                formatter.write_fmt(format_args!("struct {}", STRUCT))
            }
            #[inline]
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {

                let pokemon = match seq.next_element::<PokemonRef>()? {
                    Some(value) => value,
                    None => {
                        return Err(Error::invalid_length(0, &ELEMENTS));
                    },
                };

                let nickname = match seq.next_element::<Nickname>()? {
                    Some(value) => value,
                    None => Default::default(),
                };

                let level = match seq.next_element::<Level>()? {
                    Some(value) => value,
                    None => {
                        return Err(Error::invalid_length(2, &ELEMENTS));
                    },
                };

                let gender = match seq.next_element::<Gender>()? {
                    Some(gender) => gender,
                    None => default_gender(pokemon),
                };


                let data = match seq.next_element::<PokemonData>()? {
                    Some(value) => value,
                    None => Default::default(),
                };

                let item = match seq.next_element::<Option<ItemRef>>()? {
                    Some(item) => item,
                    None => Default::default(),
                };

                let moves = match seq.next_element::<MoveInstanceSet>()? {
                    Some(moves) => moves,
                    None => default_moves(pokemon, level),
                };

                let base = default_base(pokemon, &data, level);

                let current_hp = match seq.next_element::<Health>()? {
                    Some(value) => value,
                    None => default_current_hp(&base),
                };

                Ok(PokemonInstance {
                    pokemon,
                    nickname,
                    level,
                    gender,
                    data,
                    item,
                    moves,
                    base,
                    current_hp,
                })
            }

            #[inline]
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut pokemon: Option<PokemonRef> = None;
                let mut nickname: Option<Nickname> = None;
                let mut level: Option<Level> = None;
                let mut gender: Option<Gender> = None;
                let mut data: Option<PokemonData> = None;
                let mut item: Option<Option<ItemRef>> = None;
                let mut moves: Option<MoveInstanceSet> = None;
                let mut current_hp: Option<Health> = None;

                while let Some(key) = map.next_key::<Field>()? {
                    match key {

                        Field::Id => {
                            if pokemon.is_some() {
                                return Err(A::Error::duplicate_field(ID));
                            }
                            pokemon = Some(map.next_value::<PokemonRef>()?);
                        },

                        Field::Nickname => {
                            if nickname.is_some() {
                                return Err(A::Error::duplicate_field(NICKNAME));
                            }
                            nickname = Some(map.next_value::<Nickname>()?);
                        },

                        Field::Level => {
                            if level.is_some() {
                                return Err(A::Error::duplicate_field(LEVEL));
                            }
                            level = Some(map.next_value::<Level>()?);
                        },

                        Field::Gender => {
                            if gender.is_some() {
                                return Err(A::Error::duplicate_field(GENDER));
                            }
                            gender = Some(map.next_value::<Gender>()?);
                        }

                        Field::Data => {
                            if data.is_some() {
                                return Err(A::Error::duplicate_field(DATA));
                            }
                            data = Some(map.next_value::<PokemonData>()?);
                        }

                        Field::Moves => {
                            if moves.is_some() {
                                return Err(A::Error::duplicate_field(MOVES));
                            }
                            moves = Some(map.next_value::<MoveInstanceSet>()?);
                        },

                        Field::Item => {
                            if item.is_some() {
                                return Err(A::Error::duplicate_field(ITEM));
                            }
                            item = Some(map.next_value::<Option<ItemRef>>()?);
                        },

                        Field::CurrentHp => {
                            if current_hp.is_some() {
                                return Err(A::Error::duplicate_field(CURRENT_HP));
                            }
                            current_hp = Some(map.next_value::<Health>()?);
                        },

                        Field::Ignore => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }

                    }
                }

                let pokemon = match pokemon {
                    Some(pokemon) => pokemon,
                    None => missing_field(ID)?,
                };
                

                let nickname = match nickname {
                    Some(nickname) => nickname,
                    None => Default::default(),
                };

                let level = match level {
                    Some(level) => level,
                    None => missing_field(LEVEL)?,
                };
                
                let gender = match gender {
                    Some(gender) => gender,
                    None => default_gender(pokemon),
                };

                let data = match data {
                    Some(data) => data,
                    None => Default::default(),
                };

                let moves = match moves {
                    Some(moves) => moves,
                    None => default_moves(pokemon, level),
                };

                let item = match item {
                    Some(item) => item,
                    None => Default::default(),
                };

                let base = default_base(pokemon, &data, level);

                let current_hp = match current_hp {
                    Some(current_hp) => current_hp,
                    None => default_current_hp(&base),
                };


                Ok(PokemonInstance {
                    pokemon,
                    nickname,
                    level,
                    gender,
                    data,
                    moves,
                    item,
                    base,
                    current_hp,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            ID, 
            NICKNAME, 
            LEVEL, 
            GENDER, 
            DATA, 
            MOVES, 
            ITEM, 
            CURRENT_HP
        ];

        deserializer.deserialize_struct(
            STRUCT,
            FIELDS,
            Visitor {
                marker: PhantomData::<PokemonInstance>,
                lifetime: PhantomData,
            },
        )

    }
}

#[inline]
fn default_gender(pokemon: PokemonRef) -> Gender {
    pokemon.value().generate_gender()
}

#[inline]
fn default_moves(pokemon: PokemonRef, level: Level) -> MoveInstanceSet {
    pokemon.value().generate_moves(level)
}

#[inline]
fn default_base(pokemon: PokemonRef, data: &PokemonData, level: Level) -> BaseStatSet {
    BaseStatSet::get(pokemon.value(), data.ivs, data.evs, level)
}

#[inline]
fn default_current_hp(base: &BaseStatSet) -> Health {
    base.hp
}
