use serde::{de, Deserialize, Serialize};

use crate::{gc::*, obj::Obj, Align, Int, List, Set, Size};

// Align is serialized as a number
impl Serialize for Align {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.bytes().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Align {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Align::from_bytes(Int::deserialize(deserializer)?)
            .ok_or_else(|| de::Error::custom("invalid Align"))
    }
}

// Size is serialized as a number
impl Serialize for Size {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.bytes().serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Size::from_bytes(Int::deserialize(deserializer)?)
            .ok_or_else(|| de::Error::custom("invalid Size"))
    }
}

// Our newtype around string is serialized as a regular String.
impl Serialize for crate::String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for crate::String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(crate::String(GcCow::deserialize(deserializer)?))
    }
}

// Names are serialized as the integers that they are, again skipping the newtype wrapper.
impl Serialize for crate::Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for crate::Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Deserialize::deserialize(deserializer)?))
    }
}

// Lists also skip the newtype wrapper.
impl<T: Serialize + Obj> Serialize for List<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de, T: Obj + Deserialize<'de>> Deserialize<'de> for List<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(List(GcCow::deserialize(deserializer)?))
    }
}

// Sets also skip the newtype wrapper.
impl<T: Serialize + Obj> Serialize for Set<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}
impl<'de, T: Obj + Deserialize<'de>> Deserialize<'de> for Set<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Set(GcCow::deserialize(deserializer)?))
    }
}
