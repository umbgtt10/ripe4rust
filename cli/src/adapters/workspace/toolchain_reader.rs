// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::toml_file::TomlFile;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::toolchain_file::ToolchainFile;
use ripe4rust_core::values::location::Location;

const FILE: &str = "rust-toolchain.toml";

/// `rust-toolchain.toml`'s channel and its line; a file naming no channel
/// has an empty one, at its `[toolchain]` table or its first line.
pub struct ToolchainReader;

impl ToolchainReader {
    pub fn file(&self, text: &str) -> Result<ToolchainFile, PortFailure> {
        let toml = TomlFile::new(FILE, text);
        let document = toml.table()?;
        let toolchain = document
            .get_ref()
            .iter()
            .find(|(key, _)| key.get_ref() == "toolchain");
        let channel = toolchain.and_then(|(_, table)| table.get_ref().get("channel"));
        let (channel, offset) = match (channel, toolchain) {
            (Some(channel), _) => (
                channel.get_ref().as_str().unwrap_or_default(),
                channel.span().start,
            ),
            (None, Some((key, _))) => ("", key.span().start),
            (None, None) => ("", 0),
        };
        Ok(ToolchainFile {
            channel: String::from(channel),
            at: Location {
                file: String::from(FILE),
                line: toml.line(offset),
            },
        })
    }
}
