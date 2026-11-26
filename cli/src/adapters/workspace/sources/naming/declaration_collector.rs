// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::declarations::Declarations;
use crate::adapters::workspace::sources::naming::extern_declaration::ExternDeclaration;
use crate::adapters::workspace::sources::naming::item_name::ItemName;
use crate::adapters::workspace::sources::naming::use_leaves::UseLeaves;
use syn::File;
use syn::GenericParam;
use syn::Ident;
use syn::Item;
use syn::ItemExternCrate;
use syn::ItemMacro;
use syn::ItemUse;
use syn::Variant;
use syn::visit::Visit;
use syn::visit::visit_generic_param;
use syn::visit::visit_item;
use syn::visit::visit_item_macro;
use syn::visit::visit_variant;

const MACRO_USE: &str = "macro_use";

/// A file's declarations, at every depth -- inline modules, function bodies
/// --: a name declared anywhere in the file is taken as in scope throughout
/// it.
pub struct DeclarationCollector {
    declarations: Declarations,
}

impl DeclarationCollector {
    pub fn of(file: &File) -> Declarations {
        let mut collector = Self {
            declarations: Declarations::default(),
        };
        collector.visit_file(file);
        collector.declarations
    }

    fn local(&mut self, ident: &Ident) {
        self.declarations.locals.push(ident.to_string());
    }
}

impl<'ast> Visit<'ast> for DeclarationCollector {
    fn visit_generic_param(&mut self, node: &'ast GenericParam) {
        match node {
            GenericParam::Type(param) => self.local(&param.ident),
            GenericParam::Const(param) => self.local(&param.ident),
            GenericParam::Lifetime(_) => {}
        }
        visit_generic_param(self, node);
    }

    fn visit_item(&mut self, node: &'ast Item) {
        if let Some(ident) = ItemName.of(node) {
            self.local(ident);
        }
        visit_item(self, node);
    }

    fn visit_item_extern_crate(&mut self, node: &'ast ItemExternCrate) {
        self.declarations.externs.push(ExternDeclaration {
            name: node.ident.to_string(),
            alias: node.rename.as_ref().map(|(_, alias)| alias.to_string()),
            macro_use: node
                .attrs
                .iter()
                .any(|attribute| attribute.path().is_ident(MACRO_USE)),
        });
    }

    fn visit_item_macro(&mut self, node: &'ast ItemMacro) {
        if let Some(ident) = &node.ident {
            self.declarations.macros.push(ident.to_string());
        }
        visit_item_macro(self, node);
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        self.declarations
            .uses
            .extend(UseLeaves::new(node).of(&node.tree));
    }

    fn visit_variant(&mut self, node: &'ast Variant) {
        self.local(&node.ident);
        visit_variant(self, node);
    }
}
