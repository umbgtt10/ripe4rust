// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::op::Op;
use crate::values::parse_failure::ParseFailure;
use crate::values::partial_version::PartialVersion;
use crate::values::version::Version;
use alloc::string::String;

/// One operator and the version it compares with: `>=0.32`, `=1.0.229`,
/// `^1.2`, `1.*`, `*`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Comparator {
    op: Op,
    version: PartialVersion,
}

impl Comparator {
    pub fn parse(text: &str) -> Result<Self, ParseFailure> {
        let trimmed = text.trim();
        let (op, rest) = if trimmed == "*" {
            (Op::GreaterEq, "0")
        } else {
            Op::split(trimmed)
        };
        let rest = rest.trim();
        let (op, rest) = match rest.strip_suffix(".*") {
            Some(stripped) if op == Op::Caret => (Op::Exact, stripped),
            _ => (op, rest),
        };
        let version = PartialVersion::parse(rest).map_err(|_| ParseFailure {
            text: String::from(text),
            expected: "a version requirement",
        })?;
        Ok(Self { op, version })
    }

    pub fn exact(&self) -> Option<Version> {
        (self.op == Op::Exact && self.version.is_full()).then(|| self.version.floor())
    }

    pub fn matches(&self, version: &Version) -> bool {
        self.op.interval(&self.version).admits(version)
    }

    pub fn names_pre_release_of(&self, version: &Version) -> bool {
        self.version.names_pre_release_of(version)
    }
}
