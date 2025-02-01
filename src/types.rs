// src/types.rs
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct SecretString(Secret<String>);

impl SecretString {
    pub fn new(value: String) -> Self {
        Self(Secret::new(value))
    }

    pub fn expose(&self) -> &str {
        self.0.expose_secret()
    }
}

impl<'de> Deserialize<'de> for SecretString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|s| Self(Secret::new(s)))
    }
}

impl Serialize for SecretString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.expose_secret().serialize(serializer)
    }
}
