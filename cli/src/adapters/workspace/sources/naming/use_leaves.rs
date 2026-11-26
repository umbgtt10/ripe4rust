// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::span_line::SpanLine;
use crate::adapters::workspace::sources::naming::use_leaf::UseLeaf;
use syn::Ident;
use syn::ItemUse;
use syn::UseTree;

const SELF: &str = "self";
const UNDERSCORE: &str = "_";

/// Every name a `use` item brings in, through its groups: `use a::{b, c as
/// d, e::*}` brings `b`, `d` and a glob of `a::e`, each rooted at `a`; `self`
/// in a group binds the name before it, and `as _` binds none.
pub struct UseLeaves {
    global: bool,
}

impl UseLeaves {
    pub fn new(item: &ItemUse) -> Self {
        Self {
            global: item.leading_colon.is_some(),
        }
    }

    pub fn of(&self, tree: &UseTree) -> Vec<UseLeaf> {
        self.walk(tree, None, None)
    }

    fn leaf(&self, root: Option<&Ident>, leaf: &Ident, bound: Option<String>) -> UseLeaf {
        let root = root.unwrap_or(leaf);
        UseLeaf {
            root: root.to_string(),
            root_line: SpanLine.of(root.span()),
            global: self.global,
            bound,
            line: SpanLine.of(leaf.span()),
            glob: false,
        }
    }

    fn walk(&self, tree: &UseTree, root: Option<&Ident>, parent: Option<&Ident>) -> Vec<UseLeaf> {
        match tree {
            UseTree::Path(path) => {
                self.walk(&path.tree, root.or(Some(&path.ident)), Some(&path.ident))
            }
            UseTree::Name(name) => {
                let bound = if name.ident == SELF {
                    parent
                } else {
                    Some(&name.ident)
                };
                vec![self.leaf(root, &name.ident, bound.map(ToString::to_string))]
            }
            UseTree::Rename(rename) => {
                let bound = (rename.rename != UNDERSCORE).then(|| rename.rename.to_string());
                vec![self.leaf(root, &rename.ident, bound)]
            }
            UseTree::Glob(_) => parent
                .map(|parent| UseLeaf {
                    glob: true,
                    ..self.leaf(root, parent, None)
                })
                .into_iter()
                .collect(),
            UseTree::Group(group) => group
                .items
                .iter()
                .flat_map(|item| self.walk(item, root, parent))
                .collect(),
        }
    }
}
