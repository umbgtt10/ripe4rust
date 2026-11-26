// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// One step: `{ "run": ... }`, or `{ "edit": "file:line", "key": ..., "value": ... }`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum StepDocument<'a> {
    Edit {
        edit: String,
        key: &'a str,
        value: &'a str,
    },
    Run {
        run: &'a str,
    },
}
