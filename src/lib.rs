//! Serde adapter that avoids stack overflow by dynamically growing the stack.
//!
//! Be aware that you may need to protect against other recursive operations
//! outside of serialization and deserialization when working with deeply nested
//! data, including, but not limited to, Display and Debug and Drop impls.
//!
//! # Deserialization example
//!
//! ```
//! use serde::Deserialize;
//! use serde_json::Value;
//!
//! fn main() {
//!     let mut json = String::new();
//!     for _ in 0..10000 {
//!         json = format!("[{}]", json);
//!     }
//!
//!     let mut deserializer = serde_json::Deserializer::from_str(&json);
//!     deserializer.disable_recursion_limit();
//!     let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
//!     let value = Value::deserialize(deserializer).unwrap();
//!
//!     carefully_drop_nested_arrays(value);
//! }
//!
//! fn carefully_drop_nested_arrays(value: Value) {
//!     let mut stack = vec![value];
//!     while let Some(value) = stack.pop() {
//!         if let Value::Array(array) = value {
//!             stack.extend(array);
//!         }
//!     }
//! }
//! ```
//!
//! # Serialization example
//!
//! ```
//! use serde::Serialize;
//! use serde_json::Value;
//!
//! fn main() {
//!     let mut value = Value::Null;
//!     for _ in 0..10000 {
//!         value = Value::Array(vec![value]);
//!     }
//!
//!     let mut out = Vec::new();
//!     let mut serializer = serde_json::Serializer::new(&mut out);
//!     let serializer = serde_stacker::Serializer::new(&mut serializer);
//!     let result = value.serialize(serializer);
//!
//!     carefully_drop_nested_arrays(value);
//!
//!     result.unwrap();
//!     assert_eq!(out.len(), 10000 + 4 + 10000);
//! }
//!
//! fn carefully_drop_nested_arrays(value: Value) {
//!     let mut stack = vec![value];
//!     while let Some(value) = stack.pop() {
//!         if let Value::Array(array) = value {
//!             stack.extend(array);
//!         }
//!     }
//! }
//! ```

mod de;
mod param;
mod ser;

pub use de::{deserialize, Deserializer};
pub use ser::{serialize, Serializer};
