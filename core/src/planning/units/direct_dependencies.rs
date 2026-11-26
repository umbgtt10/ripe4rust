// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::workspace::Workspace;
use crate::planning::units::direct_dependency::DirectDependency;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// The registry crates the members depend on directly, by name; a crate
/// the members use at several versions is judged from the oldest.
pub struct DirectDependencies;

impl DirectDependencies {
    pub fn of(&self, workspace: &Workspace) -> Vec<DirectDependency> {
        let mut found: BTreeMap<String, DirectDependency> = BTreeMap::new();
        let used = workspace.members.iter().flat_map(|member| {
            member
                .dependencies
                .iter()
                .map(move |dependency| (member, dependency))
        });
        for (member, dependency) in used.filter(|(_, dependency)| {
            workspace.lock.iter().any(|package| {
                package.name == dependency.name && package.version == dependency.version
            })
        }) {
            let entry = found
                .entry(dependency.name.clone())
                .or_insert_with(|| DirectDependency {
                    name: dependency.name.clone(),
                    version: dependency.version.clone(),
                    members: Vec::new(),
                });
            entry.version = entry.version.clone().min(dependency.version.clone());
            if !entry.members.contains(&member.name) {
                entry.members.push(member.name.clone());
            }
        }
        found.into_values().collect()
    }
}
