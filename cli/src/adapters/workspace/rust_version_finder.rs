// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::toml_file::TomlFile;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::written_line::WrittenLine;
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
        Ok(self.written(file, member, root)?.map(|written| written.at))
    }

    pub fn written(
        &self,
        file: &str,
        member: &str,
        root: &str,
    ) -> Result<Option<WrittenLine>, PortFailure> {
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
            DeValue::String(value) => Ok(Some(Self::line_of(
                file,
                &toml,
                declared.span().start,
                value,
            ))),
            DeValue::Table(_) if inherited => {
                let workspace = TomlFile::new(MANIFEST, root);
                let document = workspace.table()?;
                Ok(document
                    .get_ref()
                    .get("workspace")
                    .and_then(|table| table.get_ref().get("package"))
                    .and_then(|package| package.get_ref().get(KEY))
                    .map(|version| {
                        Self::line_of(
                            MANIFEST,
                            &workspace,
                            version.span().start,
                            version.get_ref().as_str().unwrap_or_default(),
                        )
                    }))
            }
            _ => Ok(None),
        }
    }

    fn line_of(file: &str, toml: &TomlFile<'_>, offset: usize, value: &str) -> WrittenLine {
        WrittenLine {
            at: Location {
                file: String::from(file),
                line: toml.line(offset),
            },
            key: String::from(KEY),
            value: String::from(value),
        }
    }
}
