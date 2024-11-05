[![Crate](https://img.shields.io/crates/v/rskey.svg)](https://crates.io/crates/rskey)
[![Docs](https://docs.rs/rskey/badge.svg)](https://docs.rs/rskey)
![CI](https://github.com/bitfield/rskey/actions/workflows/ci.yml/badge.svg)
![Audit](https://github.com/bitfield/rskey/actions/workflows/audit.yml/badge.svg)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# rskey

A simple persistent key-value store that wraps `HashMap`.

### Getting started

```rust
use rskey::Store;

let mut s = Store::open(path)?;
s.insert("key1".to_string(), "value1".to_string());
assert_eq!(s.get("key1").unwrap(), "value1");
s.sync()?;
```

A basic CLI tool is also included to list, get, and set key-value pairs.

### Installation

```sh
cargo install rskey
```

### Usage

The `rskey` tool expects to find a data file named `store.kv` in the current
directory. If there is no such file, one will be created as soon as you set a
key.

#### Listing all data

```sh
rskey list
```
```
key1: value1
key2: value2
```

#### Getting a value by key

```sh
rskey get key1
```
```
key1: value1
```

#### Setting a key-value pair

```sh
rskey set key3 value3
```

Current version: 0.4.0

License: MIT OR Apache-2.0
