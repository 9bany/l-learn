use pgrx::prelude::*;
use serde::{Serialize, Deserialize};

::pgrx::pg_module_magic!();

#[pg_extern]
fn hello_aggregate() -> &'static str {
    "Hello, aggregate"
}

#[pg_extern]
fn to_lowercase(input: &str) -> String {
    input.to_lowercase()
}

#[pg_extern]
fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
}

#[derive(Serialize, Deserialize, PostgresType)]
struct MyType {
    values: Vec<String>,
    thing: Option<Box<MyType>>
}

#[pg_extern]
fn push_value(mut input: MyType, value: String) -> MyType {
    input.values.push(value);
    input
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_aggregate() {
        assert_eq!("Hello, aggregate", crate::hello_aggregate());
    }

    #[pg_test]
    fn test_to_lowercase() {
        assert_eq!("this is my name", crate::to_lowercase("THIS IS MY NAME"))
    }

    #[pg_test]
    fn test_to_uppercase() {
        assert_eq!("THIS IS MY NAME", crate::to_uppercase("this is my name"))
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
