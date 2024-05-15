use rsk::Store;
use std::env;
use std::ffi::OsString;
use std::process;

const USAGE: &str = r#"Usage:
rsk list - list all key-value pairs
rsk get KEY - show value for KEY
rsk set KEY VALUE - set KEY to VALUE"#;

fn main() {
    let mut s = Store::open_or_create(&OsString::from("store.kv")).unwrap();
    let mut args = env::args();
    let Some(verb) = args.nth(1) else {
        println!("{}", USAGE);
        process::exit(1)
    };
    match verb.as_str() {
        "list" => {
            for (k, v) in &s {
                println!("{k}: {v}")
            }
        }
        "get" => {
            if let Some(key) = args.next() {
                if let Some(value) = s.get(&key) {
                    println!("{key}: {value}")
                } else {
                    println!("key {key} not found")
                }
            } else {
                println!("{}", USAGE);
            }
        }
        "set" => {
            if let Some(key) = args.next() {
                if let Some(value) = args.next() {
                    s.set(&key, &value).unwrap();
                } else {
                    println!("{}", USAGE);
                }
            } else {
                println!("{}", USAGE);
            }
        }
        _ => {
            println!("{}", USAGE);
        }
    }
}
