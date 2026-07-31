use std::error::Error;
use std::fmt::{Display, Formatter};

pub use de::JsonValueSource;
pub use ser::JsonValueSink;
use serde::{de::Error as DeError, ser::Error as SerError};

#[derive(Debug)]
pub struct SerDeIoError(pub std::io::Error);

impl SerDeIoError {
    pub fn new<E>(kind: std::io::ErrorKind, msg: E) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync>>,
    {
        Self(std::io::Error::new(kind, msg.into()))
    }

    pub fn custom<E>(msg: E) -> Self
    where
        E: Into<Box<dyn Error + Send + Sync>>,
    {
        Self::new(std::io::ErrorKind::Other, msg)
    }
}

impl From<std::io::Error> for SerDeIoError {
    #[inline]
    fn from(e: std::io::Error) -> Self {
        Self(e)
    }
}

impl Display for SerDeIoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SerIoError({})", self.0)
    }
}

impl Error for SerDeIoError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl SerError for SerDeIoError {
    #[inline]
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::custom(msg.to_string())
    }
}

impl DeError for SerDeIoError {
    #[inline]
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::custom(msg.to_string())
    }
}

mod ser {
    use super::SerDeIoError;
    use crate::{JsonEvent, WriterJsonSerializer};
    use serde::ser::{
        SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
        SerializeTupleStruct, SerializeTupleVariant,
    };
    use serde::{Serialize, Serializer};
    use std::borrow::Cow;
    use std::io::Write;

