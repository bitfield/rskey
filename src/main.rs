use rsk::Store;
use std::{env, path::Path, process};

const USAGE: &str = r"Usage:
rsk list - list all key-value pairs
rsk get KEY - show value for KEY
rsk set KEY VALUE - set KEY to VALUE";

fn main() {
    let mut s = Store::open_or_create(Path::new("store.kv")).unwrap_or_else(|e| {
        eprintln!("oh no: {e:?}");
        process::exit(1);
    });
    let raw_args: Vec<_> = env::args().collect();
    let args: Vec<_> = raw_args.iter().map(String::as_str).collect();
    match args.get(1..) {
        Some(["list"]) => {
            for (k, v) in &s {
                println!("{k}: {v}");
            }
        }
        Some(["get", key]) => {
            if let Some(value) = s.get(key) {
                println!("{key}: {value}");
            } else {
                println!(r#"key "{key}" not found"#);
            }
        }
        Some(["set", key, value]) => {
            s.set(key, value).unwrap();
        }
        _ => {
            println!("{USAGE}");
        }
    }
}
