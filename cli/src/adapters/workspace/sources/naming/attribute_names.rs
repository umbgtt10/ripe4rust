// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::import_scope::ImportScope;
use crate::adapters::workspace::sources::naming::named::Named;
use crate::adapters::workspace::sources::naming::path_names::PathNames;
use crate::adapters::workspace::sources::naming::std_names::StdNames;
use crate::adapters::workspace::sources::naming::token_names::TokenNames;
use syn::Attribute;
use syn::Meta;
use syn::Path;
use syn::punctuated::Punctuated;
use syn::token::Comma;

const DERIVE: &str = "derive";
const DOC: &str = "doc";

/// The dependencies an attribute names: by its path -- `#[tokio::main]`, one
/// a crate owns by its name, `#[serde(...)]`, or one a `use` item or a glob
/// brings --, by every derive it lists, and by the tokens it is given. A doc
/// comment, a built-in attribute and a built-in derive name none.
pub struct AttributeNames<'a> {
    scope: &'a ImportScope<'a>,
}

impl<'a> AttributeNames<'a> {
    pub fn new(scope: &'a ImportScope<'a>) -> Self {
        Self { scope }
    }

    pub fn derives(&self, attribute: &Attribute) -> Vec<Named> {
        let paths = PathNames::new(self.scope);
        attribute
            .parse_args_with(Punctuated::<Path, Comma>::parse_terminated)
            .map(|derived| {
                derived
                    .iter()
                    .flat_map(|path| match path.get_ident() {
                        Some(ident) if StdNames.derive(&ident.to_string()) => Vec::new(),
                        Some(ident) => paths.ident(ident, false, false, self.scope.globs()),
                        None => paths.of(path),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn of(&self, attribute: &Attribute) -> Vec<Named> {
        let path = attribute.path();
        if path.is_ident(DOC) {
            return Vec::new();
        }
        let paths = PathNames::new(self.scope);
        let mut found = match path.get_ident() {
            Some(ident) if StdNames.attribute(&ident.to_string()) => Vec::new(),
            Some(ident) => paths.ident(ident, false, true, self.scope.globs()),
            None => paths.of(path),
        };
        match &attribute.meta {
            Meta::List(_) if path.is_ident(DERIVE) => found.extend(self.derives(attribute)),
            Meta::List(list) => found.extend(TokenNames::new(self.scope).of(list.tokens.clone())),
            Meta::Path(_) | Meta::NameValue(_) => {}
        }
        found
    }
}
