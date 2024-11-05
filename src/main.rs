use anyhow::Context;
use rskey::Store;
use std::env;

const USAGE: &str = r"Usage:
rskey list - list all key-value pairs
rskey get KEY - show value for KEY
rskey set KEY VALUE - set KEY to VALUE";

fn main() -> anyhow::Result<()> {
    let path = "store.kv";
    let mut s = Store::<String>::open(path).with_context(|| format!("reading {path}"))?;
    let raw_args: Vec<_> = env::args().collect();
    let args: Vec<_> = raw_args.iter().map(String::as_str).collect();
    match args.get(1..) {
        Some(["list"]) => {
            for (k, v) in s {
                println!("{k}: {v}");
            }
        }
        Some(["get", key]) => {
            if let Some(value) = s.get(*key) {
                println!("{key}: {value}");
            } else {
                println!(r#"key "{key}" not found"#);
            };
        }
        Some(["set", key, value]) => {
            s.insert((*key).to_string(), (*value).to_string());
            s.sync().with_context(|| format!("writing {path}"))?;
        }
        _ => {
            println!("{USAGE}");
        }
    }
    Ok(())
}
