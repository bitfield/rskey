use rsk::Store;

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
