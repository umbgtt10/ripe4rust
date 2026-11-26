// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::unreadable::Unreadable;
use crate::system::model::layout::Layout;
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
}

/// What `cargo metadata --no-deps` printed, read as the workspace's layout:
/// it resolves nothing, so it needs no network and moves no lock.
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
                .into_iter()
                .map(|member| (member.name, PathBuf::from(member.manifest_path)))
                .collect(),
        })
    }
}
