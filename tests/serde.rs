#![allow(clippy::blocks_in_conditions, clippy::redundant_static_lifetimes)]
#![cfg(feature = "serde")]

use std::borrow::Cow;
use json_event_parser::{JsonEvent, JsonValueSink, JsonValueSource, ReaderJsonParser, WriterJsonSerializer};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum DemoEnum {
    A,
    Ary2,
    Ary1,
    Ary3,
    Inner,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum DemoValueEnum {
    U8(u8),
    None,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct DemoInnerStruct {
    enum_k: DemoEnum,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct DemoStruct {
    enum_k: DemoEnum,
    inner_struct: DemoInnerStruct,
    array: Vec<DemoEnum>,
    enum_with_value: Vec<DemoValueEnum>,
}

impl DemoStruct {
    fn demo() -> Self {
        Self {
            enum_k: DemoEnum::A,
            inner_struct: DemoInnerStruct {
                enum_k: DemoEnum::Inner,
            },
            array: vec![DemoEnum::Ary1, DemoEnum::Ary2, DemoEnum::Ary3],
            enum_with_value: vec![DemoValueEnum::U8(42), DemoValueEnum::None],
        }
    }
}

#[test]
fn sink_object() {
    let demo_input = DemoStruct::demo();

    let mut write = Vec::<u8>::new();
    let mut json_writer = WriterJsonSerializer::new(&mut write);
    demo_input
        .serialize(JsonValueSink::new(&mut json_writer))
        .unwrap();

    let demo_decode = serde_json::from_slice::<DemoStruct>(&write).unwrap();
    assert_eq!(demo_input, demo_decode);
}

#[test]
fn source_object() {
    let demo_input = DemoStruct::demo();
    let demo_json = serde_json::to_string(&demo_input).unwrap();

    let mut json_reader = ReaderJsonParser::new(demo_json.as_bytes());
    let demo_decoded = DemoStruct::deserialize(JsonValueSource::new(&mut json_reader)).unwrap();
    assert_eq!(demo_input, demo_decoded);
}

#[test]
fn part_source_object() {
    let demo_input = DemoStruct::demo();
    let demo_json = serde_json::to_string(&demo_input).unwrap();
    let composed_demo_json = format!("{{\"x\": 1, \"y\": {}, \"z\": 2}}", demo_json);

    let mut json_reader = ReaderJsonParser::new(composed_demo_json.as_bytes());
    assert_eq!(json_reader.parse_next().unwrap().unwrap(), JsonEvent::StartObject);
    assert_eq!(json_reader.parse_next().unwrap().unwrap(), JsonEvent::ObjectKey(Cow::Borrowed("x")));
    assert_eq!(json_reader.parse_next().unwrap().unwrap(), JsonEvent::Number(Cow::Borrowed("1")));
    assert_eq!(json_reader.parse_next().unwrap().unwrap(), JsonEvent::ObjectKey(Cow::Borrowed("y")));
    let demo_decoded = DemoStruct::deserialize(JsonValueSource::new(&mut json_reader)).unwrap();
    assert_eq!(demo_input, demo_decoded);
    assert_eq!(json_reader.parse_next().unwrap().unwrap(), JsonEvent::ObjectKey(Cow::Borrowed("z")));
    assert_eq!(json_reader.parse_next().unwrap().unwrap(), JsonEvent::Number(Cow::Borrowed("2")));
    assert_eq!(json_reader.parse_next().unwrap().unwrap(), JsonEvent::EndObject);
    assert!(json_reader.parse_next().is_none());
}

// serde derive expansion code below, for debugging purpose only, do **NOT** edit
// if you want update these code, use IDE or something support macro expansion for generated code from serde `derive(Serialize, Deserialize)`

#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    _serde::__require_serde_not_serde_core!();
    #[automatically_derived]
    impl _serde::Serialize for DemoEnum {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private229::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                DemoEnum::A => {
                    _serde::Serializer::serialize_unit_variant(__serializer, "DemoEnum", 0u32, "a")
                }
                DemoEnum::Ary2 => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "DemoEnum",
                    1u32,
                    "ary2",
                ),
                DemoEnum::Ary1 => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "DemoEnum",
                    2u32,
                    "ary1",
                ),
                DemoEnum::Ary3 => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "DemoEnum",
                    3u32,
                    "ary3",
                ),
                DemoEnum::Inner => _serde::Serializer::serialize_unit_variant(
                    __serializer,
                    "DemoEnum",
                    4u32,
                    "inner",
                ),
            }
        }
    }
};

