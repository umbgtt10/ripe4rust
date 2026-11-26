// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::toml_file::TomlFile;
use crate::adapters::workspace::toml_values::TomlValues;
use ripe4rust_core::inputs::policy_tree::PolicyTree;
use ripe4rust_core::inputs::port_failure::PortFailure;
use toml::de::DeValue;

const MANIFEST: &str = "Cargo.toml";
const PATH: [&str; 2] = ["workspace", "metadata"];
const SECTION: &str = "ripe4rust";

/// `[workspace.metadata.ripe4rust]` of the workspace's manifest, as the
/// raw tree the core reads, its line the line of its `ripe4rust` key; none
/// where the manifest has no such table.
pub struct PolicySection;

impl PolicySection {
    pub fn tree(&self, text: &str) -> Result<PolicyTree, PortFailure> {
        let toml = TomlFile::new(MANIFEST, text);
        let document = toml.table()?;
        let metadata = PATH.iter().try_fold(document.get_ref(), |table, key| {
            table.get(*key).and_then(|value| value.get_ref().as_table())
        });
        let section =
            metadata.and_then(|metadata| metadata.iter().find(|(key, _)| key.get_ref() == SECTION));
        let Some((key, value)) = section else {
            return Ok(PolicyTree {
                at: None,
                entries: Vec::new(),
            });
        };
        let values = TomlValues::new(MANIFEST, text);
        let at = values.at(key.span().start);
        match value.get_ref() {
            DeValue::Table(table) => Ok(PolicyTree {
                at: Some(at),
                entries: values.entries(table),
            }),
            _ => Err(PortFailure::WorkspaceUnreadable {
                file: String::from(MANIFEST),
                detail: format!(
                    "line {}: `[workspace.metadata.ripe4rust]` is not a table",
                    toml.line(key.span().start)
                ),
            }),
        }
    }
}
