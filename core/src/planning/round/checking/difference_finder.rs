// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::git::git_source::GitSource;
use crate::inputs::lock_move::LockMove;
use crate::inputs::written_line::WrittenLine;
use crate::planning::pins_and_policy::hold::Hold;
use crate::planning::pins_and_policy::pin::Pin;
use crate::planning::round::checking::accountable::Accountable;
use crate::planning::round::checking::difference::Difference;
use crate::planning::round::move_wording::MoveWording;
use crate::planning::round::sorted::Sorted;
use crate::planning::round::sorting::Sorting;
use crate::planning::round::target::Target;
use crate::unit_kind::UnitKind;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

const LOCK: &str = "Cargo.lock";

/// How the unit checked differs from its plan: a move it should make that the
/// lock does not show, an edit not written, a crate its edit took past the
/// line its pin holds, a move no unit of the plan makes, a git unit's source
/// not at the rev it takes, a git source moved where no unit takes it. The lock's changes no
/// move accounts for are a breaking unit's tail where one is applied -- the
/// unit checked's, reported as its `transitive` moves, or an earlier one's --
/// and the unit checked's extra moves where none is. Of a crate the units
/// share, whose moves depend on the order they are applied in, a move it does
/// not own and a change no move explains are reported as `shared`, never a
/// difference -- unless the crate is its own, or its split's.
pub struct DifferenceFinder<'a> {
    target: &'a Target,
    sorting: &'a Sorting<'a>,
    pins: &'a [Pin],
    accountable: &'a Accountable<'a>,
}

impl<'a> DifferenceFinder<'a> {
    pub fn new(
        target: &'a Target,
        sorting: &'a Sorting<'a>,
        pins: &'a [Pin],
        accountable: &'a Accountable<'a>,
    ) -> Self {
        Self {
            target,
            sorting,
            pins,
            accountable,
        }
    }

    pub fn differences(&self, sorted: &Sorted) -> Vec<Difference> {
        let mut found = self
            .accountable
            .unmade()
            .into_iter()
            .filter(|lock_move| self.accountable.owns(lock_move))
            .map(|lock_move| self.difference(String::from(LOCK), self.missed(lock_move, sorted)))
            .collect::<Vec<Difference>>();
        found.extend(
            self.target
                .edits
                .iter()
                .filter(|edit| !self.sorting.lines().made(edit))
                .map(|edit| self.unedited(edit)),
        );
        found.extend(
            self.transitive(sorted)
                .iter()
                .filter_map(|lock_move| self.pinned(lock_move))
                .map(|what| self.difference(String::from(LOCK), what)),
        );
        found.extend(
            self.extra(sorted)
                .iter()
                .filter(|lock_move| self.accountable.answers_for(&lock_move.name))
                .map(|lock_move| {
                    self.difference(String::from(LOCK), MoveWording.unexplained(lock_move))
                }),
        );
        found.extend(
            self.target
                .unit
                .git
                .iter()
                .filter(|source| !self.sorting.git().applied(source))
                .map(|source| self.difference(String::from(LOCK), self.unmoved(source))),
        );
        found.extend(
            sorted
                .unexplained_git
                .iter()
                .filter(|moved| {
                    self.target
                        .unit
                        .git
                        .as_ref()
                        .is_none_or(|own| own.source != moved.source)
                })
                .map(|moved| self.difference(String::from(LOCK), MoveWording.git(moved))),
        );
        found
    }

    pub fn shared(&self, sorted: &Sorted) -> Vec<String> {
        let missed = self
            .accountable
            .unmade()
            .into_iter()
            .filter(|lock_move| !self.accountable.owns(lock_move))
            .map(|lock_move| MoveWording.why_shared(&self.missed(lock_move, sorted)));
        let changed = self
            .extra(sorted)
            .into_iter()
            .filter(|lock_move| !self.accountable.answers_for(&lock_move.name))
            .map(|lock_move| MoveWording.shared(&lock_move));
        missed.chain(changed).collect()
    }

