use rsk::Store;
use std::ffi::OsString;

fn main() {
    let mut s = Store::open_or_create(&OsString::from("store.kv")).unwrap();
    s.set("baz", "quux").unwrap();
    s.iter().for_each(|(k, v)| println!("{k}: {v}"));
}