#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    _serde::__require_serde_not_serde_core!();
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for DemoEnum {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private229::Ok(__Field::__field0),
                        1u64 => _serde::__private229::Ok(__Field::__field1),
                        2u64 => _serde::__private229::Ok(__Field::__field2),
                        3u64 => _serde::__private229::Ok(__Field::__field3),
                        4u64 => _serde::__private229::Ok(__Field::__field4),
                        _ => _serde::__private229::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 5",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "a" => _serde::__private229::Ok(__Field::__field0),
                        "ary2" => _serde::__private229::Ok(__Field::__field1),
                        "ary1" => _serde::__private229::Ok(__Field::__field2),
                        "ary3" => _serde::__private229::Ok(__Field::__field3),
                        "inner" => _serde::__private229::Ok(__Field::__field4),
                        _ => _serde::__private229::Err(_serde::de::Error::unknown_variant(
                            __value, VARIANTS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"a" => _serde::__private229::Ok(__Field::__field0),
                        b"ary2" => _serde::__private229::Ok(__Field::__field1),
                        b"ary1" => _serde::__private229::Ok(__Field::__field2),
                        b"ary3" => _serde::__private229::Ok(__Field::__field3),
                        b"inner" => _serde::__private229::Ok(__Field::__field4),
                        _ => {
                            let __value = &_serde::__private229::from_utf8_lossy(__value);
                            _serde::__private229::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            ))
                        }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private229::PhantomData<DemoEnum>,
                lifetime: _serde::__private229::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DemoEnum;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(__formatter, "enum DemoEnum")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private229::Ok((__Field::__field0, __variant)) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private229::Ok(DemoEnum::A)
                        }
                        _serde::__private229::Ok((__Field::__field1, __variant)) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private229::Ok(DemoEnum::Ary2)
                        }
                        _serde::__private229::Ok((__Field::__field2, __variant)) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private229::Ok(DemoEnum::Ary1)
                        }
                        _serde::__private229::Ok((__Field::__field3, __variant)) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private229::Ok(DemoEnum::Ary3)
                        }
                        _serde::__private229::Ok((__Field::__field4, __variant)) => {
                            _serde::de::VariantAccess::unit_variant(__variant)?;
                            _serde::__private229::Ok(DemoEnum::Inner)
                        }
                        _serde::__private229::Err(__err) => _serde::__private229::Err(__err),
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["a", "ary2", "ary1", "ary3", "inner"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "DemoEnum",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<DemoEnum>,
                    lifetime: _serde::__private229::PhantomData,
                },
            )
        }
    }
};

#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    _serde::__require_serde_not_serde_core!();
    #[automatically_derived]
    impl _serde::Serialize for DemoValueEnum {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private229::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                DemoValueEnum::U8(ref __field0) => {
                    let mut __struct =
                        _serde::Serializer::serialize_struct(__serializer, "DemoValueEnum", 2)?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "type",
                        &_serde::__private229::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "DemoValueEnum",
                            variant_index: 0u32,
                            variant_name: "u8",
                        },
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(&mut __struct, "data", __field0)?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
                DemoValueEnum::None => {
                    let mut __struct =
                        _serde::Serializer::serialize_struct(__serializer, "DemoValueEnum", 1)?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __struct,
                        "type",
                        &_serde::__private229::ser::AdjacentlyTaggedEnumVariant {
                            enum_name: "DemoValueEnum",
                            variant_index: 1u32,
                            variant_name: "none",
                        },
                    )?;
                    _serde::ser::SerializeStruct::end(__struct)
                }
            }
        }
    }
};

