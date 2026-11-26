// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::toml_file::TomlFile;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::written_line::WrittenLine;
use ripe4rust_core::values::location::Location;
use toml::Spanned;
use toml::de::DeTable;
use toml::de::DeValue;

const KINDS: [&str; 5] = [
    "dependencies",
    "dev-dependencies",
    "build-dependencies",
    "dev_dependencies",
    "build_dependencies",
];

/// Every line of a manifest that states a crate's requirement: in each kind
/// of dependency table, for every target, and in `[workspace.dependencies]`;
/// the line of the requirement itself -- `crate = "1.4"`, or a table's
/// `version` --, a crate renamed found by its `package`. A dependency with no
/// version of its own -- a path, a git source, `workspace = true` -- states
/// none.
pub struct RequirementFinder;

impl RequirementFinder {
    pub fn lines(&self, file: &str, text: &str, name: &str) -> Result<Vec<Location>, PortFailure> {
        Ok(self
            .written(file, text)?
            .into_iter()
            .filter(|line| line.key == name)
            .map(|line| line.at)
            .collect())
    }

    pub fn written(&self, file: &str, text: &str) -> Result<Vec<WrittenLine>, PortFailure> {
        let toml = TomlFile::new(file, text);
        let document = toml.table()?;
        let root = document.get_ref();
        let workspace = root
            .get("workspace")
            .and_then(|workspace| workspace.get_ref().get("dependencies"))
            .and_then(|table| table.get_ref().as_table());
        let targets = root
            .get("target")
            .and_then(|targets| targets.get_ref().as_table())
            .into_iter()
            .flat_map(|targets| targets.values())
            .filter_map(|target| target.get_ref().as_table());
        let tables = [root]
            .into_iter()
            .chain(targets)
            .flat_map(|table| KINDS.iter().filter_map(|kind| table.get(*kind)))
            .filter_map(|table| table.get_ref().as_table())
            .chain(workspace)
            .collect::<Vec<&DeTable<'_>>>();
        let mut lines = tables
            .iter()
            .flat_map(|table| table.iter())
            .filter_map(|(key, value)| Self::requirement(&toml, file, key.get_ref(), value))
            .collect::<Vec<WrittenLine>>();
        lines.sort_by(|left, right| left.at.cmp(&right.at));
        Ok(lines)
    }

    fn requirement(
        toml: &TomlFile<'_>,
        file: &str,
        key: &str,
        value: &Spanned<DeValue<'_>>,
    ) -> Option<WrittenLine> {
        let (name, version) = match value.get_ref() {
            DeValue::String(_) => (key, value),
            DeValue::Table(dependency) => (
                dependency
                    .get("package")
                    .and_then(|package| package.get_ref().as_str())
                    .unwrap_or(key),
                dependency.get("version")?,
            ),
            _ => return None,
        };
        Some(WrittenLine {
            at: Location {
                file: String::from(file),
                line: toml.line(version.span().start),
            },
            key: String::from(name),
            value: String::from(version.get_ref().as_str()?),
        })
    }
}
