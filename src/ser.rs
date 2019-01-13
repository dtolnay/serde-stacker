use ref_cast::RefCast;
use serde::ser;
use std::fmt::Display;

pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
    T: ser::Serialize,
{
    T::serialize(value, Serializer::new(serializer))
}

pub struct Serializer<S> {
    ser: S,
}

impl<S> Serializer<S> {
    pub fn new(serializer: S) -> Self {
        Serializer { ser: serializer }
    }
}

impl<S> ser::Serializer for Serializer<S>
where
    S: ser::Serializer,
{
    type Ok = S::Ok;
    type Error = S::Error;

    type SerializeSeq = SerializeSeq<S::SerializeSeq>;
    type SerializeTuple = SerializeTuple<S::SerializeTuple>;
    type SerializeTupleStruct = SerializeTupleStruct<S::SerializeTupleStruct>;
    type SerializeTupleVariant = SerializeTupleVariant<S::SerializeTupleVariant>;
    type SerializeMap = SerializeMap<S::SerializeMap>;
    type SerializeStruct = SerializeStruct<S::SerializeStruct>;
    type SerializeStructVariant = SerializeStructVariant<S::SerializeStructVariant>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_bool(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i8(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i16(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i32(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i64(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_i128(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u8(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u16(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u32(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u64(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_u128(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_f32(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_f64(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_char(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_str(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_none()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_some(Serialize::new(value))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_unit()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.ser.serialize_unit_struct(name)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.ser
            .serialize_unit_variant(name, variant_index, variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser
            .serialize_newtype_struct(name, Serialize::new(value))
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser
            .serialize_newtype_variant(name, variant_index, variant, Serialize::new(value))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.ser.serialize_seq(len).map(SerializeSeq::new)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.ser.serialize_tuple(len).map(SerializeTuple::new)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.ser
            .serialize_tuple_struct(name, len)
            .map(SerializeTupleStruct::new)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.ser
            .serialize_tuple_variant(name, variant_index, variant, len)
            .map(SerializeTupleVariant::new)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.ser.serialize_map(len).map(SerializeMap::new)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.ser
            .serialize_struct(name, len)
            .map(SerializeStruct::new)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.ser
            .serialize_struct_variant(name, variant_index, variant, len)
            .map(SerializeStructVariant::new)
    }

    fn collect_seq<I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        I: IntoIterator,
        I::Item: ser::Serialize,
    {
        let iter = iter.into_iter().map(|item| Serialize::new_sized(item));
        self.ser.collect_seq(iter)
    }

    fn collect_map<K, V, I>(self, iter: I) -> Result<Self::Ok, Self::Error>
    where
        K: ser::Serialize,
        V: ser::Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        let iter = iter
            .into_iter()
            .map(|(k, v)| (Serialize::new_sized(k), Serialize::new_sized(v)));
        self.ser.collect_map(iter)
    }

    fn collect_str<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Display,
    {
        self.ser.collect_str(value)
    }

    fn is_human_readable(&self) -> bool {
        self.ser.is_human_readable()
    }
}

pub struct SerializeSeq<S> {
    ser: S,
}

impl<S> SerializeSeq<S> {
    fn new(serialize_seq: S) -> Self {
        SerializeSeq { ser: serialize_seq }
    }
}

impl<S> ser::SerializeSeq for SerializeSeq<S>
where
    S: ser::SerializeSeq,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_element(Serialize::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

pub struct SerializeTuple<S> {
    ser: S,
}

impl<S> SerializeTuple<S> {
    fn new(serialize_tuple: S) -> Self {
        SerializeTuple {
            ser: serialize_tuple,
        }
    }
}

impl<S> ser::SerializeTuple for SerializeTuple<S>
where
    S: ser::SerializeTuple,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_element(Serialize::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

pub struct SerializeTupleStruct<S> {
    ser: S,
}

impl<S> SerializeTupleStruct<S> {
    fn new(serialize_tuple_struct: S) -> Self {
        SerializeTupleStruct {
            ser: serialize_tuple_struct,
        }
    }
}

impl<S> ser::SerializeTupleStruct for SerializeTupleStruct<S>
where
    S: ser::SerializeTupleStruct,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_field(Serialize::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

pub struct SerializeTupleVariant<S> {
    ser: S,
}

impl<S> SerializeTupleVariant<S> {
    fn new(serialize_tuple_variant: S) -> Self {
        SerializeTupleVariant {
            ser: serialize_tuple_variant,
        }
    }
}

impl<S> ser::SerializeTupleVariant for SerializeTupleVariant<S>
where
    S: ser::SerializeTupleVariant,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_field(Serialize::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }
}

pub struct SerializeMap<S> {
    ser: S,
}

impl<S> SerializeMap<S> {
    fn new(serialize_map: S) -> Self {
        SerializeMap { ser: serialize_map }
    }
}

impl<S> ser::SerializeMap for SerializeMap<S>
where
    S: ser::SerializeMap,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_key(Serialize::new(key))
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_value(Serialize::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + ser::Serialize,
        V: ?Sized + ser::Serialize,
    {
        self.ser
            .serialize_entry(Serialize::new(key), Serialize::new(value))
    }
}

pub struct SerializeStruct<S> {
    ser: S,
}

impl<S> SerializeStruct<S> {
    fn new(serialize_struct: S) -> Self {
        SerializeStruct {
            ser: serialize_struct,
        }
    }
}

impl<S> ser::SerializeStruct for SerializeStruct<S>
where
    S: ser::SerializeStruct,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_field(key, Serialize::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.ser.skip_field(key)
    }
}

pub struct SerializeStructVariant<S> {
    ser: S,
}

impl<S> SerializeStructVariant<S> {
    fn new(serialize_struct_variant: S) -> Self {
        SerializeStructVariant {
            ser: serialize_struct_variant,
        }
    }
}

impl<S> ser::SerializeStructVariant for SerializeStructVariant<S>
where
    S: ser::SerializeStructVariant,
{
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize,
    {
        self.ser.serialize_field(key, Serialize::new(value))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.ser.end()
    }

    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        self.ser.skip_field(key)
    }
}

#[derive(RefCast)]
#[repr(C)]
struct Serialize<T: ?Sized> {
    value: T,
}

impl<T: ?Sized> Serialize<T> {
    fn new(value: &T) -> &Self {
        Serialize::ref_cast(value)
    }
}

impl<T> Serialize<T> {
    fn new_sized(value: T) -> Self {
        Serialize { value }
    }
}

impl<T> ser::Serialize for Serialize<T>
where
    T: ?Sized + ser::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.value.serialize(Serializer::new(serializer))
    }
}
