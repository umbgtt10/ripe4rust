// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;
use serde_json::Value;
use serde_json::to_string;
use serde_json::to_value;

pub fn text(document: &impl Serialize) -> String {
    to_string(document).expect("a JSON text")
}

pub fn value(document: &impl Serialize) -> Value {
    to_value(document).expect("a JSON value")
}
