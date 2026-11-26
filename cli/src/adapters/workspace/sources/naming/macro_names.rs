// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::import_scope::ImportScope;
use crate::adapters::workspace::sources::naming::named::Named;
use crate::adapters::workspace::sources::naming::path_names::PathNames;
use crate::adapters::workspace::sources::naming::std_names::StdNames;
use crate::adapters::workspace::sources::naming::token_names::TokenNames;
use syn::Macro;

/// The dependencies a macro's call names, beyond its path's: the tokens it
/// is given, and a macro called by its name alone -- one a `use` item binds,
/// or, where neither the standard library nor the target defines it, every
/// crate the file glob-imports and the target's `#[macro_use]` crates.
pub struct MacroNames<'a> {
    scope: &'a ImportScope<'a>,
}

impl<'a> MacroNames<'a> {
    pub fn new(scope: &'a ImportScope<'a>) -> Self {
        Self { scope }
    }

    pub fn of(&self, called: &Macro) -> Vec<Named> {
        let mut found = TokenNames::new(self.scope).of(called.tokens.clone());
        if let Some(ident) = called.path.get_ident() {
            let name = ident.to_string();
            let unknown = if StdNames.macro_name(&name) || self.scope.defines_macro(&name) {
                Vec::new()
            } else {
                self.scope.macro_crates()
            };
            found.extend(PathNames::new(self.scope).ident(ident, false, false, &unknown));
        }
        found
    }
}
