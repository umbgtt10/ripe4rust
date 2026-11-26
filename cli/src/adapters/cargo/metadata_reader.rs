// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::crates_io::CratesIo;
use crate::adapters::unreadable::Unreadable;
use ripe4rust_core::inputs::locked_package::LockedPackage;
use ripe4rust_core::inputs::member::Member;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::resolved_dependency::ResolvedDependency;
use ripe4rust_core::inputs::workspace::Workspace;
use ripe4rust_core::values::partial_version::PartialVersion;
use ripe4rust_core::values::requirement::Requirement;
use ripe4rust_core::values::version::Version;
use serde::Deserialize;
use serde_json::Error;
use serde_json::from_str;
use std::collections::BTreeMap;

const DOING: &str = "reading what `cargo metadata` printed";
const ANY: &str = "*";

#[derive(Deserialize)]
struct MetadataJson {
    pub packages: Vec<PackageJson>,
    pub workspace_members: Vec<String>,
    pub resolve: Option<ResolveJson>,
}

#[derive(Deserialize)]
struct PackageJson {
    pub name: String,
    pub version: String,
    pub id: String,
    pub source: Option<String>,
    pub rust_version: Option<String>,
    pub dependencies: Vec<DependencyJson>,
}

#[derive(Deserialize)]
struct DependencyJson {
    pub name: String,
    pub req: String,
    pub source: Option<String>,
}

#[derive(Deserialize)]
struct ResolveJson {
    pub nodes: Vec<NodeJson>,
}

#[derive(Deserialize)]
struct NodeJson {
    pub id: String,
    pub deps: Vec<NodeDependencyJson>,
}

#[derive(Deserialize)]
struct NodeDependencyJson {
    pub pkg: String,
}

/// What `cargo metadata` printed, read as the core's workspace: the members,
/// each with the registry crates it depends on, and the registry packages
/// of the resolve, each with the requirement that asked for it. Only
/// crates.io is in scope: path and git packages are left out.
pub struct MetadataReader;

impl MetadataReader {
    pub fn registry_crates(&self, text: &str) -> Result<Vec<String>, PortFailure> {
        let metadata = from_str::<MetadataJson>(text).map_err(|error| Self::unreadable(&error))?;
        let mut names = metadata
            .resolve
            .iter()
            .flat_map(|resolve| resolve.nodes.iter())
            .filter_map(|node| {
                metadata
                    .packages
                    .iter()
                    .find(|package| package.id == node.id)
            })
            .filter(|package| CratesIo.holds(package.source.as_deref()))
            .map(|package| package.name.clone())
            .collect::<Vec<String>>();
        names.sort();
        names.dedup();
        Ok(names)
    }

    pub fn workspace(&self, text: &str) -> Result<Workspace, PortFailure> {
        let unreadable = Unreadable::new(DOING);
        let metadata = from_str::<MetadataJson>(text).map_err(|error| Self::unreadable(&error))?;
        let packages = metadata
            .packages
            .iter()
            .map(|package| (package.id.as_str(), package))
            .collect::<BTreeMap<&str, &PackageJson>>();
        let nodes = metadata
            .resolve
            .as_ref()
            .ok_or_else(|| unreadable.failure("it holds no resolve"))?
            .nodes
            .iter()
            .map(|node| (node.id.as_str(), node))
            .collect::<BTreeMap<&str, &NodeJson>>();
        let mut members = Vec::new();
        for id in &metadata.workspace_members {
            let (package, node) = Self::found(&packages, &nodes, id)?;
            members.push(Member {
                name: package.name.clone(),
                rust_version: package
                    .rust_version
                    .as_deref()
                    .map(PartialVersion::parse)
                    .transpose()
                    .map_err(|failure| unreadable.failure(&failure.text))?,
                dependencies: Self::resolved(package, node, &packages)?,
            });
        }
        let mut lock = Vec::new();
        for (id, node) in &nodes {
            let (package, _) = Self::found(&packages, &nodes, id)?;
            if CratesIo.holds(package.source.as_deref()) {
                lock.push(LockedPackage {
                    name: package.name.clone(),
                    version: Version::parse(&package.version).map_err(|failure| {
                        unreadable.failure(&format!("`{}` is not a version", failure.text))
                    })?,
                    dependencies: Self::resolved(package, node, &packages)?,
                });
            }
        }
        lock.sort_by(|left, right| (&left.name, &left.version).cmp(&(&right.name, &right.version)));
        Ok(Workspace { members, lock })
    }

    fn found<'m>(
        packages: &BTreeMap<&str, &'m PackageJson>,
        nodes: &BTreeMap<&str, &'m NodeJson>,
        id: &str,
    ) -> Result<(&'m PackageJson, &'m NodeJson), PortFailure> {
        packages
            .get(id)
            .copied()
            .zip(nodes.get(id).copied())
            .ok_or_else(|| Unreadable::new(DOING).failure(&format!("`{id}` is not in the resolve")))
    }

    fn resolved(
        package: &PackageJson,
        node: &NodeJson,
        packages: &BTreeMap<&str, &PackageJson>,
    ) -> Result<Vec<ResolvedDependency>, PortFailure> {
        let unreadable = Unreadable::new(DOING);
        let mut resolved = Vec::new();
        for dependency in node
            .deps
            .iter()
            .filter_map(|dependency| packages.get(dependency.pkg.as_str()))
            .filter(|dependency| CratesIo.holds(dependency.source.as_deref()))
        {
            let version = Version::parse(&dependency.version).map_err(|failure| {
                unreadable.failure(&format!("`{}` is not a version", failure.text))
            })?;
            let declared = package
                .dependencies
                .iter()
                .filter(|declared| {
                    declared.name == dependency.name && CratesIo.holds(declared.source.as_deref())
                })
                .filter_map(|declared| Requirement::parse(&declared.req).ok())
                .find(|requirement| requirement.matches(&version));
            let requirement = match declared {
                Some(requirement) => requirement,
                None => {
                    Requirement::parse(ANY).map_err(|failure| unreadable.failure(&failure.text))?
                }
            };
            resolved.push(ResolvedDependency {
                name: dependency.name.clone(),
                version,
                requirement,
            });
        }
        Ok(resolved)
    }

    fn unreadable(error: &Error) -> PortFailure {
        Unreadable::new(DOING).failure(&error.to_string())
    }
}
