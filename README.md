# rskey

rskey is a simple key-value store of strings, with a basic CLI tool to list, get, and set key-value pairs.

[![Build status](https://github.com/bitfield/rskey/actions/workflows/ci.yml/badge.svg)](https://github.com/bitfield/rskey/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/rskey.svg)](https://crates.io/crates/rskey)

## CLI installation

```sh
cargo install rskey
```

## CLI usage 

rskey expects to find a data file named `store.kv` in the current directory. If there is no such file, one will be created as soon as you set a key.

### Listing all data

```sh
rskey list
```
```
key1: value1
key2: value2
```

### Getting a value by key

```sh
rskey get key1
```
```
key1: value1
```

### Setting a key-value pair

```sh
rskey set key3 value3
```

## Crate usage

Example:

```rust
use rskey::Store;

let mut s = Store::open_or_create("data.kv");
s.set("key3", "value3")?;
assert_eq!("value3", s.get("key3").unwrap());
```
