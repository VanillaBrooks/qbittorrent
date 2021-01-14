use super::data::Hash;
use serde::de::{self, Error, Unexpected};
use serde::Deserializer;
use std::fmt;

pub trait QueryConcat {
    fn query_concat(&self, sep: char) -> String;
}

impl QueryConcat for Hash {
    fn query_concat(&self, _: char) -> String {
        return self.hash.to_string();
    }
}

impl QueryConcat for &str {
    fn query_concat(&self, _: char) -> String {
        return self.to_string();
    }
}

impl QueryConcat for &[String] {
    fn query_concat(&self, sep: char) -> String {
        let mut hash_url = self
            .iter()
            .map(|x| {
                let mut cln = x.clone();
                cln.push(sep);
                cln
            })
            .collect::<String>();

        // remove the final | from the string
        hash_url.remove(hash_url.len() - 1);
        hash_url
    }
}

impl QueryConcat for &[Hash] {
    fn query_concat(&self, sep: char) -> String {
        let mut hash_url = self
            .iter()
            .map(|x| {
                let mut cln = x.hash.clone();
                cln.push(sep);
                cln
            })
            .collect::<String>();

        // remove the final | from the string
        hash_url.remove(hash_url.len() - 1);
        hash_url
    }
}

impl QueryConcat for [&Hash] {
    fn query_concat(&self, sep: char) -> String {
        let mut hash_url = self
            .iter()
            .map(|x| {
                let mut cln = x.hash.clone();
                cln.push(sep);
                cln
            })
            .collect::<String>();

        // remove the final | from the string
        hash_url.remove(hash_url.len() - 1);
        hash_url
    }
}

struct DeserializeU32OrEmptyStringVisitor;

impl<'de> de::Visitor<'de> for DeserializeU32OrEmptyStringVisitor {
    type Value = u32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer or a string")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v as u32)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v == "" {
            Ok(0)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }
}

pub fn deserialize_u32_or_empty_string<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeU32OrEmptyStringVisitor)
}
