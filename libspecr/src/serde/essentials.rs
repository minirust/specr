use std::str::FromStr;

use serde::{
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Serialize,
};
use serde_json::Number;

use crate::{gc::*, obj::Obj, Int, Map};

use std::{fmt, marker::PhantomData};

use im::HashMap;
use num_bigint::BigInt;
use serde::de::{self, Unexpected};

// GcCow are an implementation detail of the Rust library; they do not show up in specr source code
// and similarly should not show up in the JSON.
// We thus treat them transparently when (de)serializing.
impl<T: Serialize + GcCompat> Serialize for GcCow<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        GcCow::call_ref_unchecked(*self, |x| x.serialize(serializer))
    }
}
impl<'de, T: Deserialize<'de> + GcCompat> Deserialize<'de> for GcCow<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let x = T::deserialize(deserializer)?;
        Ok(GcCow::new(x))
    }
}

// We want our BigInts to show up as integers in the JSON.
// For this, we use the `"arbitrary_precision"` feature of `serde_json`, and this requires using
// `serde_json::Number`, which has some internal magic to store arbitrary-precision integers and
// convert them from/to JSON numbers properly.
impl Serialize for Int {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let jv = Number::from_str(&format!("{self}")).unwrap();
        jv.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Int {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let jv = Number::deserialize(deserializer)?;
        Ok(Int::wrap(jv.as_str().parse::<BigInt>().map_err(|_| {
            de::Error::invalid_value(
                Unexpected::Other(format!("non-integer number {}", jv.as_str()).leak()),
                &"an integer",
            )
        })?))
    }
}

// We use this somewhat botched serialization scheme for maps so that we never generate arbitrary keys.
// Having non-string keys does not work in (serde-)JSON and JSON is the whole point of this.
impl<K: Serialize + Obj, V: Serialize + Obj> Serialize for Map<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut eleser = serializer.serialize_seq(Some(self.len_as_usize()))?;
        for kv in self.iter() {
            // This makes each element show up as JSON list with two entries, i.e. a pair.
            eleser.serialize_element(&kv)?;
        }
        eleser.end()
    }
}
impl<'de, K: Obj + Deserialize<'de>, V: Obj + Deserialize<'de>> Deserialize<'de> for Map<K, V> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ElmVisitor<K, V>(PhantomData<K>, PhantomData<V>);

        impl<'de, K: Obj + Deserialize<'de>, V: Obj + Deserialize<'de>> Visitor<'de> for ElmVisitor<K, V> {
            type Value = Map<K, V>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a libspecr::Map<_, _>")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut map = HashMap::new();

                while let Some((k, v)) = seq.next_element::<(K, V)>()? {
                    let None = map.insert(k, v) else {
                        return Err(de::Error::duplicate_field(format!("{:?}", k).leak()));
                    };
                }

                Ok(Map(GcCow::new(map)))
            }
        }

        deserializer.deserialize_seq(ElmVisitor::<K, V>(PhantomData, PhantomData))
    }
}