#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    _serde::__require_serde_not_serde_core!();
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for DemoValueEnum {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(__formatter, "variant identifier")
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private229::Ok(__Field::__field0),
                        1u64 => _serde::__private229::Ok(__Field::__field1),
                        _ => _serde::__private229::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"variant index 0 <= i < 2",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "u8" => _serde::__private229::Ok(__Field::__field0),
                        "none" => _serde::__private229::Ok(__Field::__field1),
                        _ => _serde::__private229::Err(_serde::de::Error::unknown_variant(
                            __value, VARIANTS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"u8" => _serde::__private229::Ok(__Field::__field0),
                        b"none" => _serde::__private229::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private229::from_utf8_lossy(__value);
                            _serde::__private229::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            ))
                        }
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &["u8", "none"];
            #[doc(hidden)]
            struct __Seed<'de> {
                variant: __Field,
                marker: _serde::__private229::PhantomData<DemoValueEnum>,
                lifetime: _serde::__private229::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::DeserializeSeed<'de> for __Seed<'de> {
                type Value = DemoValueEnum;
                fn deserialize<__D>(
                    self,
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self::Value, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    match self.variant {
                        __Field::__field0 => _serde::__private229::Result::map(
                            <u8 as _serde::Deserialize>::deserialize(__deserializer),
                            DemoValueEnum::U8,
                        ),
                        __Field::__field1 => match _serde::Deserializer::deserialize_any(
                            __deserializer,
                            _serde::__private229::de::UntaggedUnitVisitor::new(
                                "DemoValueEnum",
                                "None",
                            ),
                        ) {
                            _serde::__private229::Ok(()) => {
                                _serde::__private229::Ok(DemoValueEnum::None)
                            }
                            _serde::__private229::Err(__err) => _serde::__private229::Err(__err),
                        },
                    }
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private229::PhantomData<DemoValueEnum>,
                lifetime: _serde::__private229::PhantomData<&'de ()>,
            }
            //noinspection RsSortImplTraitMembers
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DemoValueEnum;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        __formatter,
                        "adjacently tagged enum DemoValueEnum",
                    )
                }
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    match {
                        let mut __rk: _serde::__private229::Option<
                            _serde::__private229::de::TagOrContentField,
                        > = _serde::__private229::None;
                        while let _serde::__private229::Some(__k) =
                            _serde::de::MapAccess::next_key_seed(
                                &mut __map,
                                _serde::__private229::de::TagContentOtherFieldVisitor {
                                    tag: "type",
                                    content: "data",
                                },
                            )?
                        {
                            match __k {
                                _serde::__private229::de::TagContentOtherField::Other => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                    continue;
                                }
                                _serde::__private229::de::TagContentOtherField::Tag => {
                                    __rk = _serde::__private229::Some(
                                        _serde::__private229::de::TagOrContentField::Tag,
                                    );
                                    break;
                                }
                                _serde::__private229::de::TagContentOtherField::Content => {
                                    __rk = _serde::__private229::Some(
                                        _serde::__private229::de::TagOrContentField::Content,
                                    );
                                    break;
                                }
                            }
                        }
                        __rk
                    } {
                        _serde::__private229::Some(
                            _serde::__private229::de::TagOrContentField::Tag,
                        ) => {
                            let __field = _serde::de::MapAccess::next_value_seed(
                                &mut __map,
                                _serde::__private229::de::AdjacentlyTaggedEnumVariantSeed::<
                                    __Field,
                                > {
                                    enum_name: "DemoValueEnum",
                                    variants: VARIANTS,
                                    fields_enum: _serde::__private229::PhantomData,
                                },
                            )?;
                            match {
                                let mut __rk: _serde::__private229::Option<
                                    _serde::__private229::de::TagOrContentField,
                                > = _serde::__private229::None;
                                while let _serde::__private229::Some(__k) =
                                    _serde::de::MapAccess::next_key_seed(
                                        &mut __map,
                                        _serde::__private229::de::TagContentOtherFieldVisitor {
                                            tag: "type",
                                            content: "data",
                                        },
                                    )?
                                {
                                    match __k {
                                        _serde::__private229::de::TagContentOtherField::Other => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                            continue;
                                        }
                                        _serde::__private229::de::TagContentOtherField::Tag => {
                                            __rk = _serde::__private229::Some(
                                                _serde::__private229::de::TagOrContentField::Tag,
                                            );
                                            break;
                                        }
                                        _serde::__private229::de::TagContentOtherField::Content => {
                                            __rk = _serde::__private229::Some(_serde::__private229::de::TagOrContentField::Content);
                                            break;
                                        }
                                    }
                                }
                                __rk
                            } {
                                _serde::__private229::Some(
                                    _serde::__private229::de::TagOrContentField::Tag,
                                ) => _serde::__private229::Err(
                                    <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                ),
                                _serde::__private229::Some(
                                    _serde::__private229::de::TagOrContentField::Content,
                                ) => {
                                    let __ret = _serde::de::MapAccess::next_value_seed(
                                        &mut __map,
                                        __Seed {
                                            variant: __field,
                                            marker: _serde::__private229::PhantomData,
                                            lifetime: _serde::__private229::PhantomData,
                                        },
                                    )?;
                                    match {
                                        let mut __rk: _serde::__private229::Option<
                                            _serde::__private229::de::TagOrContentField,
                                        > = _serde::__private229::None;
                                        while let _serde::__private229::Some(__k) = _serde::de::MapAccess::next_key_seed(&mut __map, _serde::__private229::de::TagContentOtherFieldVisitor { tag: "type", content: "data" })? {
                                                match __k {
                                                    _serde::__private229::de::TagContentOtherField::Other => {
                                                        let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                                                        continue;
                                                    }
                                                    _serde::__private229::de::TagContentOtherField::Tag => {
                                                        __rk = _serde::__private229::Some(_serde::__private229::de::TagOrContentField::Tag);
                                                        break;
                                                    }
                                                    _serde::__private229::de::TagContentOtherField::Content => {
                                                        __rk = _serde::__private229::Some(_serde::__private229::de::TagOrContentField::Content);
                                                        break;
                                                    }
                                                }
                                            }
                                        __rk
                                    } {
                                        _serde::__private229::Some(
                                            _serde::__private229::de::TagOrContentField::Tag,
                                        ) => _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "type",
                                            ),
                                        ),
                                        _serde::__private229::Some(
                                            _serde::__private229::de::TagOrContentField::Content,
                                        ) => _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "data",
                                            ),
                                        ),
                                        _serde::__private229::None => {
                                            _serde::__private229::Ok(__ret)
                                        }
                                    }
                                }
                                _serde::__private229::None => match __field {
                                    __Field::__field0 => {
                                        _serde::__private229::de::missing_field("data")
                                            .map(DemoValueEnum::U8)
                                    }
                                    __Field::__field1 => {
                                        _serde::__private229::Ok(DemoValueEnum::None)
                                    }
                                },
                            }
                        }
                        _serde::__private229::Some(
                            _serde::__private229::de::TagOrContentField::Content,
                        ) => {
                            let __content = _serde::de::MapAccess::next_value_seed(
                                &mut __map,
                                _serde::__private229::de::ContentVisitor::new(),
                            )?;
                            match {
                                let mut __rk: _serde::__private229::Option<
                                    _serde::__private229::de::TagOrContentField,
                                > = _serde::__private229::None;
                                while let _serde::__private229::Some(__k) =
                                    _serde::de::MapAccess::next_key_seed(
                                        &mut __map,
                                        _serde::__private229::de::TagContentOtherFieldVisitor {
                                            tag: "type",
                                            content: "data",
                                        },
                                    )?
                                {
                                    match __k {
                                        _serde::__private229::de::TagContentOtherField::Other => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                            continue;
                                        }
                                        _serde::__private229::de::TagContentOtherField::Tag => {
                                            __rk = _serde::__private229::Some(
                                                _serde::__private229::de::TagOrContentField::Tag,
                                            );
                                            break;
                                        }
                                        _serde::__private229::de::TagContentOtherField::Content => {
                                            __rk = _serde::__private229::Some(_serde::__private229::de::TagOrContentField::Content);
                                            break;
                                        }
                                    }
                                }
                                __rk
                            } {
                                _serde::__private229::Some(
                                    _serde::__private229::de::TagOrContentField::Tag,
                                ) => {
                                    let __seed = __Seed { variant: _serde::de::MapAccess::next_value_seed(&mut __map, _serde::__private229::de::AdjacentlyTaggedEnumVariantSeed::<__Field> { enum_name: "DemoValueEnum", variants: VARIANTS, fields_enum: _serde::__private229::PhantomData })?, marker: _serde::__private229::PhantomData, lifetime: _serde::__private229::PhantomData };
                                    let __deserializer =
                                            _serde::__private229::de::ContentDeserializer::<
                                                __A::Error,
                                            >::new(
                                                __content
                                            );
                                    let __ret = _serde::de::DeserializeSeed::deserialize(
                                        __seed,
                                        __deserializer,
                                    )?;
                                    match {
                                        let mut __rk: _serde::__private229::Option<
                                            _serde::__private229::de::TagOrContentField,
                                        > = _serde::__private229::None;
                                        while let _serde::__private229::Some(__k) = _serde::de::MapAccess::next_key_seed(&mut __map, _serde::__private229::de::TagContentOtherFieldVisitor { tag: "type", content: "data" })? {
                                                match __k {
                                                    _serde::__private229::de::TagContentOtherField::Other => {
                                                        let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                                                        continue;
                                                    }
                                                    _serde::__private229::de::TagContentOtherField::Tag => {
                                                        __rk = _serde::__private229::Some(_serde::__private229::de::TagOrContentField::Tag);
                                                        break;
                                                    }
                                                    _serde::__private229::de::TagContentOtherField::Content => {
                                                        __rk = _serde::__private229::Some(_serde::__private229::de::TagOrContentField::Content);
                                                        break;
                                                    }
                                                }
                                            }
                                        __rk
                                    } {
                                        _serde::__private229::Some(
                                            _serde::__private229::de::TagOrContentField::Tag,
                                        ) => _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "type",
                                            ),
                                        ),
                                        _serde::__private229::Some(
                                            _serde::__private229::de::TagOrContentField::Content,
                                        ) => _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "data",
                                            ),
                                        ),
                                        _serde::__private229::None => {
                                            _serde::__private229::Ok(__ret)
                                        }
                                    }
                                }
                                _serde::__private229::Some(
                                    _serde::__private229::de::TagOrContentField::Content,
                                ) => _serde::__private229::Err(
                                    <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                ),
                                _serde::__private229::None => _serde::__private229::Err(
                                    <__A::Error as _serde::de::Error>::missing_field("type"),
                                ),
                            }
                        }
                        _serde::__private229::None => _serde::__private229::Err(
                            <__A::Error as _serde::de::Error>::missing_field("type"),
                        ),
                    }
                }
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    match _serde::de::SeqAccess::next_element(&mut __seq) {
                        _serde::__private229::Ok(_serde::__private229::Some(__variant)) => {
                            match _serde::de::SeqAccess::next_element_seed(
                                &mut __seq,
                                __Seed {
                                    variant: __variant,
                                    marker: _serde::__private229::PhantomData,
                                    lifetime: _serde::__private229::PhantomData,
                                },
                            ) {
                                _serde::__private229::Ok(_serde::__private229::Some(__ret)) => {
                                    _serde::__private229::Ok(__ret)
                                }
                                _serde::__private229::Ok(_serde::__private229::None) => {
                                    _serde::__private229::Err(_serde::de::Error::invalid_length(
                                        1, &self,
                                    ))
                                }
                                _serde::__private229::Err(__err) => {
                                    _serde::__private229::Err(__err)
                                }
                            }
                        }
                        _serde::__private229::Ok(_serde::__private229::None) => {
                            _serde::__private229::Err(_serde::de::Error::invalid_length(0, &self))
                        }
                        _serde::__private229::Err(__err) => _serde::__private229::Err(__err),
                    }
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["type", "data"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "DemoValueEnum",
                FIELDS,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<DemoValueEnum>,
                    lifetime: _serde::__private229::PhantomData,
                },
            )
        }
    }
};

