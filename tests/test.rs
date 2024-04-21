#![allow(clippy::uninlined_format_args)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[test]
fn test_deserialize() {
    let mut json = String::new();
    for _ in 0..10000 {
        json = format!("[{}]", json);
    }

    let mut deserializer = serde_json::Deserializer::from_str(&json);
    deserializer.disable_recursion_limit();
    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
    let value = Value::deserialize(deserializer).unwrap();

    drop_carefully(value);
}

#[test]
fn test_serialize() {
    let mut value = Value::Null;
    for _ in 0..10000 {
        value = Value::Array(vec![value]);
    }

    let mut out = Vec::new();
    let mut serializer = serde_json::Serializer::new(&mut out);
    let serializer = serde_stacker::Serializer::new(&mut serializer);
    let result = value.serialize(serializer);

    drop_carefully(value);

    result.unwrap();
    assert_eq!(out.len(), 10000 + 4 + 10000);
}

fn drop_carefully(value: Value) {
    let mut stack = vec![value];
    while let Some(value) = stack.pop() {
        if let Value::Array(array) = value {
            stack.extend(array);
        }
    }
}
