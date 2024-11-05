//! A simple persistent key-value store that wraps `HashMap`.
//!
//! ## Getting started
//!
//! ```
//! # fn main() -> std::io::Result<()> {
//! use rskey::Store;
//! # use tempfile::TempDir;
//!
//! # let tmp_dir = TempDir::new()?;
//! # let path = tmp_dir.path().join("data.kv");
//! let mut s = Store::open(path)?;
//! s.insert("key1".to_string(), "value1".to_string());
//! assert_eq!(s.get("key1").unwrap(), "value1");
//! s.sync()?;
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
//! The `rskey` tool expects to find a data file named `store.kv` in the current
//! directory. If there is no such file, one will be created as soon as you set a
//! key.
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

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

/// A key-value store associated with a particular data file.
///
/// Changes to the store are persisted to the file when [`Self::sync()`] is called.
#[derive(Debug, Deserialize, Serialize)]
pub struct Store<V> {
    pub path: PathBuf,
    inner: HashMap<String, V>,
}

impl<V> Store<V>
where
    V: DeserializeOwned + Serialize,
{
    /// Creates a [`Store`] associated with a data file at the given `path`.
    ///
    /// If the specified file does not exist, one will be created as soon as
    /// the Store is saved (for example, by calling [`Self::sync()`]).
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> std::io::Result<()> {
    /// use rskey::Store;
    /// # use tempfile::TempDir;
    ///
    /// # let tmp_dir = TempDir::new()?;
    /// # let path = tmp_dir.path().join("data.kv");
    /// let s = Store::<usize>::open(path)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns any error opening the file (if it exists).
    pub fn open(path: impl AsRef<Path>) -> Result<Self, std::io::Error> {
        let mut store = Self {
            path: path.as_ref().into(),
            inner: HashMap::<String, V>::new(),
        };
        if fs::exists(&path)? {
            store.inner = serde_json::from_reader(BufReader::new(File::open(&path)?))?;
        }
        Ok(store)
    }

    /// Writes the store data to the associated file.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> std::io::Result<()> {
    /// # use tempfile::TempDir;
    /// # use rskey::Store;
    /// # let tmp_dir = TempDir::new()?;
    /// # let path = tmp_dir.path().join("data.kv");
    /// let mut s = Store::<usize>::open(path)?;
    /// s.insert("foo".to_string(), 42);
    /// s.sync()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Will return `Err` for any error creating the file or serializing the
    /// JSON to it.
    pub fn sync(&self) -> Result<(), std::io::Error> {
        let file = File::create(&self.path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self.inner)?;
        Ok(())
    }
}

impl<V> Deref for Store<V> {
    type Target = HashMap<String, V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<V> DerefMut for Store<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<V> IntoIterator for Store<V> {
    type Item = (String, V);

    type IntoIter = IntoIter<String, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
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
        assert!(s.store.is_empty(), "unexpected data found in new store");
    }

    #[test]
    fn sync_persists_changes_to_store() {
        let mut tmp = TmpStore::new();
        assert!(
            tmp.store.insert("k1".into(), "v1".into()).is_none(),
            "key should not already be present in new empty store"
        );
        tmp.store.sync().unwrap();
        let s2 =
            Store::<String>::open(&tmp.store.path).expect("opening existing store should succeed");
        assert_eq!("v1", s2.get("k1").unwrap(), "expected data not returned");
    }

    #[test]
    fn open_or_create_fn_accepts_nonexistent_path() {
        let s = Store::<String>::open(&PathBuf::from("bogus"));
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
        let s = Store::<String>::open(&path);
        assert!(s.is_err(), "want error for invalid path, got {s:?}");
    }

    struct TmpStore {
        _tmp_dir: TempDir,
        store: Store<String>,
    }

    impl TmpStore {
        fn new() -> Self {
            let tmp_dir = TempDir::new().unwrap();
            let path = tmp_dir.path().join("store.kv");
            File::create(&path).unwrap();
            TmpStore {
                _tmp_dir: tmp_dir,
                store: Store {
                    path,
                    inner: HashMap::new(),
                },
            }
        }
    }
}
