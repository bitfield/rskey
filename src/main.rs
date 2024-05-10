use rsk::Store;

fn main() {
    let mut s = Store::open_or_create("store.tmp").unwrap();
    s.set("foo", "bar");
    let v = s.get("bogus");
    match v {
        Some(v) => println!("{v}"),
        None => println!("not found"),
    }
    for (k, v) in s.iter() {
        println!("{k}: {v}");
    }
}
