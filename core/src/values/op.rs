// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::edge::Edge;
use crate::values::interval::Interval;
use crate::values::partial_version::PartialVersion;

const OPERATORS: [(&str, Op); 7] = [
    (">=", Op::GreaterEq),
    ("<=", Op::LessEq),
    (">", Op::Greater),
    ("<", Op::Less),
    ("=", Op::Exact),
    ("~", Op::Tilde),
    ("^", Op::Caret),
];

/// A comparator's operator; a version written bare is a caret.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Caret,
    Exact,
    Greater,
    GreaterEq,
    Less,
    LessEq,
    Tilde,
}

impl Op {
    pub fn split(text: &str) -> (Self, &str) {
        OPERATORS
            .iter()
            .find_map(|(symbol, op)| text.strip_prefix(symbol).map(|rest| (*op, rest)))
            .unwrap_or((Self::Caret, text))
    }

    pub fn interval(self, version: &PartialVersion) -> Interval {
        let floor = version.floor();
        match self {
            Self::Caret => Interval::new(
                Edge::Inclusive(floor),
                Edge::Exclusive(version.caret_ceiling()),
            ),
            Self::Exact => Interval::new(Edge::Inclusive(floor), version.last()),
            Self::Greater => Interval::new(version.last().flipped(), Edge::Open),
            Self::GreaterEq => Interval::new(Edge::Inclusive(floor), Edge::Open),
            Self::Less => Interval::new(Edge::Open, Edge::Exclusive(floor)),
            Self::LessEq => Interval::new(Edge::Open, version.last()),
            Self::Tilde => Interval::new(
                Edge::Inclusive(floor),
                Edge::Exclusive(version.tilde_ceiling()),
            ),
        }
    }
}
