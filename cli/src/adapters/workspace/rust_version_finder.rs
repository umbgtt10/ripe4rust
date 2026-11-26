// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::toml_file::TomlFile;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::location::Location;
use toml::de::DeValue;

const MANIFEST: &str = "Cargo.toml";
const KEY: &str = "rust-version";

/// The line of a member's `rust-version`: its own, or, where it inherits
/// it with `workspace = true`, the workspace's in `[workspace.package]`,
/// which is where raising it raises it; none where it declares none.
pub struct RustVersionFinder;

impl RustVersionFinder {
    pub fn line(
        &self,
        file: &str,
        member: &str,
        root: &str,
    ) -> Result<Option<Location>, PortFailure> {
        let toml = TomlFile::new(file, member);
        let document = toml.table()?;
        let declared = document
            .get_ref()
            .get("package")
            .and_then(|package| package.get_ref().get(KEY));
        let Some(declared) = declared else {
            return Ok(None);
        };
        let inherited = declared
            .get_ref()
            .get("workspace")
            .and_then(|workspace| workspace.get_ref().as_bool())
            .unwrap_or(false);
        match declared.get_ref() {
            DeValue::String(_) => Ok(Some(Location {
                file: String::from(file),
                line: toml.line(declared.span().start),
            })),
            DeValue::Table(_) if inherited => {
                let workspace = TomlFile::new(MANIFEST, root);
                let document = workspace.table()?;
                Ok(document
                    .get_ref()
                    .get("workspace")
                    .and_then(|table| table.get_ref().get("package"))
                    .and_then(|package| package.get_ref().get(KEY))
                    .map(|version| Location {
                        file: String::from(MANIFEST),
                        line: workspace.line(version.span().start),
                    }))
            }
            _ => Ok(None),
        }
    }
}
