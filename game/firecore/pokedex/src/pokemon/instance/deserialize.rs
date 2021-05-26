use crate::item::ItemRef;
use crate::moves::instance::MoveInstanceSet;
use crate::pokemon::{
    PokemonRef,
    Health,
    Level,
    Experience,
    Friendship,
    data::Gender,
    instance::Nickname,
    stat::{Stats, BaseStats},
    status::StatusEffect,
    default_iv,
    default_friendship,
};

use super::PokemonInstance;


impl<'de> serde::Deserialize<'de> for PokemonInstance {
    fn deserialize<__D>(__deserializer: __D) -> serde::__private::Result<Self, __D::Error>
    where
        __D: serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __field3,
            __field4,
            __field5,
            __field6,
            __field7,
            __field8,
            __field9,
            __field10,
            __field12,
            __ignore,
        }
        struct __FieldVisitor;
        impl<'de> serde::de::Visitor<'de> for __FieldVisitor {
            type Value = __Field;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(__formatter, "field identifier")
            }
            fn visit_u64<__E>(
                self,
                __value: u64,
            ) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    0u64 => serde::__private::Ok(__Field::__field0),
                    1u64 => serde::__private::Ok(__Field::__field1),
                    2u64 => serde::__private::Ok(__Field::__field2),
                    3u64 => serde::__private::Ok(__Field::__field3),
                    4u64 => serde::__private::Ok(__Field::__field4),
                    5u64 => serde::__private::Ok(__Field::__field5),
                    6u64 => serde::__private::Ok(__Field::__field6),
                    7u64 => serde::__private::Ok(__Field::__field7),
                    8u64 => serde::__private::Ok(__Field::__field8),
                    9u64 => serde::__private::Ok(__Field::__field9),
                    10u64 => serde::__private::Ok(__Field::__field10),
                    11u64 => serde::__private::Ok(__Field::__field12),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_str<__E>(
                self,
                __value: &str,
            ) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    "id" => serde::__private::Ok(__Field::__field0),
                    "nickname" => serde::__private::Ok(__Field::__field1),
                    "level" => serde::__private::Ok(__Field::__field2),
                    "gender" => serde::__private::Ok(__Field::__field3),
                    "ivs" => serde::__private::Ok(__Field::__field4),
                    "evs" => serde::__private::Ok(__Field::__field5),
                    "experience" => serde::__private::Ok(__Field::__field6),
                    "friendship" => serde::__private::Ok(__Field::__field7),
                    "moves" => serde::__private::Ok(__Field::__field8),
                    "status" => serde::__private::Ok(__Field::__field9),
                    "item" => serde::__private::Ok(__Field::__field10),
                    "current_hp" => serde::__private::Ok(__Field::__field12),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
            fn visit_bytes<__E>(
                self,
                __value: &[u8],
            ) -> serde::__private::Result<Self::Value, __E>
            where
                __E: serde::de::Error,
            {
                match __value {
                    b"id" => serde::__private::Ok(__Field::__field0),
                    b"nickname" => serde::__private::Ok(__Field::__field1),
                    b"level" => serde::__private::Ok(__Field::__field2),
                    b"gender" => serde::__private::Ok(__Field::__field3),
                    b"ivs" => serde::__private::Ok(__Field::__field4),
                    b"evs" => serde::__private::Ok(__Field::__field5),
                    b"experience" => serde::__private::Ok(__Field::__field6),
                    b"friendship" => serde::__private::Ok(__Field::__field7),
                    b"moves" => serde::__private::Ok(__Field::__field8),
                    b"status" => serde::__private::Ok(__Field::__field9),
                    b"item" => serde::__private::Ok(__Field::__field10),
                    b"current_hp" => serde::__private::Ok(__Field::__field12),
                    _ => serde::__private::Ok(__Field::__ignore),
                }
            }
        }
        impl<'de> serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> serde::__private::Result<Self, __D::Error>
            where
                __D: serde::Deserializer<'de>,
            {
                serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }
        struct __Visitor<'de> {
            marker: serde::__private::PhantomData<PokemonInstance>,
            lifetime: serde::__private::PhantomData<&'de ()>,
        }
        impl<'de> serde::de::Visitor<'de> for __Visitor<'de> {
            type Value = PokemonInstance;
            fn expecting(
                &self,
                __formatter: &mut serde::__private::Formatter,
            ) -> serde::__private::fmt::Result {
                serde::__private::Formatter::write_str(
                    __formatter,
                    "struct PokemonInstance",
                )
            }
            #[inline]
            fn visit_seq<__A>(
                self,
                mut __seq: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::SeqAccess<'de>,
            {
                let __field0 = match match serde::de::SeqAccess::next_element::<PokemonRef>(
                    &mut __seq,
                ) {
                    serde::__private::Ok(__val) => __val,
                    serde::__private::Err(__err) => {
                        return serde::__private::Err(__err);
                    }
                } {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => {
                        return serde::__private::Err(serde::de::Error::invalid_length(
                            0usize,
                            &"struct PokemonInstance with 12 elements",
                        ));
                    }
                };
                let __field1 =
                    match match serde::de::SeqAccess::next_element::<Nickname>(&mut __seq)
                    {
                        serde::__private::Ok(__val) => __val,
                        serde::__private::Err(__err) => {
                            return serde::__private::Err(__err);
                        }
                    } {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => serde::__private::Default::default(),
                    };
                let __field2 =
                    match match serde::de::SeqAccess::next_element::<Level>(&mut __seq) {
                        serde::__private::Ok(__val) => __val,
                        serde::__private::Err(__err) => {
                            return serde::__private::Err(__err);
                        }
                    } {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => {
                            return serde::__private::Err(
                                serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct PokemonInstance with 12 elements",
                                ),
                            );
                        }
                    };
                let __field3 =
                    match match serde::de::SeqAccess::next_element::<Gender>(&mut __seq) {
                        serde::__private::Ok(__val) => __val,
                        serde::__private::Err(__err) => {
                            return serde::__private::Err(__err);
                        }
                    } {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => default_gender(__field0),
                    };
                let __field4 = match match serde::de::SeqAccess::next_element::<Stats>(
                    &mut __seq,
                ) {
                    serde::__private::Ok(__val) => __val,
                    serde::__private::Err(__err) => {
                        return serde::__private::Err(__err);
                    }
                } {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => default_iv(),
                };
                let __field5 = match match serde::de::SeqAccess::next_element::<Stats>(
                    &mut __seq,
                ) {
                    serde::__private::Ok(__val) => __val,
                    serde::__private::Err(__err) => {
                        return serde::__private::Err(__err);
                    }
                } {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => serde::__private::Default::default(),
                };
                let __field6 = match match serde::de::SeqAccess::next_element::<Experience>(
                    &mut __seq,
                ) {
                    serde::__private::Ok(__val) => __val,
                    serde::__private::Err(__err) => {
                        return serde::__private::Err(__err);
                    }
                } {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => serde::__private::Default::default(),
                };
                let __field7 = match match serde::de::SeqAccess::next_element::<Friendship>(
                    &mut __seq,
                ) {
                    serde::__private::Ok(__val) => __val,
                    serde::__private::Err(__err) => {
                        return serde::__private::Err(__err);
                    }
                } {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => default_friendship(),
                };
                let __field8 = match match serde::de::SeqAccess::next_element::<
                    MoveInstanceSet,
                >(&mut __seq)
                {
                    serde::__private::Ok(__val) => __val,
                    serde::__private::Err(__err) => {
                        return serde::__private::Err(__err);
                    }
                } {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => default_moves(__field0, __field2),
                };
                let __field9 = match match serde::de::SeqAccess::next_element::<
                    Option<StatusEffect>,
                >(&mut __seq)
                {
                    serde::__private::Ok(__val) => __val,
                    serde::__private::Err(__err) => {
                        return serde::__private::Err(__err);
                    }
                } {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => serde::__private::Default::default(),
                };
                let __field10 = match match serde::de::SeqAccess::next_element::<
                    Option<ItemRef>,
                >(&mut __seq)
                {
                    serde::__private::Ok(__val) => __val,
                    serde::__private::Err(__err) => {
                        return serde::__private::Err(__err);
                    }
                } {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => serde::__private::Default::default(),
                };
                let __field11 = default_base(__field0, &__field4, &__field5, __field2);
                let __field12 =
                    match match serde::de::SeqAccess::next_element::<Health>(&mut __seq) {
                        serde::__private::Ok(__val) => __val,
                        serde::__private::Err(__err) => {
                            return serde::__private::Err(__err);
                        }
                    } {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => default_current_hp(&__field11),
                    };
                serde::__private::Ok(PokemonInstance {
                    pokemon: __field0,
                    nickname: __field1,
                    level: __field2,
                    gender: __field3,
                    ivs: __field4,
                    evs: __field5,
                    experience: __field6,
                    friendship: __field7,
                    moves: __field8,
                    status: __field9,
                    persistent: None,
                    item: __field10,
                    base: __field11,
                    current_hp: __field12,
                })
            }
            #[inline]
            fn visit_map<__A>(
                self,
                mut __map: __A,
            ) -> serde::__private::Result<Self::Value, __A::Error>
            where
                __A: serde::de::MapAccess<'de>,
            {
                let mut __field0: serde::__private::Option<PokemonRef> =
                    serde::__private::None;
                let mut __field1: serde::__private::Option<Nickname> =
                    serde::__private::None;
                let mut __field2: serde::__private::Option<Level> =
                    serde::__private::None;
                let mut __field3: serde::__private::Option<Gender> =
                    serde::__private::None;
                let mut __field4: serde::__private::Option<Stats> =
                    serde::__private::None;
                let mut __field5: serde::__private::Option<Stats> =
                    serde::__private::None;
                let mut __field6: serde::__private::Option<Experience> =
                    serde::__private::None;
                let mut __field7: serde::__private::Option<Friendship> =
                    serde::__private::None;
                let mut __field8: serde::__private::Option<MoveInstanceSet> =
                    serde::__private::None;
                let mut __field9: serde::__private::Option<Option<StatusEffect>> =
                    serde::__private::None;
                let mut __field10: serde::__private::Option<Option<ItemRef>> =
                    serde::__private::None;
                let mut __field12: serde::__private::Option<Health> =
                    serde::__private::None;
                while let serde::__private::Some(__key) =
                    match serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                        serde::__private::Ok(__val) => __val,
                        serde::__private::Err(__err) => {
                            return serde::__private::Err(__err);
                        }
                    }
                {
                    match __key {
                        __Field::__field0 => {
                            if serde::__private::Option::is_some(&__field0) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "id",
                                    ),
                                );
                            }
                            __field0 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<PokemonRef>(
                                    &mut __map,
                                ) {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        __Field::__field1 => {
                            if serde::__private::Option::is_some(&__field1) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "nickname",
                                    ),
                                );
                            }
                            __field1 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<Nickname>(
                                    &mut __map,
                                ) {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        __Field::__field2 => {
                            if serde::__private::Option::is_some(&__field2) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "level",
                                    ),
                                );
                            }
                            __field2 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<Level>(&mut __map)
                                {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        __Field::__field3 => {
                            if serde::__private::Option::is_some(&__field3) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "gender",
                                    ),
                                );
                            }
                            __field3 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<Gender>(
                                    &mut __map,
                                ) {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        __Field::__field4 => {
                            if serde::__private::Option::is_some(&__field4) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "ivs",
                                    ),
                                );
                            }
                            __field4 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<Stats>(
                                    &mut __map,
                                ) {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        __Field::__field5 => {
                            if serde::__private::Option::is_some(&__field5) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "evs",
                                    ),
                                );
                            }
                            __field5 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<Stats>(
                                    &mut __map,
                                ) {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        __Field::__field6 => {
                            if serde::__private::Option::is_some(&__field6) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "experience",
                                    ),
                                );
                            }
                            __field6 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<Experience>(
                                    &mut __map,
                                ) {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        __Field::__field7 => {
                            if serde::__private::Option::is_some(&__field7) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "friendship",
                                    ),
                                );
                            }
                            __field7 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<Friendship>(
                                    &mut __map,
                                ) {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        __Field::__field8 => {
                            if serde::__private::Option::is_some(&__field8) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "moves",
                                    ),
                                );
                            }
                            __field8 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<MoveInstanceSet>(
                                    &mut __map,
                                ) {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        __Field::__field9 => {
                            if serde::__private::Option::is_some(&__field9) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "status",
                                    ),
                                );
                            }
                            __field9 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<
                                    Option<StatusEffect>,
                                >(&mut __map)
                                {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        __Field::__field10 => {
                            if serde::__private::Option::is_some(&__field10) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "item",
                                    ),
                                );
                            }
                            __field10 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<Option<ItemRef>>(
                                    &mut __map,
                                ) {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        __Field::__field12 => {
                            if serde::__private::Option::is_some(&__field12) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "current_hp",
                                    ),
                                );
                            }
                            __field12 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<Health>(
                                    &mut __map,
                                ) {
                                    serde::__private::Ok(__val) => __val,
                                    serde::__private::Err(__err) => {
                                        return serde::__private::Err(__err);
                                    }
                                },
                            );
                        }
                        _ => {
                            let _ = match serde::de::MapAccess::next_value::<
                                serde::de::IgnoredAny,
                            >(&mut __map)
                            {
                                serde::__private::Ok(__val) => __val,
                                serde::__private::Err(__err) => {
                                    return serde::__private::Err(__err);
                                }
                            };
                        }
                    }
                }
                let __field0 = match __field0 {
                    serde::__private::Some(__field0) => __field0,
                    serde::__private::None => {
                        match serde::__private::de::missing_field("id") {
                            serde::__private::Ok(__val) => __val,
                            serde::__private::Err(__err) => {
                                return serde::__private::Err(__err);
                            }
                        }
                    }
                };
                let __field1 = match __field1 {
                    serde::__private::Some(__field1) => __field1,
                    serde::__private::None => serde::__private::Default::default(),
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => {
                        match serde::__private::de::missing_field("level") {
                            serde::__private::Ok(__val) => __val,
                            serde::__private::Err(__err) => {
                                return serde::__private::Err(__err);
                            }
                        }
                    }
                };
                let __field3 = match __field3 {
                    serde::__private::Some(__field3) => __field3,
                    serde::__private::None => default_gender(__field0),
                };
                let __field4 = match __field4 {
                    serde::__private::Some(__field4) => __field4,
                    serde::__private::None => default_iv(),
                };
                let __field5 = match __field5 {
                    serde::__private::Some(__field5) => __field5,
                    serde::__private::None => serde::__private::Default::default(),
                };
                let __field6 = match __field6 {
                    serde::__private::Some(__field6) => __field6,
                    serde::__private::None => serde::__private::Default::default(),
                };
                let __field7 = match __field7 {
                    serde::__private::Some(__field7) => __field7,
                    serde::__private::None => default_friendship(),
                };
                let __field8 = match __field8 {
                    serde::__private::Some(__field8) => __field8,
                    serde::__private::None => default_moves(__field0, __field2),
                };
                let __field9 = match __field9 {
                    serde::__private::Some(__field9) => __field9,
                    serde::__private::None => serde::__private::Default::default(),
                };
                let __field10 = match __field10 {
                    serde::__private::Some(__field10) => __field10,
                    serde::__private::None => serde::__private::Default::default(),
                };

                let base = default_base(__field0, &__field4, &__field5, __field2);

                let __field12 = match __field12 {
                    serde::__private::Some(__field12) => __field12,
                    serde::__private::None => default_current_hp(&base),
                };
                serde::__private::Ok(PokemonInstance {
                    pokemon: __field0,
                    nickname: __field1,
                    level: __field2,
                    gender: __field3,
                    ivs: __field4,
                    evs: __field5,
                    experience: __field6,
                    friendship: __field7,
                    moves: __field8,
                    status: __field9,
                    persistent: None,
                    item: __field10,
                    base,
                    current_hp: __field12,
                })
            }
        }
        const FIELDS: &'static [&'static str] = &[
            "id",
            "nickname",
            "level",
            "gender",
            "ivs",
            "evs",
            "experience",
            "friendship",
            "moves",
            "status",
            "item",
            "current_hp",
        ];
        serde::Deserializer::deserialize_struct(
            __deserializer,
            "PokemonInstance",
            FIELDS,
            __Visitor {
                marker: serde::__private::PhantomData::<PokemonInstance>,
                lifetime: serde::__private::PhantomData,
            },
        )
    }
}


#[inline]
fn default_gender(pokemon: PokemonRef) -> Gender {
    pokemon.unwrap().generate_gender()
}

#[inline]
fn default_moves(pokemon: PokemonRef, level: Level) -> MoveInstanceSet {
    pokemon.unwrap().generate_moves(level)
}

#[inline]
fn default_base(pokemon: PokemonRef, ivs: &Stats, evs: &Stats, level: Level) -> BaseStats {
    BaseStats::new(pokemon.unwrap(), ivs, evs, level)
}

#[inline]
fn default_current_hp(base: &BaseStats) -> Health {
    base.hp()
}