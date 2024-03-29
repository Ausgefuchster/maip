use std::{fmt, marker::PhantomData};

use serde::{de, Deserialize, Deserializer};

pub fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec(PhantomData<Vec<String>>);

    impl<'de> de::Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec(PhantomData))
}

pub fn serialize_string_or_vec<S>(value: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if value.len() == 1 {
        serializer.serialize_str(&value[0])
    } else {
        serializer.collect_seq(value)
    }
}
