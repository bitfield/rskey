use serde::{Deserialize, Serialize};
use std::collections::{hash_map, HashMap};
use std::ffi::OsString;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};

#[derive(Debug, Deserialize, Serialize)]
pub struct Store {
    path: OsString,
    data: HashMap<String, String>,
}

impl Store {
    /// # Errors
    ///
    /// Will return `Err` for any error opening the file other than
    /// [`ErrorKind::NotFound`].
    pub fn open_or_create(path: &OsString) -> Result<Self, std::io::Error> {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let data = serde_json::from_reader(reader)?;
                Ok(Self {
                    path: path.clone(),
                    data,
                })
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Self {
                path: path.clone(),
                data: HashMap::new(),
            }),
            Err(e) => Err(e),
        }
    }

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

    /// # Errors
    ///
    /// Will return any `Err` encountered by [`Self::save`].
    pub fn set(&mut self, k: &str, v: &str) -> Result<(), std::io::Error> {
        self.data.insert(k.to_string(), v.to_string());
        self.save()
    }

    #[must_use]
    pub fn get(&self, k: &str) -> Option<&String> {
        self.data.get(k)
    }

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
        let (_td, s) = new_test_store();
        assert_eq!(
            Vec::<(&String, &String)>::new(),
            s.iter().collect::<Vec<_>>(),
            "unexpected data found in new store"
        );
    }

    #[test]
    fn get_returns_none_for_nonexistent_key() {
        let (_td, s) = new_test_store();
        assert_eq!(None, s.get("bogus"), "unexpected value found for bogus");
    }

    #[test]
    fn get_returns_expected_value_for_existing_key() {
        let (_td, mut s) = new_test_store();
        s.set("foo", "bar").unwrap();
        assert_eq!(
            &"bar".to_string(),
            s.get("foo").unwrap(),
            "get returned unexpected result"
        );
    }

    #[test]
    fn set_same_key_fn_overwrites_old_value_and_returns_it() {
        let (_td, mut s) = new_test_store();
        s.set("foo", "old").unwrap();
        s.set("foo", "new").unwrap();
        assert_eq!(
            &"new".to_string(),
            s.get("foo").unwrap(),
            "old value not overwritten by new"
        );
        assert_eq!(
            s.get("foo").unwrap(),
            &"new".to_string(),
            "no value found for existing key"
        );
    }

    #[test]
    fn store_contains_expected_data() {
        let (_td, mut s) = new_test_store();
        s.set("k1", "v1").unwrap();
        s.set("k2", "v2").unwrap();
        let (k2, v2, k1, v1) = (
            String::from("k2"),
            String::from("v2"),
            String::from("k1"),
            String::from("v1"),
        );
        let want = vec![(&k1, &v1), (&k2, &v2)];
        let mut data = s.iter().collect::<Vec<_>>();
        data.sort();
        assert_eq!(want, data, "expected data not returned");
    }

    #[test]
    fn open_or_create_fn_accepts_nonexistent_path() {
        let s = Store::open_or_create(&OsString::from("bogus"));
        assert!(s.is_ok(), "unexpected error: {:?}", s.err());
    }

    #[test]
    #[cfg(not(windows))] // can't simulate a non-NotFound error on Windows
    fn open_or_create_fn_errors_on_invalid_path() {
        use std::fs;
        let tmp_dir = TempDir::new().unwrap();
        let path = tmp_dir.path().join("not_a_directory");
        fs::write(&path, "").unwrap();
        let store_path = path.join("store_file").into_os_string();
        let s = Store::open_or_create(&store_path);
        assert!(s.is_err(), "want error for invalid path, got {s:?}");
    }

    fn new_test_store() -> (TempDir, Store) {
        let tmp_dir = TempDir::new().unwrap();
        let path = tmp_dir.path().join("store.kv").into_os_string();
        File::create(&path).unwrap();
        (
            tmp_dir,
            Store {
                path,
                data: HashMap::new(),
            },
        )
    }
}
