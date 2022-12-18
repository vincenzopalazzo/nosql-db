//! No SQL database interface that provide an
//! easy way to interact with any kind of
//! key value databases.
//!
//! The main reason to have a key value database
//! is that you should be able to switch between
//! databases without a big refactoring.
//!
//! This is possible because usually a key value
//! database provide a interface with simple
//! API, like you are insert inside a map.
#![feature(associated_type_defaults)]

pub mod err;

/// Simple No SQL interface that expose simple
/// sync and async API to interact with the
/// database.
pub trait NoSQL: Sized {
    type Err = err::Error;

    /// create a new instance of the database with the
    /// URI provided.
    ///
    /// The URI can be a http link or a simple disk path.
    fn new(uri: &str) -> Result<Self, Self::Err>;

    /// get the value with the key inside the database
    /// panic if the value if the look up fails.
    fn get(&self, key: &str) -> String;

    /// put the value inside the database with the key
    /// panic is there is any error while communicating with
    /// the db.
    fn put(&self, key: &str, value: &str);

    /// like the `get` API but return an error
    /// if the key is not present
    fn opt_get(&self, key: &str) -> Result<String, Self::Err>;

    /// like the `put` API bit return and error if
    /// this occurs.
    fn opt_put(&self, key: &str, value: &str) -> Result<(), Self::Err>;

    /// check if the key is present inside the database
    fn contains(&self, key: &str) -> bool;

    /// return the list of keys that are insert inside the
    /// database.
    fn keys(&self) -> Vec<&'static str>;

    // FIXME: add method to iterate over a range
    // of keys.
}