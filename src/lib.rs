use serde::Deserialize;
use std::collections::{hash_map, HashMap};
use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    pub fn open_or_create<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        match File::open(&path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let s = serde_json::from_reader(reader)?;
                Ok(s)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(Self {
                data: HashMap::new(),
            }),
            Err(e) => Err(e),
        }
    }

    pub fn set(&mut self, k: &str, v: &str) -> Option<String> {
        self.data.insert(k.to_string(), v.to_string())
    }

    pub fn get(&self, k: &str) -> Option<&str> {
        Some(self.data.get(k)?.as_str())
    }

    pub fn iter(&self) -> hash_map::Iter<String, String> {
        self.data.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn new_store_contains_no_data() {
        let s = new_test_store();
        let want: Vec<(&String, &String)> = vec![];
        assert_eq!(
            want,
            s.iter().collect::<Vec<_>>(),
            "unexpected data found in new store"
        )
    }

    #[test]
    fn get_returns_none_for_nonexistent_key() {
        let s = new_test_store();
        assert_eq!(None, s.get("bogus"), "unexpected value found for bogus")
    }

    #[test]
    fn get_returns_expected_value_for_existing_key() {
        let mut s = new_test_store();
        s.set("foo", "bar");
        assert_eq!(Some("bar"), s.get("foo"), "get returned unexpected result")
    }

    #[test]
    fn set_same_key_fn_overwrites_old_value_and_returns_it() {
        let mut s = new_test_store();
        s.set("foo", "old");
        let old = s.set("foo", "new").unwrap();
        assert_eq!("old", old);
        assert_eq!(
            Some("new"),
            s.get("foo"),
            "old value not overwritten by new"
        );
        assert_eq!(s.get("foo"), Some("new"), "no value found for existing key")
    }

    #[test]
    fn store_contains_expected_data() {
        let mut s = new_test_store();
        s.set("foo", "bat");
        s.set("baz", "quux");
        let want = vec![(&"baz", &"quux"), (&"foo", &"bar")];
        let mut data: Vec<(&&str, &&str)> = s.iter().collect();
        data.sort();
        assert_eq!(want, data, "expected data not returned")
    }

    #[test]
    fn open_or_create_fn_accepts_nonexistent_path() {
        let s = Store::open_or_create("bogus");
        assert!(s.is_ok(), "unexpected error: {:?}", s.err())
    }

    #[test]
    fn open_or_create_fn_errors_on_invalid_path() {
        let tmp_dir = TempDir::new().unwrap();
        let path = tmp_dir.path().join("not_a_directory");
        fs::write(&path, "").unwrap();
        // 'TMPDIR/not_a_directory/store_file' is invalid
        // because 'not_a_directory' is not a directory
        let s = Store::open_or_create(path.join("store_file"));
        assert!(s.is_err(), "want error for invalid path")
    }

    fn new_test_store() -> Store {
        Store {
            data: HashMap::new(),
        }
    }
}