#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    _serde::__require_serde_not_serde_core!();
    #[automatically_derived]
    impl _serde::Serialize for DemoInnerStruct {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private229::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "DemoInnerStruct",
                false as usize + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "enum_k",
                &self.enum_k,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};

#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    _serde::__require_serde_not_serde_core!();
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for DemoInnerStruct {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private229::Ok(__Field::__field0),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "enum_k" => _serde::__private229::Ok(__Field::__field0),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"enum_k" => _serde::__private229::Ok(__Field::__field0),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private229::PhantomData<DemoInnerStruct>,
                lifetime: _serde::__private229::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DemoInnerStruct;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        __formatter,
                        "struct DemoInnerStruct",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match _serde::de::SeqAccess::next_element::<DemoEnum>(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct DemoInnerStruct with 1 element",
                                    ),
                                )
                            }
                        };
                    _serde::__private229::Ok(DemoInnerStruct { enum_k: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private229::Option<DemoEnum> =
                        _serde::__private229::None;
                    while let _serde::__private229::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private229::Option::is_some(&__field0) {
                                    return _serde::__private229::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enum_k",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private229::Some(
                                    _serde::de::MapAccess::next_value::<DemoEnum>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private229::Some(__field0) => __field0,
                        _serde::__private229::None => {
                            _serde::__private229::de::missing_field("enum_k")?
                        }
                    };
                    _serde::__private229::Ok(DemoInnerStruct { enum_k: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["enum_k"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "DemoInnerStruct",
                FIELDS,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<DemoInnerStruct>,
                    lifetime: _serde::__private229::PhantomData,
                },
            )
        }
    }
};

