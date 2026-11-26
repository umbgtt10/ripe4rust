// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use syn::Ident;
use syn::ItemUse;
use syn::UseTree;
use syn::ext::IdentExt;

const SELF: &str = "self";

/// What a `use` item re-exports, told by name as the library's types are:
/// the item each of its names stands for -- `a::B` and `a::B as C` both
/// `B`, `a::{self}` the module `a` --, and the module each of its globs
/// opens, `a::*` opening `a`.
pub struct ReExports;

impl ReExports {
    pub fn modules(&self, item: &ItemUse) -> Vec<String> {
        Self::walk(&item.tree, None, true)
    }

    pub fn names(&self, item: &ItemUse) -> Vec<String> {
        Self::walk(&item.tree, None, false)
    }

    fn named<'a>(ident: &'a Ident, parent: Option<&'a Ident>) -> Option<&'a Ident> {
        if ident == SELF { parent } else { Some(ident) }
    }

    fn walk(tree: &UseTree, parent: Option<&Ident>, globs: bool) -> Vec<String> {
        let found = match tree {
            UseTree::Path(path) => return Self::walk(&path.tree, Some(&path.ident), globs),
            UseTree::Group(group) => {
                return group
                    .items
                    .iter()
                    .flat_map(|item| Self::walk(item, parent, globs))
                    .collect();
            }
            UseTree::Name(name) if !globs => Self::named(&name.ident, parent),
            UseTree::Rename(rename) if !globs => Self::named(&rename.ident, parent),
            UseTree::Glob(_) if globs => parent,
            _ => None,
        };
        found
            .map(|ident| ident.unraw().to_string())
            .into_iter()
            .collect()
    }
}
