// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::naming::import_scope::ImportScope;
use crate::adapters::workspace::sources::naming::named::Named;
use crate::adapters::workspace::sources::naming::resolution::Resolution;
use crate::adapters::workspace::sources::naming::span_line::SpanLine;
use proc_macro2::Ident;
use proc_macro2::Spacing;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

const COLON: char = ':';
const JOINERS: [char; 2] = ['.', '$'];

/// The dependencies named in tokens the parse leaves unparsed -- a macro's
/// arguments, an attribute's --: a dependency's name starting a path,
/// `itoa::Buffer`, or a name a `use` item binds to one, at any depth of
/// brackets. A name after `::`, `.` or `$` continues something else, and a
/// glob's names are not guessed at here.
pub struct TokenNames<'a> {
    scope: &'a ImportScope<'a>,
}

impl<'a> TokenNames<'a> {
    pub fn new(scope: &'a ImportScope<'a>) -> Self {
        Self { scope }
    }

    pub fn of(&self, tokens: TokenStream) -> Vec<Named> {
        let trees = tokens.into_iter().collect::<Vec<TokenTree>>();
        trees
            .iter()
            .enumerate()
            .flat_map(|(at, tree)| self.tree(&trees, at, tree))
            .collect()
    }

    fn continues(trees: &[TokenTree], at: usize) -> bool {
        let joined = at
            .checked_sub(1)
            .and_then(|before| trees.get(before))
            .is_some_and(|before| {
                matches!(before, TokenTree::Punct(punct) if JOINERS.contains(&punct.as_char()))
            });
        joined
            || at
                .checked_sub(2)
                .is_some_and(|before| Self::separator(trees, before))
    }

    fn ident(&self, ident: &Ident, starts_path: bool) -> Option<Named> {
        match self.scope.root(&ident.to_string(), false, starts_path) {
            Resolution::Crate(package) => Some(Named {
                package,
                line: SpanLine.of(ident.span()),
            }),
            Resolution::Local | Resolution::Unknown => None,
        }
    }

    fn separator(trees: &[TokenTree], at: usize) -> bool {
        matches!(
            (trees.get(at), trees.get(at + 1)),
            (Some(TokenTree::Punct(first)), Some(TokenTree::Punct(second)))
                if first.as_char() == COLON
                    && first.spacing() == Spacing::Joint
                    && second.as_char() == COLON
        )
    }

    fn tree(&self, trees: &[TokenTree], at: usize, tree: &TokenTree) -> Vec<Named> {
        match tree {
            TokenTree::Group(group) => self.of(group.stream()),
            TokenTree::Ident(ident) if !Self::continues(trees, at) => self
                .ident(ident, Self::separator(trees, at + 1))
                .into_iter()
                .collect(),
            _ => Vec::new(),
        }
    }
}
