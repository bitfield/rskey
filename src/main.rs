use rsk::Store;

fn main() {
    let mut s = Store::open_or_create("store.kv").unwrap();
    s.set("foo", "bar").unwrap();
    s.iter().for_each(|(k, v)| println!("{k}: {v}"));
}
