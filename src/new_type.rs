use std::{cmp::Ordering, marker::PhantomData};

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct StringType<T> {
    value: String,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> StringType<T> {
    pub fn new(value: String) -> Self {
        Self {
            value: value,
            _phantom: PhantomData,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn take(self) -> String {
        self.value
    }
}

impl<T> Clone for StringType<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for StringType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringType")
            .field("value", &self.value)
            .finish()
    }
}

impl<T> PartialEq for StringType<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for StringType<T> {}

impl<T> PartialOrd for StringType<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> Ord for StringType<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> std::hash::Hash for StringType<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<'a, T> From<&str> for StringType<T> {
    fn from(value: &str) -> Self {
        Self::new(value.to_owned())
    }
}

impl<T> From<String> for StringType<T> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl<T> Serialize for StringType<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}
impl<'de, T> Deserialize<'de> for StringType<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self::new)
    }
}

#[derive(Default)]
pub struct IntType<T> {
    value: i64,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> IntType<T> {
    pub fn new(value: i64) -> Self {
        Self {
            value: value,
            _phantom: PhantomData,
        }
    }

    pub fn value(self) -> i64 {
        self.value
    }

    pub fn pre_incr(&mut self) -> Self {
        self.value += 1;
        *self
    }

    pub fn pre_decr(&mut self) -> Self {
        self.value += 1;
        *self
    }

    pub fn post_incr(&mut self) -> Self {
        let result = *self;
        self.value += 1;
        result
    }

    pub fn post_decr(&mut self) -> Self {
        let result = *self;
        self.value += 1;
        result
    }
}

impl<T> Clone for IntType<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Copy for IntType<T> {}

impl<T> std::fmt::Debug for IntType<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IntType")
            .field("value", &self.value)
            .finish()
    }
}

impl<T> PartialEq for IntType<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for IntType<T> {}

impl<T> PartialOrd for IntType<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> Ord for IntType<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> std::hash::Hash for IntType<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<T> Serialize for IntType<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}
impl<'de, T> Deserialize<'de> for IntType<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        i64::deserialize(deserializer).map(Self::new)
    }
}
