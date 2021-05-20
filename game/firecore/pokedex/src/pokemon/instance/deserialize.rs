use crate::item::ItemRef;
// use crate::item::ItemRef;
use crate::moves::instance::MoveInstanceSet;
use crate::pokemon::stat::BaseStatSet;
use crate::pokemon::instance::PokemonData;
use crate::pokemon::Health;
use crate::pokemon::PokemonRef;

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
            __field5,
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
                    4u64 => serde::__private::Ok(__Field::__field5),
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
                    "data" => serde::__private::Ok(__Field::__field1),
                    "item" => serde::__private::Ok(__Field::__field2),
                    "moves" => serde::__private::Ok(__Field::__field3),
                    "current_hp" => serde::__private::Ok(__Field::__field5),
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
                    b"data" => serde::__private::Ok(__Field::__field1),
                    b"item" => serde::__private::Ok(__Field::__field2),
                    b"moves" => serde::__private::Ok(__Field::__field3),
                    b"current_hp" => serde::__private::Ok(__Field::__field5),
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
                            &"struct PokemonInstance with 5 elements",
                        ));
                    }
                };
                let __field1 = match match serde::de::SeqAccess::next_element::<PokemonData>(
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
                            1usize,
                            &"struct PokemonInstance with 5 elements",
                        ));
                    }
                };
                let __field2 = match match serde::de::SeqAccess::next_element::<
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
                let __field3 = match match serde::de::SeqAccess::next_element::<
                    MoveInstanceSet,
                >(&mut __seq)
                {
                    serde::__private::Ok(__val) => __val,
                    serde::__private::Err(__err) => {
                        return serde::__private::Err(__err);
                    }
                } {
                    serde::__private::Some(__value) => __value,
                    serde::__private::None => moveinstancedefaultgen(__field0, &__field1),
                };
                let __field4 = basestatsetdefaultgen(__field0, &__field1);
                let __field5 =
                    match match serde::de::SeqAccess::next_element::<Health>(&mut __seq) {
                        serde::__private::Ok(__val) => __val,
                        serde::__private::Err(__err) => {
                            return serde::__private::Err(__err);
                        }
                    } {
                        serde::__private::Some(__value) => __value,
                        serde::__private::None => current_hpdefaultgen(&__field4),
                    };
                serde::__private::Ok(PokemonInstance {
                    pokemon: __field0,
                    data: __field1,
                    item: __field2,
                    moves: __field3,
                    base: __field4,
                    current_hp: __field5,
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
                let mut __field1: serde::__private::Option<PokemonData> =
                    serde::__private::None;
                let mut __field2: serde::__private::Option<Option<ItemRef>> =
                    serde::__private::None;
                let mut __field3: serde::__private::Option<MoveInstanceSet> =
                    serde::__private::None;
                let mut __field5: serde::__private::Option<Health> =
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
                                        "data",
                                    ),
                                );
                            }
                            __field1 = serde::__private::Some(
                                match serde::de::MapAccess::next_value::<PokemonData>(
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
                                        "item",
                                    ),
                                );
                            }
                            __field2 = serde::__private::Some(
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
                        __Field::__field3 => {
                            if serde::__private::Option::is_some(&__field3) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "moves",
                                    ),
                                );
                            }
                            __field3 = serde::__private::Some(
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
                        __Field::__field5 => {
                            if serde::__private::Option::is_some(&__field5) {
                                return serde::__private::Err(
                                    <__A::Error as serde::de::Error>::duplicate_field(
                                        "current_hp",
                                    ),
                                );
                            }
                            __field5 = serde::__private::Some(
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
                    serde::__private::None => {
                        match serde::__private::de::missing_field("data") {
                            serde::__private::Ok(__val) => __val,
                            serde::__private::Err(__err) => {
                                return serde::__private::Err(__err);
                            }
                        }
                    }
                };
                let __field2 = match __field2 {
                    serde::__private::Some(__field2) => __field2,
                    serde::__private::None => serde::__private::Default::default(),
                };
                let __field3 = match __field3 {
                    serde::__private::Some(__field3) => __field3,
                    serde::__private::None => moveinstancedefaultgen(__field0, &__field1),
                };

                let base = basestatsetdefaultgen(__field0, &__field1);

                let __field5 = match __field5 {
                    serde::__private::Some(__field5) => __field5,
                    serde::__private::None => current_hpdefaultgen(&base),
                };
                serde::__private::Ok(PokemonInstance {
                    pokemon: __field0,
                    data: __field1,
                    item: __field2,
                    moves: __field3,
                    base,
                    current_hp: __field5,
                })
            }
        }
        const FIELDS: &'static [&'static str] =
            &["id", "data", "item", "moves", "current_hp"];
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
fn moveinstancedefaultgen(pokemon: PokemonRef, data: &PokemonData) -> MoveInstanceSet {
    pokemon.value().generate_moves(data.level)
    // match pokemon {
    //     crate::Ref::Init(pokemon) => pokemon.generate_moves(data.level),
    //     crate::Ref::Uninit(id) => //MoveInstanceSet::Uninit(id),
    // }
}

#[inline]
fn basestatsetdefaultgen(pokemon: PokemonRef, data: &PokemonData) -> BaseStatSet {
    BaseStatSet::get(pokemon.value(), data.ivs, data.evs, data.level)
}

#[inline]
fn current_hpdefaultgen(base: &BaseStatSet) -> Health {
    base.hp
}
