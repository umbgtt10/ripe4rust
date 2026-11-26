// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::code_names::CodeNames;
use crate::adapters::workspace::sources::naming::declarations::Declarations;
use crate::adapters::workspace::sources::naming::resolution::Resolution;
use crate::adapters::workspace::sources::naming::std_names::StdNames;
use crate::adapters::workspace::sources::naming::target_macros::TargetMacros;
use crate::adapters::workspace::sources::naming::use_leaf::UseLeaf;

const KEYWORDS: [&str; 4] = ["Self", "crate", "self", "super"];

#[derive(Clone, Debug, PartialEq, Eq)]
struct Binding {
    pub name: String,
    pub package: Option<String>,
}

/// What the names of one file stand for: the names its `use` and `extern
/// crate` items bind -- through one another, `use a::b; use b::C;`, until
/// nothing changes --, its own
/// items, the dependencies its member declares, the globs it imports from
/// them, and the macros its target sees. A binding or an item of the file's
/// shadows a dependency of the same name. A dependency's own name counts only
/// where it can stand: at the start of a longer path, an attribute, a `use`
/// or an `extern crate` item -- a single name in an expression or a type is
/// a variable's, a parameter's or a binding's.
pub struct ImportScope<'a> {
    declarations: &'a Declarations,
    codes: &'a CodeNames,
    macros: &'a TargetMacros,
    bound: Vec<Binding>,
    globs: Vec<String>,
}

impl<'a> ImportScope<'a> {
    pub fn new(
        declarations: &'a Declarations,
        codes: &'a CodeNames,
        macros: &'a TargetMacros,
    ) -> Self {
        let mut scope = Self {
            declarations,
            codes,
            macros,
            bound: Vec::new(),
            globs: Vec::new(),
        };
        for _ in 0..=declarations.uses.len() {
            let bound = scope.bindings();
            if bound == scope.bound {
                break;
            }
            scope.bound = bound;
        }
        scope.globs = scope.glob_packages();
        scope
    }

    pub fn defines_macro(&self, name: &str) -> bool {
        self.macros.defines(name)
    }

    pub fn globs(&self) -> &[String] {
        &self.globs
    }

    pub fn macro_crates(&self) -> Vec<String> {
        let mut crates = [self.macros.macro_use(), &self.globs].concat();
        crates.sort();
        crates.dedup();
        crates
    }

    pub fn root(&self, name: &str, global: bool, own_name: bool) -> Resolution {
        if global {
            return self
                .codes
                .package(name)
                .map_or(Resolution::Local, |package| {
                    Resolution::Crate(String::from(package))
                });
        }
        if let Some(bound) = self.bound(name) {
            return bound;
        }
        let local =
            self.declarations.locals.iter().any(|local| local == name) || StdNames.prelude(name);
        match self.codes.package(name) {
            Some(package) if own_name && !local => Resolution::Crate(String::from(package)),
            _ if local || self.codes.known(name) => Resolution::Local,
            _ => Resolution::Unknown,
        }
    }

    fn bindings(&self) -> Vec<Binding> {
        let externs = self.declarations.externs.iter().map(|declared| Binding {
            name: declared
                .alias
                .clone()
                .unwrap_or_else(|| declared.name.clone()),
            package: self.codes.package(&declared.name).map(String::from),
        });
        let uses = self.declarations.uses.iter().filter_map(|leaf| {
            Some(Binding {
                name: leaf.bound.clone()?,
                package: self.package_of(leaf),
            })
        });
        externs.chain(uses).collect()
    }

    fn bound(&self, name: &str) -> Option<Resolution> {
        if KEYWORDS.contains(&name) {
            return Some(Resolution::Local);
        }
        self.bound
            .iter()
            .find(|binding| binding.name == name)
            .map(|binding| {
                binding
                    .package
                    .clone()
                    .map_or(Resolution::Local, Resolution::Crate)
            })
    }

    fn glob_packages(&self) -> Vec<String> {
        let mut globs = self
            .declarations
            .uses
            .iter()
            .filter(|leaf| leaf.glob)
            .filter_map(|leaf| self.package_of(leaf))
            .collect::<Vec<String>>();
        globs.sort();
        globs.dedup();
        globs
    }

    fn package_of(&self, leaf: &UseLeaf) -> Option<String> {
        match self.root(&leaf.root, leaf.global, true) {
            Resolution::Crate(package) => Some(package),
            Resolution::Local | Resolution::Unknown => None,
        }
    }
}
