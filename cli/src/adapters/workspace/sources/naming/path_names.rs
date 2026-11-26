// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::import_scope::ImportScope;
use crate::adapters::workspace::sources::naming::named::Named;
use crate::adapters::workspace::sources::naming::resolution::Resolution;
use crate::adapters::workspace::sources::naming::span_line::SpanLine;
use syn::Ident;
use syn::Path;

const NOTHING: &[String] = &[];

/// The dependency a path names by its first name: one a `use` item binds to
/// it, or the dependency's own name starting a longer path or a path from the
/// root, `::`. A first name nothing declares that a glob may have brought --
/// a longer path's, or one in capitals as a type's is -- is put down to every
/// dependency the file glob-imports.
pub struct PathNames<'a> {
    scope: &'a ImportScope<'a>,
}

impl<'a> PathNames<'a> {
    pub fn new(scope: &'a ImportScope<'a>) -> Self {
        Self { scope }
    }

    pub fn ident(
        &self,
        ident: &Ident,
        global: bool,
        own_name: bool,
        unknown: &[String],
    ) -> Vec<Named> {
        let line = SpanLine.of(ident.span());
        match self.scope.root(&ident.to_string(), global, own_name) {
            Resolution::Crate(package) => vec![Named { package, line }],
            Resolution::Unknown => unknown
                .iter()
                .map(|package| Named {
                    package: package.clone(),
                    line,
                })
                .collect(),
            Resolution::Local => Vec::new(),
        }
    }

    pub fn of(&self, path: &Path) -> Vec<Named> {
        let Some(first) = path.segments.first() else {
            return Vec::new();
        };
        let longer = path.segments.len() > 1;
        let capital = first.ident.to_string().starts_with(char::is_uppercase);
        let unknown = if longer || capital {
            self.scope.globs()
        } else {
            NOTHING
        };
        self.ident(&first.ident, path.leading_colon.is_some(), longer, unknown)
    }
}
