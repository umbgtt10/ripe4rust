// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::units::unit::Unit;
use crate::planning::writing::constants::PLAN_FILE;
use crate::planning::writing::revert::Revert;
use crate::planning::writing::split_step::SplitStep;
use crate::planning::writing::step::Step;
use crate::planning::writing::steps::Steps;
use crate::unit_kind::UnitKind;
use crate::values::version::Version;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

const LOCK: &str = "Cargo.lock";

/// A unit's steps. A compatible unit is one `cargo update --recursive`, or,
/// with a move held out of it -- or for the rest, whose moves are taken
/// alone -- one `--precise` update per move; it splits into its moves. A
/// breaking unit edits its lines, then updates; a git unit updates its
/// source by one of its crates, which moves every crate of it, and never
/// splits; the toolchain writes its channel. A crate the lock holds at several versions is named at its own.
pub struct StepWriter {
    gates: Vec<String>,
    ambiguous: Vec<String>,
}

impl StepWriter {
    pub fn new(gates: Vec<String>, ambiguous: Vec<String>) -> Self {
        Self { gates, ambiguous }
    }

    pub fn spec(&self, name: &str, from: Option<&Version>) -> String {
        match from {
            Some(from) if self.ambiguous.iter().any(|known| known == name) => {
                format!("{name}@{from}")
            }
            _ => String::from(name),
        }
    }

    pub fn steps(&self, unit: &Unit, edits: &[Step]) -> Option<Steps> {
        let apply = match unit.kind {
            UnitKind::Advisory => return None,
            UnitKind::Compatible => self.compatible(unit),
            UnitKind::Breaking => {
                let mut apply = edits.to_vec();
                apply.extend(unit.moves.iter().map(|lock_move| Step::Run {
                    command: format!(
                        "cargo update -p {}",
                        self.spec(&lock_move.name, lock_move.from.as_ref())
                    ),
                }));
                apply
            }
            UnitKind::Toolchain => edits.to_vec(),
            UnitKind::Git => unit
                .name
                .iter()
                .map(|name| Step::Run {
                    command: format!("cargo update -p {}", self.spec(name, None)),
                })
                .collect(),
        };
        let mut restore: Vec<String> = Vec::new();
        if unit.kind != UnitKind::Toolchain {
            restore.push(String::from(LOCK));
        }
        for file in edits.iter().filter_map(|edit| match edit {
            Step::Edit { at, .. } => Some(&at.file),
            Step::Run { .. } => None,
        }) {
            if !restore.contains(file) {
                restore.push(file.clone());
            }
        }
        Some(Steps {
            apply,
            check: format!("cargo ripe4rust check {} --plan {PLAN_FILE}", unit.id),
            gates: self.gates.clone(),
            revert: Revert {
                run: format!("git restore -- {}", restore.join(" ")),
                restore,
            },
            split: self.split(unit),
        })
    }

    fn compatible(&self, unit: &Unit) -> Vec<Step> {
        if unit.held.is_empty() && !unit.members.is_empty() {
            let packages = unit
                .members
                .iter()
                .map(|member| {
                    let from = unit
                        .moves
                        .iter()
                        .find(|lock_move| lock_move.name == *member)
                        .and_then(|lock_move| lock_move.from.as_ref());
                    format!("-p {}", self.spec(member, from))
                })
                .collect::<Vec<String>>();
            return vec![Step::Run {
                command: format!("cargo update --recursive {}", packages.join(" ")),
            }];
        }
        unit.moves
            .iter()
            .filter_map(|lock_move| self.precise(lock_move))
            .collect()
    }

    fn precise(&self, lock_move: &LockMove) -> Option<Step> {
        let (Some(from), Some(to)) = (&lock_move.from, &lock_move.to) else {
            return None;
        };
        Some(Step::Run {
            command: format!(
                "cargo update -p {} --precise {to}",
                self.spec(&lock_move.name, Some(from))
            ),
        })
    }

    fn split(&self, unit: &Unit) -> Vec<SplitStep> {
        let moves = unit
            .moves
            .iter()
            .filter_map(|lock_move| self.precise(lock_move).map(|step| (&lock_move.name, step)))
            .collect::<Vec<_>>();
        if unit.kind != UnitKind::Compatible || moves.len() < 2 {
            return Vec::new();
        }
        moves
            .into_iter()
            .map(|(name, step)| SplitStep {
                id: unit.id.split(name),
                apply: vec![step],
            })
            .collect()
    }
}
