// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::fingerprint::Fingerprint;
use crate::adapters::workspace::policy_section::PolicySection;
use crate::adapters::workspace::requirement_finder::RequirementFinder;
use crate::adapters::workspace::rust_version_finder::RustVersionFinder;
use crate::adapters::workspace::toolchain_reader::ToolchainReader;
use crate::system::model::file_failure::FileFailure;
use crate::system::model::layout::Layout;
use crate::system::traits::file_system::FileSystem;
use crate::system::traits::layout_source::LayoutSource;
use ripe4rust_core::inputs::policy_tree::PolicyTree;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::toolchain_file::ToolchainFile;
use ripe4rust_core::traits::workspace_files::WorkspaceFiles;
use ripe4rust_core::values::location::Location;
use std::path::Path;
use std::path::PathBuf;

const LOCK: &str = "Cargo.lock";
const MANIFEST: &str = "Cargo.toml";
const TOOLCHAIN: &str = "rust-toolchain.toml";

/// The workspace's own files, found where its layout says they are, each
/// named from the workspace's root with `/` on every system.
pub struct ManifestFiles<'a> {
    source: &'a dyn LayoutSource,
    files: &'a dyn FileSystem,
}

impl<'a> ManifestFiles<'a> {
    pub fn new(source: &'a dyn LayoutSource, files: &'a dyn FileSystem) -> Self {
        Self { source, files }
    }

    fn manifests(layout: &Layout) -> Vec<PathBuf> {
        let mut manifests = vec![layout.root.join(MANIFEST)];
        for (_, manifest) in &layout.members {
            if !manifests.contains(manifest) {
                manifests.push(manifest.clone());
            }
        }
        manifests
    }

    fn named(layout: &Layout, path: &Path) -> String {
        path.strip_prefix(&layout.root)
            .unwrap_or(path)
            .components()
            .map(|part| part.as_os_str().to_string_lossy().into_owned())
            .collect::<Vec<String>>()
            .join("/")
    }

    fn read(&self, layout: &Layout, path: &Path) -> Result<Option<String>, PortFailure> {
        match self.files.read(path) {
            Ok(text) => Ok(Some(text)),
            Err(FileFailure::Missing) => Ok(None),
            Err(FileFailure::Failed { detail }) => Err(PortFailure::WorkspaceUnreadable {
                file: Self::named(layout, path),
                detail,
            }),
        }
    }

    fn required(&self, layout: &Layout, path: &Path) -> Result<String, PortFailure> {
        self.read(layout, path)?
            .ok_or_else(|| PortFailure::WorkspaceUnreadable {
                file: Self::named(layout, path),
                detail: String::from("it is not there"),
            })
    }
}

impl WorkspaceFiles for ManifestFiles<'_> {
    fn fingerprint(&self) -> Result<String, PortFailure> {
        let layout = self.source.layout()?;
        let mut paths = vec![layout.root.join(LOCK), layout.root.join(TOOLCHAIN)];
        paths.extend(Self::manifests(&layout));
        let files = paths
            .iter()
            .map(|path| Ok((Self::named(&layout, path), self.read(&layout, path)?)))
            .collect::<Result<Vec<(String, Option<String>)>, PortFailure>>()?;
        Ok(Fingerprint.of(&files))
    }

    fn policy(&self) -> Result<PolicyTree, PortFailure> {
        let layout = self.source.layout()?;
        PolicySection.tree(&self.required(&layout, &layout.root.join(MANIFEST))?)
    }

    fn requirement_lines(&self, name: &str) -> Result<Vec<Location>, PortFailure> {
        let layout = self.source.layout()?;
        let mut lines = Vec::new();
        for manifest in Self::manifests(&layout) {
            let text = self.required(&layout, &manifest)?;
            lines.extend(RequirementFinder.lines(&Self::named(&layout, &manifest), &text, name)?);
        }
        lines.sort();
        Ok(lines)
    }

    fn rust_version_line(&self, member: &str) -> Result<Option<Location>, PortFailure> {
        let layout = self.source.layout()?;
        let Some((_, manifest)) = layout.members.iter().find(|(name, _)| name == member) else {
            return Ok(None);
        };
        let root = self.required(&layout, &layout.root.join(MANIFEST))?;
        let text = self.required(&layout, manifest)?;
        RustVersionFinder.line(&Self::named(&layout, manifest), &text, &root)
    }

    fn toolchain(&self) -> Result<Option<ToolchainFile>, PortFailure> {
        let layout = self.source.layout()?;
        self.read(&layout, &layout.root.join(TOOLCHAIN))?
            .map(|text| ToolchainReader.file(&text))
            .transpose()
    }
}
