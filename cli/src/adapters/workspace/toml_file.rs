// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::port_failure::PortFailure;
use toml::Spanned;
use toml::de::DeTable;

/// One TOML file of the workspace: its table, every value with its span,
/// and the line a span starts on. One that does not parse is unreadable,
/// at the line where it breaks.
pub struct TomlFile<'t> {
    file: String,
    text: &'t str,
}

impl<'t> TomlFile<'t> {
    pub fn new(file: &str, text: &'t str) -> Self {
        Self {
            file: String::from(file),
            text,
        }
    }

    pub fn line(&self, offset: usize) -> u32 {
        let before = self.text.get(..offset).unwrap_or(self.text);
        u32::try_from(before.matches('\n').count() + 1).unwrap_or(u32::MAX)
    }

    pub fn table(&self) -> Result<Spanned<DeTable<'t>>, PortFailure> {
        DeTable::parse(self.text).map_err(|error| PortFailure::WorkspaceUnreadable {
            file: self.file.clone(),
            detail: format!(
                "line {}: {}",
                self.line(error.span().map_or(0, |span| span.start)),
                error.message().trim()
            ),
        })
    }
}
