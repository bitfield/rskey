use std::collections::HashMap;

pub struct Store {
    data: HashMap<&'static str, &'static str>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    pub fn set(&mut self, k: &'static str, v: &'static str) {
        self.data.insert(k, v);
    }
    pub fn get(&self, k: &str) -> Option<&str> {
        return self.data.get(k).copied();
    }
    pub fn all(self) -> Vec<Pair> {
        let mut result = Vec::with_capacity(self.data.len());
        for (k, v) in self.data {
            result.push(Pair { key: k, value: v })
        }
        result
    }
}

#[derive(PartialEq, Debug, Eq, PartialOrd, Ord)]
pub struct Pair {
    key: &'static str,
    value: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_store_contains_no_data() {
        let s = Store::new();
        assert!(s.all().is_empty(), "unexpected data found in new store")
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
    fn all_returns_all_pairs() {
        let mut s = Store::new();
        s.set("foo", "bar");
        s.set("baz", "quux");
        let want = vec![
            Pair {
                key: "baz",
                value: "quux",
            },
            Pair {
                key: "foo",
                value: "bar",
            },
        ];
        let mut got = s.all();
        got.sort();
        assert_eq!(want, got, "expected data not returned")
    }
}
