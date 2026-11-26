// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::code_names::CodeNames;
use crate::adapters::workspace::sources::naming::declarations::Declarations;

/// The macros every file of a target sees without naming them: those its
/// `#[macro_use] extern crate` items bring in, by package, and those its
/// files define with `macro_rules!`.
pub struct TargetMacros {
    macro_use: Vec<String>,
    defined: Vec<String>,
}

impl TargetMacros {
    pub fn new(declarations: &[Declarations], codes: &CodeNames) -> Self {
        let mut macro_use = declarations
            .iter()
            .flat_map(|declared| declared.externs.iter())
            .filter(|declared| declared.macro_use)
            .filter_map(|declared| codes.package(&declared.name))
            .map(String::from)
            .collect::<Vec<String>>();
        macro_use.sort();
        macro_use.dedup();
        Self {
            macro_use,
            defined: declarations
                .iter()
                .flat_map(|declared| declared.macros.iter().cloned())
                .collect(),
        }
    }

    pub fn defines(&self, name: &str) -> bool {
        self.defined.iter().any(|defined| defined == name)
    }

    pub fn macro_use(&self) -> &[String] {
        &self.macro_use
    }
}
