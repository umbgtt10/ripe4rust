// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::inputs::lock_edge::LockEdge;
use crate::planning::round::lock_changes::LockChanges;
use crate::planning::round::move_wording::MoveWording;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;

/// The lock's versioned edges as the plan found them and as they are now,
/// beside its packages: every edge re-pointed between versions both locks
/// hold -- its dependent at one version in both, its dependency moved from
/// one version to another, each held by the plan's lock and by the lock now
/// --, which no version move explains. An edge whose old version left the
/// lock or whose new one joined it is that move's. A plan written before
/// the edges were recorded has none to compare, which is said once.
pub struct EdgeChanges {
    lock: LockChanges,
    base: Option<Vec<LockEdge>>,
    current: Vec<LockEdge>,
}

impl EdgeChanges {
    pub fn new(lock: LockChanges, base: Option<&[LockEdge]>, current: &[LockEdge]) -> Self {
        Self {
            lock,
            base: base.map(<[LockEdge]>::to_vec),
            current: current.to_vec(),
        }
    }

    pub fn repointed(&self) -> Vec<String> {
        let base = self.base.as_deref().unwrap_or_default();
        let mut keys = base.iter().collect::<Vec<&LockEdge>>();
        keys.sort();
        keys.dedup_by(|edge, kept| Self::same_key(edge, kept));
        keys.into_iter()
            .flat_map(|key| self.moved(base, key))
            .collect()
    }

    pub fn unseen(&self) -> Option<Diagnostic> {
        self.base.is_none().then(|| {
            DiagnosticCode::PlanWithoutEdges.diagnose(
                None,
                String::from(
                    "the plan records no edges of the lock -- ripe4rust 0.1.2 and earlier wrote none --, so a dependency re-pointed between versions the lock already held cannot be seen",
                ),
                String::from(
                    "nothing stops the check; to see re-points, plan again once this unit is committed or reverted -- `cargo ripe4rust plan --format json > plan.json` -- and go on with the new plan",
                ),
            )
        })
    }

    fn moved(&self, base: &[LockEdge], key: &LockEdge) -> Vec<String> {
        let (was, now) = (
            Self::versions(base, key),
            Self::versions(&self.current, key),
        );
        let left = was.iter().filter(|version| !now.contains(version));
        let arrived = now.iter().filter(|version| !was.contains(version));
        left.zip(arrived)
            .filter(|(from, to)| {
                self.lock.kept(&key.dependency, from) && self.lock.kept(&key.dependency, to)
            })
            .map(|(from, to)| {
                let edge = LockEdge {
                    dependency_version: from.clone(),
                    ..key.clone()
                };
                MoveWording.repointed(&edge, to)
            })
            .collect()
    }

    fn same_key(edge: &LockEdge, other: &LockEdge) -> bool {
        edge.dependent == other.dependent
            && edge.dependent_version == other.dependent_version
            && edge.dependency == other.dependency
    }

    fn versions(edges: &[LockEdge], key: &LockEdge) -> Vec<Version> {
        let mut versions = edges
            .iter()
            .filter(|edge| Self::same_key(edge, key))
            .map(|edge| edge.dependency_version.clone())
            .collect::<Vec<Version>>();
        versions.sort();
        versions
    }
}
