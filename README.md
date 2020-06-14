Serde stack growth adapter
==========================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/serde--stacker-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/serde-stacker)
[<img alt="crates.io" src="https://img.shields.io/crates/v/serde_stacker.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/serde_stacker)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-serde--stacker-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/serde_stacker)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/dtolnay/serde-stacker/CI/master?style=for-the-badge" height="20">](https://github.com/dtolnay/serde-stacker/actions?query=branch%3Amaster)

This crate provides a Serde adapter that avoids stack overflow by dynamically
growing the stack.

Be aware that you may need to protect against other recursive operations outside
of serialization and deserialization when working with deeply nested data,
including, but not limited to, Display and Debug and Drop impls.

```toml
[dependencies]
serde = "1.0"
serde_stacker = "0.1"
```

## Deserialization example

```rust
use serde::Deserialize;
use serde_json::Value;

fn main() {
    let mut json = String::new();
    for _ in 0..10000 {
        json = format!("[{}]", json);
    }

    let mut deserializer = serde_json::Deserializer::from_str(&json);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let value = Value::deserialize(deserializer).unwrap();

    carefully_drop_nested_arrays(value);
}

fn carefully_drop_nested_arrays(value: Value) {
    let mut stack = vec![value];
    while let Some(value) = stack.pop() {
        if let Value::Array(array) = value {
            stack.extend(array);
        }
    }
}
```

## Serialization example

```rust
use serde::Serialize;
use serde_json::Value;

fn main() {
    let mut value = Value::Null;
    for _ in 0..10000 {
        value = Value::Array(vec![value]);
    }

    let mut out = Vec::new();
    let mut serializer = serde_json::Serializer::new(&mut out);
    let serializer = serde_stacker::Serializer::new(&mut serializer);
    let result = value.serialize(serializer);

    carefully_drop_nested_arrays(value);

    result.unwrap();
    assert_eq!(out.len(), 10000 + "null".len() + 10000);
}

fn carefully_drop_nested_arrays(value: Value) {
    let mut stack = vec![value];
    while let Some(value) = stack.pop() {
        if let Value::Array(array) = value {
            stack.extend(array);
        }
    }
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
