use rskey::Store;
use std::{env, path::Path, process};

const USAGE: &str = r"Usage:
rskey list - list all key-value pairs
rskey get KEY - show value for KEY
rskey set KEY VALUE - set KEY to VALUE";

fn main() {
    let path = Path::new("./store.kv");
    let mut s = Store::open_or_create(path).unwrap_or_else(|e| {
        eprintln!("opening {}: {e:?}", path.display());
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
            if let Err(e) = s.set(key, value) {
                eprintln!("writing to {}: {e:?}", s.path.display());
                process::exit(1);
            }
        }
        _ => {
            println!("{USAGE}");
        }
    }
}
