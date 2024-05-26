#![warn(missing_docs)]
//! A simple key-value store of strings.
//!
//! ## Getting started
//!
//! ```
//! # fn main() -> std::io::Result<()> {
//! # use std::path::Path;
//! use rskey::Store;
//! use tempfile::TempDir;
//!
//! let tmp_dir = TempDir::new()?;
//! let mut s = Store::open_or_create(tmp_dir.path().join("data.kv"))?;
//! s.set("key1", "value1")?;
//! assert_eq!("value1", s.get("key1").unwrap());
//! # Ok(())
//! # }
//! ```
//!
//! ## Iteration
//!
//! ```
//! # fn main() -> std::io::Result<()> {
//! # use std::path::Path;
//! use rskey::Store;
//! use tempfile::TempDir;
//!
//! let tmp_dir = TempDir::new()?;
//! let mut s = Store::open_or_create(tmp_dir.path().join("data.kv"))?;
//! s.set("key1", "value1")?;
//! s.set("key2", "value2")?;
//! for (key, value) in s {
//!     println!("{key} = ${value}");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! A basic CLI tool is also included to list, get, and set key-value pairs.
//!
//! ## Installation
//!
//! ```sh
//! cargo install rskey
//! ```
//!
//! ## Usage
//!
//! `rskey` expects to find a data file named `store.kv` in the current
//! directory. If there is no such file, one will be created as soon as you set
//! a key.
//!
//! ### Listing all data
//!
//! ```sh
//! rskey list
//! ```
//! ```text
//! key1: value1
//! key2: value2
//! ```
//!
//! ### Getting a value by key
//!
//! ```sh
//! rskey get key1
//! ```
//! ```text
//! key1: value1
//! ```
//!
//! ### Setting a key-value pair
//!
//! ```sh
//! rskey set key3 value3
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::path::Path;

/// A key-value store associated with a particular data file.
///
/// Changes to the store (for example, adding a new key-value pair with
/// [`Self::set()`]) are automatically persisted to the file.
#[derive(Debug, Deserialize, Serialize)]
pub struct Store<P: AsRef<Path>> {
    path: P,
    data: HashMap<String, String>,
}

impl<P: AsRef<Path>> Store<P> {
    /// Create a [`Store`] associated with a data file at the given `path`.
    ///
    /// If the specified file does not exist, one will be created as soon as
    /// the Store is saved (for example, on calling [`Self::set()`]).
    ///
    /// ```
    /// # fn main() -> std::io::Result<()> {
    /// # use rskey::Store;
    /// # use std::path::Path;
    /// let s = Store::open_or_create(Path::new("data.kv"))?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `Err` for any error opening the file other than
    /// [`ErrorKind::NotFound`].
    pub fn open_or_create(path: P) -> Result<Self, std::io::Error> {
        match File::open(&path) {
            Ok(file) => {
                let data = serde_json::from_reader(BufReader::new(file))?;
                Ok(Self { path, data })
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Self {
                path,
                data: HashMap::new(),
            }),
            Err(e) => Err(e),
        }
    }

    /// Write the store data to the associated file.
    ///
    /// It should never be necessary to call `save` explicitly, because any
    /// change to the `Store` (for example, calling [`Self::set()`])
    /// automatically saves the data.
    ///
    /// # Errors
    ///
    /// Will return `Err` for any error creating the file or serializing the
    /// JSON to it.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let file = File::create(&self.path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.data)?;
        Ok(())
    }

    /// Set a `value` associated with a given `key`.
    ///
    /// If `key` already exists, its current value will be overwritten.
    ///
    /// ```
    /// # fn main() -> std::io::Result<()> {
    /// # use rskey::Store;
    /// # use std::path::Path;
    /// use tempfile::TempDir;
    ///
    /// let tmp_dir = TempDir::new()?;
    /// let mut s = Store::open_or_create(tmp_dir.path().join("data.kv"))?;
    /// s.set("key1", "value1")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Will return any `Err` encountered by [`Self::save()`].
    pub fn set(&mut self, key: &str, value: &str) -> Result<(), std::io::Error> {
        self.data.insert(key.to_string(), value.to_string());
        self.save()
    }

    /// Get the value associated with `key`.
    ///
    /// If `key` does not exist in the `Store`, the result will be `None`.
    ///
    /// ```
    /// # fn main() -> std::io::Result<()> {
    /// # use rskey::Store;
    /// # use std::path::Path;
    /// let s = Store::open_or_create(Path::new("data.kv"))?;
    /// assert!(s.get("key1").is_none());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