    pub fn transitive(&self, sorted: &Sorted) -> Vec<LockMove> {
        if self.target.unit.kind == UnitKind::Breaking
            && sorted.breaking.contains(&self.target.unit.id)
        {
            sorted.unexplained.clone()
        } else {
            Vec::new()
        }
    }

    fn difference(&self, at: String, what: String) -> Difference {
        Difference {
            at,
            what,
            action: format!(
                "carry out the unit's `revert`, `{}`: {} becomes ask, with these differences attached",
                self.target.unit.revert.as_deref().unwrap_or_default(),
                self.target.id
            ),
        }
    }

    fn extra(&self, sorted: &Sorted) -> Vec<LockMove> {
        if !sorted.breaking.is_empty() {
            return Vec::new();
        }
        let unmade = self.accountable.unmade();
        sorted
            .unexplained
            .iter()
            .filter(|lock_move| !unmade.iter().any(|planned| planned.name == lock_move.name))
            .cloned()
            .collect()
    }

    fn missed(&self, lock_move: &LockMove, sorted: &Sorted) -> String {
        let name = &lock_move.name;
        let elsewhere = sorted
            .unexplained
            .iter()
            .find(|other| other.name == *name)
            .and_then(|other| other.to.as_ref());
        match (&lock_move.from, &lock_move.to) {
            (Some(from), Some(to)) if self.sorting.lock().holds(name, from) => {
                format!("`{name}` is still {from}, where the unit moves it to {to}")
            }
            (Some(from), Some(to)) => elsewhere.map_or_else(
                || format!("`{name}` {from} left the lock, where the unit moves it to {to}"),
                |found| {
                    format!("`{name}` moved {from} -> {found}, not to {to} as the unit moves it")
                },
            ),
            (None, Some(to)) => format!("`{name}` {to} is not in the lock, where the unit adds it"),
            (from, None) => format!(
                "`{name}` {} is still in the lock, where the unit removes it",
                from.as_ref().map(ToString::to_string).unwrap_or_default()
            ),
        }
    }

    fn pinned(&self, lock_move: &LockMove) -> Option<String> {
        let (Some(from), Some(to)) = (&lock_move.from, &lock_move.to) else {
            return None;
        };
        let pin = self.pins.iter().find(|pin| pin.name == lock_move.name)?;
        let name = &lock_move.name;
        match pin.hold {
            Hold::All => Some(format!(
                "`{name}` moved {from} -> {to}, which its pin holds with `hold = \"all\"`: {}",
                pin.reason
            )),
            Hold::Line if from.line() != to.line() => Some(format!(
                "`{name}` moved {from} -> {to}, off the {} line its pin holds: {}",
                from.line(),
                pin.reason
            )),
            Hold::Line => None,
        }
    }

    fn unedited(&self, edit: &WrittenLine) -> Difference {
        let now = self.sorting.lines().now(&edit.at, &edit.key);
        let reads = now
            .as_ref()
            .map(|now| now.value.clone())
            .unwrap_or_default();
        self.difference(
            now.map_or_else(|| edit.at.to_string(), |now| now.at.to_string()),
            format!(
                "`{}` still reads `{reads}`, where the unit writes `{}`",
                edit.key, edit.value
            ),
        )
    }

    fn unmoved(&self, source: &GitSource) -> String {
        match self.sorting.git().rev(&source.source) {
            Some(rev) if rev == source.from => format!(
                "`{}` is still at {rev}, where the unit moves it to {}",
                source.source, source.to
            ),
            Some(rev) => format!(
                "`{}` moved {} -> {rev}, not to {} as the unit moves it",
                source.source, source.from, source.to
            ),
            None => format!(
                "`{}` left the lock, where the unit moves it to {}",
                source.source, source.to
            ),
        }
    }
}
