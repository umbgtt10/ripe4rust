// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::unreadable::Unreadable;
use crate::system::model::declared_dependency::DeclaredDependency;
use crate::system::model::layout::Layout;
use crate::system::model::member_sources::MemberSources;
use crate::system::model::source_target::SourceTarget;
use ripe4rust_core::inputs::port_failure::PortFailure;
use serde::Deserialize;
use serde_json::from_str;
use std::path::PathBuf;

#[derive(Deserialize)]
struct NoDepsJson {
    pub workspace_root: String,
    pub packages: Vec<MemberJson>,
}

#[derive(Deserialize)]
struct MemberJson {
    pub name: String,
    pub manifest_path: String,
    pub targets: Vec<SourceTargetJson>,
    pub dependencies: Vec<DeclaredJson>,
}

#[derive(Deserialize)]
struct SourceTargetJson {
    pub kind: Vec<String>,
    pub src_path: String,
}

#[derive(Deserialize)]
struct DeclaredJson {
    pub name: String,
    pub rename: Option<String>,
    pub source: Option<String>,
}

/// What `cargo metadata --no-deps` printed, read as the workspace's layout:
/// it resolves nothing, so it needs no network and moves no lock. Each
/// member's targets and declared dependencies are what the parse reads.
pub struct LayoutReader;

impl LayoutReader {
    pub fn layout(&self, text: &str) -> Result<Layout, PortFailure> {
        let metadata = from_str::<NoDepsJson>(text).map_err(|error| {
            Unreadable::new("reading what `cargo metadata --no-deps` printed")
                .failure(&error.to_string())
        })?;
        Ok(Layout {
            root: PathBuf::from(metadata.workspace_root),
            members: metadata
                .packages
                .iter()
                .map(|member| (member.name.clone(), PathBuf::from(&member.manifest_path)))
                .collect(),
            sources: metadata.packages.into_iter().map(Self::sources).collect(),
        })
    }

    fn sources(member: MemberJson) -> MemberSources {
        MemberSources {
            name: member.name,
            targets: member
                .targets
                .into_iter()
                .map(|target| SourceTarget {
                    kinds: target.kind,
                    root: PathBuf::from(target.src_path),
                })
                .collect(),
            dependencies: member
                .dependencies
                .into_iter()
                .map(|dependency| DeclaredDependency {
                    name: dependency.name,
                    rename: dependency.rename,
                    source: dependency.source,
                })
                .collect(),
        }
    }
}
