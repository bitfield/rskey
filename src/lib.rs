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

#[derive(PartialEq, Debug)]
pub struct Pair {
    key: &'static str,
    value: &'static str,
}
