use std::collections::HashMap;

pub struct Store<'a> {
    data: HashMap<&'a str, &'a str>,
}

impl<'a> Store<'a> {
    pub fn new() -> Self {
        let data = HashMap::new();
        Self { data }
    }
    pub fn set(&mut self, k: &'a str, v: &'a str) {
        self.data.insert(k, v);
    }
    pub fn get(&self, k: &str) -> Option<&str> {
        return self.data.get(k).copied();
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<&'a str, &'a str> {
        self.data.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_store_contains_no_data() {
        let s = Store::new();
        assert_eq!(
            Vec::<(&&str, &&str)>::new(),
            s.iter().collect::<Vec<_>>(),
            "unexpected data found in new store"
        )
    }

    #[test]
    fn get_nonexistent_key_returns_none() {
        let s = Store::new();
        assert!(s.get("bogus").is_none(), "unexpected value found for bogus")
    }

    #[test]
    fn get_existing_key_returns_expected_value() {
        let mut s = Store::new();
        s.set("foo", "bar");
        assert_eq!(s.get("foo"), Some("bar"), "no value found for existing key")
    }

    #[test]
    fn set_same_key_overwrites_previous_value() {
        let mut s = Store::new();
        s.set("foo", "old");
        s.set("foo", "new");
        assert_ne!(
            s.get("foo"),
            Some("old"),
            "old value not overwritten by new"
        );
        assert_eq!(s.get("foo"), Some("new"), "no value found for existing key")
    }

    #[test]
    fn store_contains_expected_data() {
        let mut s = Store::new();
        s.set("foo", "bar");
        s.set("baz", "quux");
        let want = vec![(&"baz", &"quux"), (&"foo", &"bar")];
        let mut data: Vec<(&&str, &&str)> = s.iter().collect();
        data.sort();
        assert_eq!(want, data, "expected data not returned")
    }
}