    pub struct JsonValueSink<'a, W: Write> {
        writer: &'a mut WriterJsonSerializer<W>,
    }

    impl<'a, W: Write> JsonValueSink<'a, W> {
        pub fn new(writer: &'a mut WriterJsonSerializer<W>) -> Self {
            Self { writer }
        }
    }

    impl<'a, W: Write> Serializer for JsonValueSink<'a, W> {
        type Ok = &'a mut WriterJsonSerializer<W>;
        type Error = SerDeIoError;
        type SerializeSeq = Self;
        type SerializeTuple = Self;
        type SerializeTupleStruct = Self;
        type SerializeTupleVariant = Self;
        type SerializeMap = Self;
        type SerializeStruct = Self;
        type SerializeStructVariant = Self;

        fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
            self.writer.serialize_event(JsonEvent::Boolean(v))?;
            Ok(self.writer)
        }

        fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::Number(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::String(Cow::Owned(v.to_string())))?;
            Ok(self.writer)
        }

        fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
            self.writer
                .serialize_event(JsonEvent::String(Cow::Borrowed(v)))?;
            Ok(self.writer)
        }

        fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartArray)?;
            self.writer.serialize_event(JsonEvent::EndArray)?;
            for b in v {
                self.writer
                    .serialize_event(JsonEvent::Number(Cow::Owned(b.to_string())))?;
            }
            Ok(self.writer)
        }

        fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
            self.writer.serialize_event(JsonEvent::Null)?;
            Ok(self.writer)
        }

        fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }

        fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
            self.writer.serialize_event(JsonEvent::Null)?;
            Ok(self.writer)
        }

        fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
            self.serialize_unit()
        }

        fn serialize_unit_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
        ) -> Result<Self::Ok, Self::Error> {
            self.serialize_str(variant)
        }

        fn serialize_newtype_struct<T>(
            self,
            _name: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(self)
        }

        fn serialize_newtype_variant<T>(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Self::Ok, Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let writer = self.writer;
            writer.serialize_event(JsonEvent::StartObject)?;
            writer.serialize_event(JsonEvent::ObjectKey(Cow::Borrowed(variant)))?;
            let writer = value.serialize(Self::new(writer))?;
            writer.serialize_event(JsonEvent::EndObject)?;
            Ok(writer)
        }

        fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartArray)?;
            Ok(self)
        }

        fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartArray)?;
            Ok(self)
        }

        fn serialize_tuple_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleStruct, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartArray)?;
            Ok(self)
        }

        fn serialize_tuple_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeTupleVariant, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartObject)?;
            self.writer
                .serialize_event(JsonEvent::ObjectKey(Cow::Borrowed(variant)))?;
            self.writer.serialize_event(JsonEvent::StartArray)?;
            Ok(self)
        }

        fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartObject)?;
            Ok(self)
        }

        fn serialize_struct(
            self,
            _name: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStruct, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartObject)?;
            Ok(self)
        }

        fn serialize_struct_variant(
            self,
            _name: &'static str,
            _variant_index: u32,
            variant: &'static str,
            _len: usize,
        ) -> Result<Self::SerializeStructVariant, Self::Error> {
            self.writer.serialize_event(JsonEvent::StartObject)?;
            self.writer
                .serialize_event(JsonEvent::ObjectKey(Cow::Borrowed(variant)))?;
            self.writer.serialize_event(JsonEvent::StartObject)?;
            Ok(self)
        }
    }

    macro_rules! imp_ser {
        ($t:ty, $name:ident, $($end:expr),+) => {
            impl<'a, W: Write> $t for JsonValueSink<'a, W> {
                type Ok = &'a mut WriterJsonSerializer<W>;
                type Error = SerDeIoError;

                fn $name<T>(&mut self, value: &T) -> Result<(), Self::Error>
                where
                    T: ?Sized + Serialize,
                {
                    value.serialize(JsonValueSink::new(self.writer))?;
                    Ok(())
                }

                fn end(self) -> Result<Self::Ok, Self::Error> {
                    $(self.writer.serialize_event($end)?;)+
                    Ok(self.writer)
                }
            }
        };
    }

    imp_ser!(SerializeSeq, serialize_element, JsonEvent::EndArray);
    imp_ser!(SerializeTuple, serialize_element, JsonEvent::EndArray);
    imp_ser!(SerializeTupleStruct, serialize_field, JsonEvent::EndArray);
    imp_ser!(
        SerializeTupleVariant,
        serialize_field,
        JsonEvent::EndArray,
        JsonEvent::EndObject
    );

    impl<'a, W: Write> SerializeMap for JsonValueSink<'a, W> {
        type Ok = &'a mut WriterJsonSerializer<W>;
        type Error = SerDeIoError;

        fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            let key = serde_json::to_string(key).map_err(|err| {
                SerDeIoError::custom(format!("convert key to value error: {:#?}", err))
            })?;
            self.writer
                .serialize_event(JsonEvent::ObjectKey(Cow::Owned(key)))?;
            Ok(())
        }

        fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where
            T: ?Sized + Serialize,
        {
            value.serialize(JsonValueSink::new(self.writer))?;
            Ok(())
        }

        fn end(self) -> Result<Self::Ok, Self::Error> {
            self.writer.serialize_event(JsonEvent::EndObject)?;
            Ok(self.writer)
        }
    }

    macro_rules! imp_ser_kv {
        ($t:ty, $($end:expr),+) => {
            impl<'a, W: Write> $t for JsonValueSink<'a, W> {
                type Ok = &'a mut WriterJsonSerializer<W>;
                type Error = SerDeIoError;

                fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
                where
                    T: ?Sized + Serialize,
                {
                    self.writer
                        .serialize_event(JsonEvent::ObjectKey(Cow::Borrowed(key)))?;
                    value.serialize(JsonValueSink::new(self.writer))?;
                    Ok(())
                }

                fn end(self) -> Result<Self::Ok, Self::Error> {
                    $(self.writer.serialize_event($end)?;)+
                    Ok(self.writer)
                }
            }
        };
    }

    imp_ser_kv!(SerializeStruct, JsonEvent::EndObject);
    imp_ser_kv!(
        SerializeStructVariant,
        JsonEvent::EndObject,
        JsonEvent::EndObject
    );
}

mod de {
    use crate::serde::SerDeIoError;
    use crate::{JsonEvent, ReaderJsonParser, Skipper};
    use serde::de::value::StrDeserializer;
    use serde::de::{DeserializeSeed, EnumAccess, MapAccess, SeqAccess, VariantAccess, Visitor};
    use serde::{Deserialize, Deserializer};
    use std::borrow::{Borrow, Cow};
    use std::fmt::{Display, Formatter};
    use std::io::Read;
    use std::marker::PhantomData;
    use std::num::IntErrorKind;
    use std::str::FromStr;

