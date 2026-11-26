// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::comparator::Comparator;
use crate::values::parse_failure::ParseFailure;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;

/// A version requirement as cargo writes one: comparators joined by commas,
/// all of which a version must meet. A pre-release meets it only where a
/// comparator names a pre-release of the same version.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Requirement {
    text: String,
    comparators: Vec<Comparator>,
}

impl Requirement {
    pub fn parse(text: &str) -> Result<Self, ParseFailure> {
        let failure = || ParseFailure {
            text: String::from(text),
            expected: "a version requirement",
        };
        if text.trim().is_empty() {
            return Err(failure());
        }
        let comparators = text
            .split(',')
            .map(Comparator::parse)
            .collect::<Result<Vec<Comparator>, ParseFailure>>()
            .map_err(|_| failure())?;
        Ok(Self {
            text: String::from(text.trim()),
            comparators,
        })
    }

    pub fn exact(&self) -> Option<Version> {
        match self.comparators.as_slice() {
            [only] => only.exact(),
            _ => None,
        }
    }

    pub fn matches(&self, version: &Version) -> bool {
        self.comparators
            .iter()
            .all(|comparator| comparator.matches(version))
            && (!version.is_pre_release()
                || self
                    .comparators
                    .iter()
                    .any(|comparator| comparator.names_pre_release_of(version)))
    }
}

impl Display for Requirement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.text)
    }
}
