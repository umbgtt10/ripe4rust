// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::span_line::SpanLine;
use crate::adapters::workspace::sources::walking::included_file::IncludedFile;
use syn::ItemMacro;
use syn::ItemMod;
use syn::LitStr;
use syn::Macro;
use syn::Visibility;
use syn::ext::IdentExt;
use syn::spanned::Spanned;
use syn::visit::Visit;

const INCLUDE: &str = "include";
const GENERATED: [&str; 1] = ["include_proto"];

/// The `include!`s of a literal path among a file's items and its inline
/// modules', each with the module its items land in and whether they are
/// reachable through `pub mod` declarations: the file is, and every inline
/// module on the way is `pub`; and the lines of those it cannot follow -- an
/// `include!` whose path is no literal, or one in an expression, and a macro
/// that includes generated code, `include_proto!`.
pub struct IncludeFinder {
    public: bool,
    module: Option<String>,
    found: Vec<IncludedFile>,
    unfollowed: Vec<u32>,
}

impl IncludeFinder {
    pub fn new(public: bool, module: Option<String>) -> Self {
        Self {
            public,
            module,
            found: Vec::new(),
            unfollowed: Vec::new(),
        }
    }

    pub fn found(&self) -> Vec<IncludedFile> {
        self.found.clone()
    }

    pub fn unfollowed(&self) -> Vec<u32> {
        self.unfollowed.clone()
    }

    fn named(mac: &Macro) -> Option<String> {
        let name = mac.path.segments.last()?.ident.to_string();
        (name == INCLUDE || GENERATED.contains(&name.as_str())).then_some(name)
    }
}

impl<'ast> Visit<'ast> for IncludeFinder {
    fn visit_item_macro(&mut self, item: &'ast ItemMacro) {
        let Some(name) = Self::named(&item.mac) else {
            return;
        };
        let line = SpanLine.of(item.mac.path.span());
        match item.mac.parse_body::<LitStr>() {
            Ok(path) if name == INCLUDE => self.found.push(IncludedFile {
                path: path.value(),
                line,
                public: self.public,
                module: self.module.clone(),
            }),
            _ => self.unfollowed.push(line),
        }
    }

    fn visit_macro(&mut self, mac: &'ast Macro) {
        if Self::named(mac).is_some() {
            self.unfollowed.push(SpanLine.of(mac.path.span()));
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