    pub struct JsonValueSource<'a, R: Read> {
        reader: &'a mut ReaderJsonParser<R>,
        peek: Option<JsonEvent<'static>>,
    }

    impl<'a, R: Read> JsonValueSource<'a, R> {
        pub fn new(reader: &'a mut ReaderJsonParser<R>) -> Self {
            Self { reader, peek: None }
        }

        fn with_peek(self, event: JsonEvent<'static>) -> Self {
            Self {
                reader: self.reader,
                peek: Some(event),
            }
        }

        fn with_opt_peek(self, event: Option<JsonEvent<'static>>) -> Self {
            if let Some(event) = event {
                self.with_peek(event)
            } else {
                self
            }
        }

        fn own_next_event(mut self) -> Result<JsonEvent<'a>, SerDeIoError> {
            if let Some(event) = self.peek.take() {
                return Ok(event);
            }

            self.reader
                .parse_next()
                .ok_or_else(|| SerDeIoError::new(std::io::ErrorKind::UnexpectedEof, "eof"))?
                .map_err(SerDeIoError::custom)
        }

        fn next_event(&mut self) -> Result<JsonEvent<'_>, SerDeIoError> {
            if let Some(event) = self.peek.take() {
                return Ok(event);
            }

            self.reader
                .parse_next()
                .ok_or_else(|| SerDeIoError::new(std::io::ErrorKind::UnexpectedEof, "eof"))?
                .map_err(SerDeIoError::custom)
        }

        fn next_event_static(&mut self) -> Result<JsonEvent<'static>, SerDeIoError> {
            if let Some(event) = self.peek.take() {
                return Ok(event);
            }

            self.next_event().map(event_to_static)
        }

        fn consume_byte_buf(&mut self, buf: &mut Vec<u8>) -> Result<(), SerDeIoError> {
            loop {
                match &self.next_event()? {
                    JsonEvent::EndArray => {
                        return Ok(());
                    }
                    JsonEvent::Number(text) => match text.parse::<u8>() {
                        Ok(v) => buf.push(v),
                        Err(err) => {
                            return Err(SerDeIoError::custom(format!(
                                "can't parse \"{}\" as byte while parsing bytes from array: {:#?}",
                                text, err
                            )))
                        }
                    },
                    event => {
                        return Err(SerDeIoError::custom(format!(
                            "unexpect {} while parsing bytes from array",
                            event_name(event)
                        )))
                    }
                }
            }
        }
    }

    macro_rules! primitive_visit {
        ($name:ident, $visit_fn:ident, $visit_ty:ty) => {
            fn $name<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                match &self.own_next_event()? {
                    JsonEvent::String(v) => match v.parse::<$visit_ty>() {
                        Ok(v) => visitor.$visit_fn(v),
                        Err(_) => Err(unexpect_type("String", &visitor)),
                    },
                    JsonEvent::Number(v) => {
                        visitor.$visit_fn(v.parse::<$visit_ty>().map_err(|err| {
                            SerDeIoError::new(std::io::ErrorKind::InvalidInput, err)
                        })?)
                    }
                    event => Err(unexpect_type(event_name(event), &visitor)),
                }
            }
        };
    }

    impl<'a, 'de: 'a, R: Read> Deserializer<'de> for JsonValueSource<'a, R> {
        type Error = SerDeIoError;

        fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::String(v) => visitor.visit_str(&v),
                JsonEvent::Number(v) => {
                    let num = v.parse::<JsonNumber>()?;
                    num.visit(visitor)
                }
                JsonEvent::Boolean(v) => visitor.visit_bool(v),
                JsonEvent::Null => visitor.visit_none(),
                JsonEvent::StartArray => visitor.visit_seq(self),
                JsonEvent::StartObject => visitor.visit_map(self),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        primitive_visit!(deserialize_bool, visit_bool, bool);
        primitive_visit!(deserialize_i8, visit_i8, i8);
        primitive_visit!(deserialize_u8, visit_u8, u8);
        primitive_visit!(deserialize_i16, visit_i16, i16);
        primitive_visit!(deserialize_u16, visit_u16, u16);
        primitive_visit!(deserialize_i32, visit_i32, i32);
        primitive_visit!(deserialize_u32, visit_u32, u32);
        primitive_visit!(deserialize_i64, visit_i64, i64);
        primitive_visit!(deserialize_u64, visit_u64, u64);
        primitive_visit!(deserialize_i128, visit_i128, i128);
        primitive_visit!(deserialize_u128, visit_u128, u128);
        primitive_visit!(deserialize_f32, visit_f32, f32);
        primitive_visit!(deserialize_f64, visit_f64, f64);

        fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.own_next_event()? {
                JsonEvent::String(v) => {
                    let mut chars = v.chars();
                    let ch = match chars.next() {
                        Some(v) => v,
                        None => return Err(unexpect_type("String(empty)", &visitor)),
                    };
                    if chars.next().is_some() {
                        return Err(unexpect_type("String(multi chars)", &visitor));
                    }
                    visitor.visit_char(ch)
                }
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.own_next_event()? {
                JsonEvent::String(v) => visitor.visit_str(v.as_ref()),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_string<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event()? {
                JsonEvent::String(v) => visitor.visit_string(v.into_owned()),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_bytes<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event()? {
                JsonEvent::String(v) => visitor.visit_bytes(v.as_bytes()),
                JsonEvent::StartArray => {
                    let mut buf = Vec::<u8>::new();
                    self.consume_byte_buf(&mut buf)?;
                    visitor.visit_bytes(&buf)
                }
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_byte_buf<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let mut buf = Vec::<u8>::new();
            match self.next_event()? {
                JsonEvent::String(v) => buf.extend_from_slice(v.as_bytes()),
                JsonEvent::StartArray => {
                    self.consume_byte_buf(&mut buf)?;
                }
                event => return Err(unexpect_type(event_name(&event), &visitor)),
            }
            visitor.visit_byte_buf(buf)
        }

        fn deserialize_option<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::Null => visitor.visit_none(),
                event => visitor.visit_some(self.with_peek(event)),
            }
        }

        fn deserialize_unit<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match &self.next_event()? {
                JsonEvent::Null => visitor.visit_none(),
                event => Err(unexpect_type(event_name(event), &visitor)),
            }
        }

        fn deserialize_unit_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.deserialize_unit(visitor)
        }

        fn deserialize_newtype_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_newtype_struct(self)
        }

        fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::StartArray => visitor.visit_seq(self),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_tuple<V>(mut self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::StartArray => visitor.visit_seq(self),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_tuple_struct<V>(
            mut self,
            _name: &'static str,
            _len: usize,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::StartArray => visitor.visit_seq(self),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event()? {
                JsonEvent::StartObject => visitor.visit_map(self),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_struct<V>(
            mut self,
            _name: &'static str,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::StartArray => visitor.visit_seq(self),
                JsonEvent::StartObject => visitor.visit_map(self),
                event => Err(unexpect_type(event_name(&event), &visitor)),
            }
        }

        fn deserialize_enum<V>(
            mut self,
            _name: &'static str,
            _variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let event = self.next_event_static()?;
            if matches!(event, JsonEvent::String(_) | JsonEvent::Number(_)) {
                visitor.visit_enum(JsonVariantAccess {
                    is_unit: true,
                    peek: Some(event),
                    source: self,
                })
            } else if matches!(event, JsonEvent::StartObject) {
                visitor.visit_enum(JsonVariantAccess {
                    is_unit: false,
                    peek: Some(event),
                    source: self,
                })
            } else {
                Err(unexpect_type(event_name(&event), &visitor))
            }
        }

        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            self.deserialize_str(visitor)
        }

        fn deserialize_ignored_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let mut skipper = Skipper::new();
            while skipper.skipping() {
                skipper
                    .on_event(&self.next_event()?)
                    .map_err(SerDeIoError::custom)?;
            }
            visitor.visit_unit()
        }
    }

    struct JsonVariantAccess<'a, R: Read> {
        is_unit: bool,
        peek: Option<JsonEvent<'static>>,
        source: JsonValueSource<'a, R>,
    }

    impl<'a, 'de: 'a, R: Read> EnumAccess<'de> for JsonVariantAccess<'a, R> {
        type Error = SerDeIoError;
        type Variant = Self;

        fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            let peek = self.peek.take();
            let val =
                seed.deserialize(JsonValueSource::new(self.source.reader).with_opt_peek(peek))?;
            Ok((val, self))
        }
    }

    impl<'a, 'de: 'a, R: Read> VariantAccess<'de> for JsonVariantAccess<'a, R> {
        type Error = SerDeIoError;

        fn unit_variant(self) -> Result<(), Self::Error> {
            if self.is_unit {
                return Ok(());
            }
            Deserialize::deserialize(self.source)
        }

        fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            if self.is_unit {
                Err(SerDeIoError::custom(
                    "UnitVariantAccess do **NOT** support newtype_variant_seed",
                ))
            } else {
                seed.deserialize(self.source)
            }
        }

        fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            if self.is_unit {
                Err(SerDeIoError::custom(
                    "UnitVariantAccess do **NOT** support tuple_variant",
                ))
            } else {
                Deserializer::deserialize_seq(self.source, visitor)
            }
        }

        fn struct_variant<V>(
            self,
            fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            if self.is_unit {
                Err(SerDeIoError::custom(
                    "UnitVariantAccess do **NOT** support struct_variant",
                ))
            } else {
                Deserializer::deserialize_struct(self.source, "", fields, visitor)
            }
        }
    }

    impl<'a, 'de: 'a, R: Read> MapAccess<'de> for JsonValueSource<'a, R> {
        type Error = SerDeIoError;

        fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            let Some(event) = self.reader.parse_next() else {
                return Err(SerDeIoError::new(std::io::ErrorKind::UnexpectedEof, "EOF"));
            };
            let event = event.map_err(SerDeIoError::custom)?;
            match event {
                JsonEvent::ObjectKey(k) => {
                    seed.deserialize(StrDeserializer::new(k.as_ref())).map(Some)
                }
                JsonEvent::EndObject => Ok(None),
                event => Err(SerDeIoError::custom(format!(
                    "unexpect event for map key: {:#?}",
                    event
                ))),
            }
        }

        fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            seed.deserialize(JsonValueSource::new(self.reader))
        }
    }

    impl<'a, 'de: 'a, R: Read> SeqAccess<'de> for JsonValueSource<'a, R> {
        type Error = SerDeIoError;

        fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            match self.next_event_static()? {
                JsonEvent::EndArray => Ok(None),
                event => seed
                    .deserialize(JsonValueSource::new(self.reader).with_peek(event))
                    .map(Some),
            }
        }
    }

    enum JsonNumber {
        I64(i64),
        U64(u64),
        F64(f64),
    }

    impl FromStr for JsonNumber {
        type Err = SerDeIoError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.trim();
            let err = if s.starts_with('-') {
                match s.parse::<i64>() {
                    Ok(v) => return Ok(Self::I64(v)),
                    Err(err) => err,
                }
            } else {
                match s.parse::<u64>() {
                    Ok(v) => return Ok(Self::U64(v)),
                    Err(err) => err,
                }
            };

            if matches!(
                err.kind(),
                IntErrorKind::NegOverflow | IntErrorKind::PosOverflow
            ) {
                match s.parse::<f64>() {
                    Ok(v) => Ok(Self::F64(v)),
                    Err(err) => Err(SerDeIoError::custom(format!("parse number error: {}", err))),
                }
            } else {
                Err(SerDeIoError::custom(format!("parse number error: {}", err)))
            }
        }
    }

    impl JsonNumber {
        fn visit<'de, V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, SerDeIoError> {
            match self {
                JsonNumber::I64(v) => visitor.visit_i64(v),
                JsonNumber::U64(v) => visitor.visit_u64(v),
                JsonNumber::F64(v) => visitor.visit_f64(v),
            }
        }
    }

    fn event_to_static<'a, V: Borrow<JsonEvent<'a>>>(event: V) -> JsonEvent<'static> {
        match event.borrow() {
            JsonEvent::String(v) => JsonEvent::String(Cow::Owned(v.to_string())),
            JsonEvent::Number(v) => JsonEvent::Number(Cow::Owned(v.to_string())),
            JsonEvent::Boolean(v) => JsonEvent::Boolean(*v),
            JsonEvent::Null => JsonEvent::Null,
            JsonEvent::StartArray => JsonEvent::StartArray,
            JsonEvent::EndArray => JsonEvent::EndArray,
            JsonEvent::StartObject => JsonEvent::StartObject,
            JsonEvent::EndObject => JsonEvent::EndObject,
            JsonEvent::ObjectKey(k) => JsonEvent::ObjectKey(Cow::Owned(k.to_string())),
        }
    }

    fn event_name(event: &JsonEvent<'_>) -> &'static str {
        match event {
            JsonEvent::String(_) => "String",
            JsonEvent::Number(_) => "Number",
            JsonEvent::Boolean(_) => "Boolean",
            JsonEvent::Null => "Null",
            JsonEvent::StartArray => "StartArray",
            JsonEvent::EndArray => "EndArray",
            JsonEvent::StartObject => "StartObject",
            JsonEvent::EndObject => "EndObject",
            JsonEvent::ObjectKey(_) => "ObjectKey",
        }
    }

    fn unexpect_type<'de, V: Visitor<'de>>(event_name: &str, visitor: &V) -> SerDeIoError {
        struct ErrMeta<'a, 'de, V: Visitor<'de>> {
            event_name: &'a str,
            visitor: &'a V,
            mark: PhantomData<&'de ()>,
        }

        impl<'a, 'de, V: Visitor<'de>> Display for ErrMeta<'a, 'de, V> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "UnexpectType[got={}, expect=", self.event_name)?;
                self.visitor.expecting(f)?;
                write!(f, "]")?;
                Ok(())
            }
        }

        SerDeIoError::new(
            std::io::ErrorKind::InvalidInput,
            ErrMeta {
                event_name,
                visitor,
                mark: PhantomData,
            }
            .to_string(),
        )
    }
}

