#![warn(missing_docs)]
//! A simple key-value store of strings.
//!
//! ## Getting started
//!
//! ```no_run
//! # fn main() -> std::io::Result<()> {
//! # use std::path::Path;
//! use rskey::Store;
//!
//! let mut s = Store::open_or_create(Path::new("data.kv"))?;
//! s.set("key1", "value1")?;
//! assert_eq!("value1", s.get("key1").unwrap());
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::{hash_map, HashMap};
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::path::{Path, PathBuf};

/// A key-value store associated with a particular data file.
///
/// Changes to the store (for example, adding a new key-value pair with
/// [`Self::set()`]) are automatically persisted to the file.
#[derive(Debug, Deserialize, Serialize)]
pub struct Store {
    path: PathBuf,
    data: HashMap<String, String>,
}

impl Store {
    /// Create a [`Store`] associated with a data file at the given `path`.
    ///
    /// If the specified file does not exist, one will be created as soon as
    /// the Store is saved (for example, on calling [`Self::set()`]).
    ///
    /// ```rust
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
    pub fn open_or_create(store_path: &Path) -> Result<Self, std::io::Error> {
        let path: PathBuf = store_path.into();
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
    /// ```no_run
    /// # fn main() -> std::io::Result<()> {
    /// # use rskey::Store;
    /// # use std::path::Path;
    /// let mut s = Store::open_or_create(Path::new("data.kv"))?;
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
    /// ```should_panic
    /// # fn main() -> std::io::Result<()> {
    /// # use rskey::Store;
    /// # use std::path::Path;
    /// let s = Store::open_or_create(Path::new("data.kv"))?;
    /// let v = s.get("key1").unwrap();
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// Creates an iterator of (key, value) tuples from the store's data.
    #[must_use]
    pub fn iter(&self) -> hash_map::Iter<String, String> {
        self.data.iter()
    }
}

impl<'a> IntoIterator for &'a Store {
    type IntoIter = std::collections::hash_map::Iter<'a, std::string::String, std::string::String>;
    type Item = (&'a std::string::String, &'a std::string::String);
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use tempfile::TempDir;

    #[test]
    fn new_store_contains_no_data() {
        let s = new_tmp_store();
        assert!(
            s.store.data.is_empty(),
            "unexpected data found in new store"
        );
    }

    #[test]
    fn get_returns_none_for_nonexistent_key() {
        let s = new_tmp_store();
        if let Some(v) = s.store.get("bogus") {
            panic!("unexpected value {v} found for bogus");
        }
    }

    #[test]
    fn get_returns_expected_value_for_existing_key() {
        let mut s = new_tmp_store();
        s.store.set("foo", "bar").unwrap();
        assert_eq!(
            "bar",
            s.store.get("foo").unwrap(),
            "get returned unexpected result"
        );
    }

    #[test]
    fn set_same_key_fn_overwrites_old_value_and_returns_it() {
        let mut s = new_tmp_store();
        s.store.set("foo", "old").unwrap();
        s.store.set("foo", "new").unwrap();
        match s.store.get("foo").map(String::as_str) {
            Some("new") => (),
            Some("old") => panic!("old value not overwritten by new"),
            Some(v) => panic!("incorrect value {v} for new key"),
            None => panic!("no value found for existing key"),
        }
    }

    #[test]
    fn store_contains_expected_data() {
        let mut s = new_tmp_store();
        s.store.set("k1", "v1").unwrap();
        s.store.set("k2", "v2").unwrap();
        let (k2, v2, k1, v1) = (
            String::from("k2"),
            String::from("v2"),
            String::from("k1"),
            String::from("v1"),
        );
        let want = vec![(&k1, &v1), (&k2, &v2)];
        let mut data = s.store.iter().collect::<Vec<_>>();
        data.sort();
        assert_eq!(want, data, "expected data not returned");
    }

    #[test]
    fn store_persists_changes() {
        let mut s = new_tmp_store();
        s.store.set("k1", "v1").unwrap();
        let s2 = Store::open_or_create(&s.store.path).unwrap();
        assert_eq!("v1", s2.get("k1").unwrap(), "expected data not returned");
    }

    #[test]
    fn store_implements_into_iterator() {
        let mut s = new_tmp_store();
        s.store.set("k1", "v1").unwrap();
        assert_eq!("k1", s.store.into_iter().next().unwrap().0);
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
        let path = tmp_dir.path().join("not_a_directory");
        fs::write(&path, "").unwrap();
        let store_path = path.join("store_file");
        let s = Store::open_or_create(&store_path);
        assert!(s.is_err(), "want error for invalid path, got {s:?}");
    }

    struct TestFixture {
        _tmp_dir: TempDir,
        store: Store,
    }
    fn new_tmp_store() -> TestFixture {
        let tmp_dir = TempDir::new().unwrap();
        let path = tmp_dir.path().join("store.kv");
        File::create(&path).unwrap();
        TestFixture {
            _tmp_dir: tmp_dir,
            store: Store {
                path,
                data: HashMap::new(),
            },
        }
    }
}
