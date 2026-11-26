// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::span_line::SpanLine;
use crate::adapters::workspace::sources::walking::declared_module::DeclaredModule;
use syn::Expr;
use syn::ExprLit;
use syn::Item;
use syn::ItemMod;
use syn::Lit;
use syn::Meta;
use syn::MetaNameValue;
use syn::Visibility;
use syn::ext::IdentExt;

const PATH: &str = "path";

/// The modules a file declares out of line, `mod name;`, at its top and in
/// its inline modules, each reachable through `pub mod` declarations only
/// where its file is and every module on the way is `pub`.
pub struct ModuleDeclarations;

impl ModuleDeclarations {
    pub fn of(&self, items: &[Item], inline: &[String], public: bool) -> Vec<DeclaredModule> {
        items
            .iter()
            .filter_map(|item| match item {
                Item::Mod(module) => Some(module),
                _ => None,
            })
            .flat_map(|module| {
                let public = public && matches!(module.vis, Visibility::Public(_));
                let name = module.ident.unraw().to_string();
                match &module.content {
                    Some((_, items)) => self.of(items, &[inline, &[name]].concat(), public),
                    None => vec![DeclaredModule {
                        name,
                        inline: inline.to_vec(),
                        path: self.path(module),
                        public,
                        line: SpanLine.of(module.ident.span()),
                    }],
                }
            })
            .collect()
    }

    pub fn path(&self, module: &ItemMod) -> Option<String> {
        module
            .attrs
            .iter()
            .filter(|attribute| attribute.path().is_ident(PATH))
            .find_map(|attribute| match &attribute.meta {
                Meta::NameValue(MetaNameValue {
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(text),
                            ..
                        }),
                    ..
                }) => Some(text.value()),
                _ => None,
            })
    }
}
