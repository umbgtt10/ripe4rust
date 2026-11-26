// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::string::String;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// A unit's id: `<kind>:<crate>`, `rest` for the rest, `advisory:<crate>@<version>`
/// for an advisory row; one of its moves, for bisecting it, `<unit>/<crate>`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnitId {
    text: String,
}

impl UnitId {
    pub fn rest() -> Self {
        Self {
            text: String::from("rest"),
        }
    }

    pub fn of(kind: UnitKind, name: &str) -> Self {
        Self {
            text: format!("{}:{name}", kind.label()),
        }
    }

    pub fn split(&self, name: &str) -> String {
        format!("{}/{name}", self.text)
    }
}

impl Display for UnitId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(&self.text)
    }
}
