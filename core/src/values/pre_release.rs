// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use core::cmp::Ordering;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// The dot-separated identifiers after a version's `-`, ordered as semver
/// orders them; none at all is a release, after every pre-release of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreRelease {
    text: String,
}

impl PreRelease {
    pub fn new(text: &str) -> Self {
        Self {
            text: String::from(text),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn is_well_formed(&self) -> bool {
        self.text.split('.').all(|identifier| {
            !identifier.is_empty()
                && identifier
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '-')
        })
    }

    fn compare_identifiers(left: &str, right: &str) -> Ordering {
        match (left.parse::<u64>(), right.parse::<u64>()) {
            (Ok(left), Ok(right)) => left.cmp(&right),
            (Ok(_), Err(_)) => Ordering::Less,
            (Err(_), Ok(_)) => Ordering::Greater,
            (Err(_), Err(_)) => left.cmp(right),
        }
    }
}

impl Ord for PreRelease {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.is_empty(), other.is_empty()) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => self
                .text
                .split('.')
                .zip(other.text.split('.'))
                .map(|(left, right)| Self::compare_identifiers(left, right))
                .find(|ordering| ordering.is_ne())
                .unwrap_or_else(|| {
                    self.text
                        .split('.')
                        .count()
                        .cmp(&other.text.split('.').count())
                }),
        }
    }
}

impl PartialOrd for PreRelease {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for PreRelease {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_str(&self.text)
    }
}
