// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::exposure::fragment::Fragment;
use crate::adapters::workspace::sources::naming::import_scope::ImportScope;
use crate::adapters::workspace::sources::naming::name_finder::NameFinder;
use syn::visit::Visit;

/// The dependencies a public API's fragments name, each once, read as the
/// footprint reads every line, through the file's scope.
pub struct SurfaceNames<'a> {
    scope: &'a ImportScope<'a>,
}

impl<'a> SurfaceNames<'a> {
    pub fn new(scope: &'a ImportScope<'a>) -> Self {
        Self { scope }
    }

    pub fn of(&self, fragments: &[Fragment<'_>]) -> Vec<String> {
        let mut finder = NameFinder::new(self.scope);
        for fragment in fragments {
            match *fragment {
                Fragment::Attribute(attribute) => finder.visit_attribute(attribute),
                Fragment::Bound(bound) => finder.visit_type_param_bound(bound),
                Fragment::ExternCrate(item) => finder.visit_item_extern_crate(item),
                Fragment::Generics(generics) => finder.visit_generics(generics),
                Fragment::Path(path) => finder.visit_path(path),
                Fragment::Signature(signature) => finder.visit_signature(signature),
                Fragment::Type(ty) => finder.visit_type(ty),
                Fragment::Use(item) => finder.visit_item_use(item),
            }
        }
        finder.packages()
    }
}
