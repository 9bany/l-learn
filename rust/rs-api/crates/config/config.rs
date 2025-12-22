use serde::Deserialize;

use serde::de;
use std::{fmt, fmt::Display, marker::PhantomData, str::FromStr};

pub fn deserialize_stringified_any<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: de::Deserializer<'de>,
    T: FromStr,
    T::Err: Display,
{
    deserializer.deserialize_any(StringifiedAnyVisitor(PhantomData))
}

pub struct StringifiedAnyVisitor<T>(PhantomData<T>);

impl<'de, T> de::Visitor<'de> for StringifiedAnyVisitor<T>
where
    T: FromStr,
    T::Err: Display,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing json data")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Self::Value::from_str(v).map_err(E::custom)
    }
}

serde_with::with_prefix!(app "app__");
#[derive(Deserialize, Debug)]
pub struct Config {
    pub env: String,
    #[serde(flatten, with = "app")]
    pub app: App,
}

#[derive(Deserialize, Debug)]
pub struct App {
    #[serde(rename = "log_level")]
    pub log_level: String,
}
impl App {
    pub fn get_log_level(&self) -> log::LevelFilter {
        match self.log_level.as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Info,
        }
    }
}

pub fn from_env() -> Result<Config, envy::Error> {
    envy::from_env::<Config>()
}
