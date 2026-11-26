// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::span_line::SpanLine;
use crate::adapters::workspace::sources::walking::included_file::IncludedFile;
use syn::ItemMacro;
use syn::ItemMod;
use syn::LitStr;
use syn::Visibility;
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::visit::Visit;

const INCLUDE: &str = "include";

/// The `include!`s of a literal path among a file's items and its inline
/// modules', each with the module its items land in and whether they are
/// reachable through `pub mod` declarations: the file is, and every inline
/// module on the way is `pub`.
pub struct IncludeFinder {
    public: bool,
    module: Option<String>,
    found: Vec<IncludedFile>,
}

impl IncludeFinder {
    pub fn new(public: bool, module: Option<String>) -> Self {
        Self {
            public,
            module,
            found: Vec::new(),
        }
    }

    pub fn found(&self) -> Vec<IncludedFile> {
        self.found.clone()
    }
}

impl<'ast> Visit<'ast> for IncludeFinder {
    fn visit_item_macro(&mut self, item: &'ast ItemMacro) {
        let named = item
            .mac
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == INCLUDE);
        if let (true, Ok(path)) = (named, item.mac.parse_body::<LitStr>()) {
            self.found.push(IncludedFile {
                path: path.value(),
                line: SpanLine.of(item.mac.path.span()),
                public: self.public,
                module: self.module.clone(),
            });
        }
    }

    fn visit_item_mod(&mut self, item: &'ast ItemMod) {
        let Some((_, items)) = &item.content else {
            return;
        };
        let outer = (self.public, self.module.clone());
        self.public = self.public && matches!(item.vis, Visibility::Public(_));
        self.module = Some(item.ident.unraw().to_string());
        for inner in items {
            self.visit_item(inner);
        }
        (self.public, self.module) = outer;
    }
}
