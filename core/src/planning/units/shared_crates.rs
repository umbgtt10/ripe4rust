// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::units::crate_change::CrateChange;
use alloc::string::String;
use alloc::vec::Vec;

/// The crates the units share: those whose moves, each claimant's computed
/// alone, do not add up to what `cargo update --dry-run` does for the whole
/// workspace -- a claimant leaves a version the whole workspace keeps or
/// reaches one it does not, two claimants leaving one version disagree on
/// where it goes, or the whole workspace moves a version no claimant does.
/// Applied one after another, the units move such a crate as those before
/// leave it, not as the plan computed each alone. A crate pinned with
/// `hold = "all"` is never shared: every move of it stays a difference.
pub struct SharedCrates {
    held: Vec<String>,
}

impl SharedCrates {
    pub fn new(held: Vec<String>) -> Self {
        Self { held }
    }

    pub fn of(&self, claimants: &[&[LockMove]], workspace: &[LockMove]) -> Vec<String> {
        let mut names = claimants
            .iter()
            .flat_map(|moves| moves.iter())
            .chain(workspace)
            .map(|lock_move| lock_move.name.clone())
            .collect::<Vec<String>>();
        names.sort();
        names.dedup();
        names
            .into_iter()
            .filter(|name| !self.held.contains(name))
            .filter(|name| Self::shared(name, claimants, workspace))
            .collect()
    }

    fn shared(name: &str, claimants: &[&[LockMove]], workspace: &[LockMove]) -> bool {
        let whole = CrateChange::of(name, workspace);
        let changes = claimants
            .iter()
            .map(|moves| CrateChange::of(name, moves))
            .filter(|change| !change.is_empty())
            .collect::<Vec<CrateChange>>();
        let strays = changes.iter().any(|change| !change.within(&whole));
        let joint = !whole.within(&CrateChange::union(&changes));
        let split = changes.iter().any(|change| {
            changes
                .iter()
                .any(|other| change.overlaps(other) && change != other)
        });
        strays || joint || split
    }
}
