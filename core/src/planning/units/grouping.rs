// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::locked_package::LockedPackage;
use crate::planning::choice::histories::Histories;
use crate::planning::units::crate_repository::CrateRepository;
use crate::planning::units::unit::Unit;
use alloc::vec;
use alloc::vec::Vec;

/// Crates released together move together: units of one kind whose crates
/// share a repository and released the same version on the same day, or
/// one of which requires an exact version of the other, become one unit,
/// named for its first member.
pub struct Grouping<'a> {
    repositories: &'a [CrateRepository],
    lock: &'a [LockedPackage],
    histories: &'a Histories,
}

impl<'a> Grouping<'a> {
    pub fn new(
        repositories: &'a [CrateRepository],
        lock: &'a [LockedPackage],
        histories: &'a Histories,
    ) -> Self {
        Self {
            repositories,
            lock,
            histories,
        }
    }

    pub fn group(&self, units: Vec<Unit>) -> Vec<Unit> {
        let mut groups: Vec<Vec<Unit>> = Vec::new();
        for unit in units {
            match groups
                .iter()
                .position(|group| group.iter().any(|member| self.related(member, &unit)))
            {
                Some(index) => groups[index].push(unit),
                None => groups.push(vec![unit]),
            }
        }
        groups
            .into_iter()
            .filter_map(|group| {
                group.into_iter().reduce(|mut first, other| {
                    first.members.extend(other.members);
                    first.moves.extend(other.moves);
                    first.held.extend(other.held);
                    first
                })
            })
            .collect()
    }

    pub fn related(&self, left: &Unit, right: &Unit) -> bool {
        left.kind == right.kind
            && (self.released_together(left, right)
                || self.requires_exactly(left, right)
                || self.requires_exactly(right, left))
    }

    fn released_together(&self, left: &Unit, right: &Unit) -> bool {
        let (Some(left_name), Some(right_name), Some(version)) =
            (&left.name, &right.name, &left.to)
        else {
            return false;
        };
        let repository = |name: &str| {
            self.repositories
                .iter()
                .find(|known| known.name == name)
                .map(|known| &known.repository)
        };
        let published = self.histories.published_on(left_name, version);
        repository(left_name).is_some()
            && repository(left_name) == repository(right_name)
            && right.to.as_ref() == Some(version)
            && published.is_some()
            && published == self.histories.published_on(right_name, version)
    }

    fn requires_exactly(&self, dependent: &Unit, dependency: &Unit) -> bool {
        self.lock.iter().any(|package| {
            dependent.name.as_ref() == Some(&package.name)
                && package.dependencies.iter().any(|required| {
                    dependency.name.as_ref() == Some(&required.name)
                        && required.requirement.exact().is_some()
                })
        })
    }
}
