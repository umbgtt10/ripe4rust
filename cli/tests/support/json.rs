// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;
use serde_json::Map;
use serde_json::Value;
use serde_json::to_string;
use serde_json::to_value;

fn keyed<'a>(path: &str, object: &'a Map<String, Value>) -> Vec<(String, &'a Value)> {
    object
        .iter()
        .map(|(key, value)| {
            let joined = if path.is_empty() {
                key.clone()
            } else {
                format!("{path}.{key}")
            };
            (joined, value)
        })
        .collect()
}

/// Every key path a document holds, an array's items as `[]`, each of its
/// objects' keys under its own path.
pub fn paths(document: &Value) -> Vec<String> {
    let mut found = Vec::new();
    let mut pending = vec![(String::new(), document)];
    while let Some((path, value)) = pending.pop() {
        match value {
            Value::Object(object) => pending.extend(keyed(&path, object)),
            Value::Array(items) => {
                pending.extend(items.iter().map(|item| (format!("{path}[]"), item)));
            }
            _ => {}
        }
        if !path.is_empty() && !path.ends_with("[]") {
            found.push(path);
        }
    }
    found.sort();
    found.dedup();
    found
}

pub fn text(document: &impl Serialize) -> String {
    to_string(document).expect("a JSON text")
}

pub fn value(document: &impl Serialize) -> Value {
    to_value(document).expect("a JSON value")
}