#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    _serde::__require_serde_not_serde_core!();
    #[automatically_derived]
    impl _serde::Serialize for DemoStruct {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private229::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "DemoStruct",
                false as usize + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "enum_k",
                &self.enum_k,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "inner_struct",
                &self.inner_struct,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "array",
                &self.array,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "enum_with_value",
                &self.enum_with_value,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};

#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    _serde::__require_serde_not_serde_core!();
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for DemoStruct {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private229::Ok(__Field::__field0),
                        1u64 => _serde::__private229::Ok(__Field::__field1),
                        2u64 => _serde::__private229::Ok(__Field::__field2),
                        3u64 => _serde::__private229::Ok(__Field::__field3),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "enum_k" => _serde::__private229::Ok(__Field::__field0),
                        "inner_struct" => _serde::__private229::Ok(__Field::__field1),
                        "array" => _serde::__private229::Ok(__Field::__field2),
                        "enum_with_value" => _serde::__private229::Ok(__Field::__field3),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"enum_k" => _serde::__private229::Ok(__Field::__field0),
                        b"inner_struct" => _serde::__private229::Ok(__Field::__field1),
                        b"array" => _serde::__private229::Ok(__Field::__field2),
                        b"enum_with_value" => _serde::__private229::Ok(__Field::__field3),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private229::PhantomData<DemoStruct>,
                lifetime: _serde::__private229::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DemoStruct;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter<'_>,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(__formatter, "struct DemoStruct")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match _serde::de::SeqAccess::next_element::<DemoEnum>(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct DemoStruct with 4 elements",
                                    ),
                                )
                            }
                        };
                    let __field1 =
                        match _serde::de::SeqAccess::next_element::<DemoInnerStruct>(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct DemoStruct with 4 elements",
                                    ),
                                )
                            }
                        };
                    let __field2 =
                        match _serde::de::SeqAccess::next_element::<Vec<DemoEnum>>(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct DemoStruct with 4 elements",
                                    ),
                                )
                            }
                        };
                    let __field3 = match _serde::de::SeqAccess::next_element::<Vec<DemoValueEnum>>(
                        &mut __seq,
                    )? {
                        _serde::__private229::Some(__value) => __value,
                        _serde::__private229::None => {
                            return _serde::__private229::Err(_serde::de::Error::invalid_length(
                                3usize,
                                &"struct DemoStruct with 4 elements",
                            ))
                        }
                    };
                    _serde::__private229::Ok(DemoStruct {
                        enum_k: __field0,
                        inner_struct: __field1,
                        array: __field2,
                        enum_with_value: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private229::Option<DemoEnum> =
                        _serde::__private229::None;
                    let mut __field1: _serde::__private229::Option<DemoInnerStruct> =
                        _serde::__private229::None;
                    let mut __field2: _serde::__private229::Option<Vec<DemoEnum>> =
                        _serde::__private229::None;
                    let mut __field3: _serde::__private229::Option<Vec<DemoValueEnum>> =
                        _serde::__private229::None;
                    while let _serde::__private229::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private229::Option::is_some(&__field0) {
                                    return _serde::__private229::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enum_k",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private229::Some(
                                    _serde::de::MapAccess::next_value::<DemoEnum>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private229::Option::is_some(&__field1) {
                                    return _serde::__private229::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inner_struct",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private229::Some(
                                    _serde::de::MapAccess::next_value::<DemoInnerStruct>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private229::Option::is_some(&__field2) {
                                    return _serde::__private229::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("array"),
                                    );
                                }
                                __field2 = _serde::__private229::Some(
                                    _serde::de::MapAccess::next_value::<Vec<DemoEnum>>(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private229::Option::is_some(&__field3) {
                                    return _serde::__private229::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "enum_with_value",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private229::Some(
                                    _serde::de::MapAccess::next_value::<Vec<DemoValueEnum>>(
                                        &mut __map,
                                    )?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private229::Some(__field0) => __field0,
                        _serde::__private229::None => {
                            _serde::__private229::de::missing_field("enum_k")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private229::Some(__field1) => __field1,
                        _serde::__private229::None => {
                            _serde::__private229::de::missing_field("inner_struct")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private229::Some(__field2) => __field2,
                        _serde::__private229::None => {
                            _serde::__private229::de::missing_field("array")?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private229::Some(__field3) => __field3,
                        _serde::__private229::None => {
                            _serde::__private229::de::missing_field("enum_with_value")?
                        }
                    };
                    _serde::__private229::Ok(DemoStruct {
                        enum_k: __field0,
                        inner_struct: __field1,
                        array: __field2,
                        enum_with_value: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] =
                &["enum_k", "inner_struct", "array", "enum_with_value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "DemoStruct",
                FIELDS,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<DemoStruct>,
                    lifetime: _serde::__private229::PhantomData,
                },
            )
        }
    }
};
