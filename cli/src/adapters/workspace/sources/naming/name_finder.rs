// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::attribute_names::AttributeNames;
use crate::adapters::workspace::sources::naming::import_scope::ImportScope;
use crate::adapters::workspace::sources::naming::macro_names::MacroNames;
use crate::adapters::workspace::sources::naming::named::Named;
use crate::adapters::workspace::sources::naming::path_names::PathNames;
use crate::adapters::workspace::sources::naming::resolution::Resolution;
use crate::adapters::workspace::sources::naming::use_leaves::UseLeaves;
use syn::Attribute;
use syn::ItemExternCrate;
use syn::ItemUse;
use syn::Macro;
use syn::Path;
use syn::visit::Visit;
use syn::visit::visit_path;

/// Every line naming a dependency, by its package, wherever the parse goes:
/// the paths of the code, its types and its patterns, its `use` and `extern
/// crate` items, its macros and the tokens they are given, its attributes and
/// its derives. What a macro expands to is never seen.
pub struct NameFinder<'a> {
    scope: &'a ImportScope<'a>,
    found: Vec<Named>,
}

impl<'a> NameFinder<'a> {
    pub fn new(scope: &'a ImportScope<'a>) -> Self {
        Self {
            scope,
            found: Vec::new(),
        }
    }

    pub fn found(&self) -> Vec<Named> {
        let mut found = self.found.clone();
        found.sort();
        found.dedup();
        found
    }

    pub fn packages(&self) -> Vec<String> {
        let mut packages = self
            .found
            .iter()
            .map(|named| named.package.clone())
            .collect::<Vec<String>>();
        packages.sort();
        packages.dedup();
        packages
    }

    pub fn uses(&self, item: &ItemUse) -> Vec<Named> {
        UseLeaves::new(item)
            .of(&item.tree)
            .into_iter()
            .filter_map(
                |leaf| match self.scope.root(&leaf.root, leaf.global, true) {
                    Resolution::Crate(package) => Some([
                        Named {
                            package: package.clone(),
                            line: leaf.root_line,
                        },
                        Named {
                            package,
                            line: leaf.line,
                        },
                    ]),
                    Resolution::Local | Resolution::Unknown => None,
                },
            )
            .flatten()
            .collect()
    }
}

impl<'ast> Visit<'ast> for NameFinder<'_> {
    fn visit_attribute(&mut self, node: &'ast Attribute) {
        self.found.extend(AttributeNames::new(self.scope).of(node));
    }

    fn visit_item_extern_crate(&mut self, node: &'ast ItemExternCrate) {
        for attribute in &node.attrs {
            self.visit_attribute(attribute);
        }
        self.found
            .extend(PathNames::new(self.scope).ident(&node.ident, true, true, &[]));
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        for attribute in &node.attrs {
            self.visit_attribute(attribute);
        }
        self.found.extend(self.uses(node));
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        self.visit_path(&node.path);
        self.found.extend(MacroNames::new(self.scope).of(node));
    }

    fn visit_path(&mut self, node: &'ast Path) {
        self.found.extend(PathNames::new(self.scope).of(node));
        visit_path(self, node);
    }
}