impl<P: AsRef<Path>> IntoIterator for Store<P> {
    type Item = (String, String);
    type IntoIter = std::collections::hash_map::IntoIter<String, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn new_store_contains_no_data() {
        let s = TmpStore::new();
        assert!(
            s.store.data.is_empty(),
            "unexpected data found in new store"
        );
    }

    #[test]
    fn get_returns_none_for_nonexistent_key() {
        let s = TmpStore::new();
        if let Some(v) = s.store.get("bogus") {
            panic!("unexpected value {v} found for bogus");
        }
    }

    #[test]
    fn get_returns_expected_value_for_existing_key() {
        let mut s = TmpStore::new();
        s.store.set("foo", "bar").unwrap();
        assert_eq!(
            "bar",
            s.store.get("foo").unwrap(),
            "get returned unexpected result"
        );
    }

    #[test]
    fn set_same_key_fn_overwrites_old_value_and_returns_it() {
        let mut tmp = TmpStore::new();
        tmp.store.set("foo", "old").unwrap();
        tmp.store.set("foo", "new").unwrap();
        match tmp.store.get("foo").map(String::as_str) {
            Some("new") => (),
            Some("old") => panic!("old value not overwritten by new"),
            Some(v) => panic!("incorrect value {v} for new key"),
            None => panic!("no value found for existing key"),
        }
    }

    #[test]
    fn store_contains_expected_data() {
        let mut tmp = TmpStore::new();
        tmp.store.set("k1", "v1").unwrap();
        tmp.store.set("k2", "v2").unwrap();
        assert_eq!("v1", tmp.store.get("k1").unwrap());
        assert_eq!("v2", tmp.store.get("k2").unwrap());
    }

    #[test]
    fn store_persists_changes() {
        let mut tmp = TmpStore::new();
        tmp.store.set("k1", "v1").unwrap();
        let s2 = Store::open_or_create(&tmp.store.path).unwrap();
        assert_eq!("v1", s2.get("k1").unwrap(), "expected data not returned");
    }

    #[test]
    fn store_data_implements_into_iterator() {
        let mut tmp = TmpStore::new();
        tmp.store.set("k1", "v1").unwrap();
        assert_eq!(
            ("k1".to_string(), "v1".to_string()),
            tmp.store.data.into_iter().next().unwrap()
        );
    }

    #[test]
    fn open_or_create_fn_accepts_nonexistent_path() {
        let s = Store::open_or_create(Path::new("bogus"));
        assert!(s.is_ok(), "unexpected error: {:?}", s.err());
    }

    #[test]
    #[cfg(not(windows))] // can't simulate a non-NotFound error on Windows
    fn open_or_create_fn_errors_on_invalid_path() {
        use std::fs;
        let tmp_dir = TempDir::new().unwrap();
        let mut path = tmp_dir.path().join("not_a_directory");
        fs::write(&path, "").unwrap();
        path.push("store_file");
        let s = Store::open_or_create(&path);
        assert!(s.is_err(), "want error for invalid path, got {s:?}");
    }

    struct TmpStore<P: AsRef<Path>> {
        _tmp_dir: TempDir,
        store: Store<P>,
    }

    impl TmpStore<PathBuf> {
        fn new() -> Self {
            let tmp_dir = TempDir::new().unwrap();
            let path = tmp_dir.path().join("store.kv");
            File::create(&path).unwrap();
            TmpStore {
                _tmp_dir: tmp_dir,
                store: Store {
                    path,
                    data: HashMap::new(),
                },
            }
        }
    }
}
