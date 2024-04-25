use std::collections::HashMap;

fn main() {
    let mut s = Store::new();
    s.set("foo", "bar");
    let v = s.get("foo");
    match v {
        Some(v) => println!("{}", v),
        None => println!("not found"),
    }
    println!("{:?}", s.all())
}

struct Store {
    data: HashMap<String, String>,
}

impl Store {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
    fn set(&mut self, k: &str, v: &str) {
        self.data.insert(k.to_string(), v.to_string());
    }
    fn get(&self, k: &str) -> Option<&String> {
        return self.data.get(k);
    }
    fn all(self) -> HashMap<String, String> {
        self.data
    }
}
