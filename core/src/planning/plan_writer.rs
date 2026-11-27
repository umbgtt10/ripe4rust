// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::judging::facts::dependents::Dependents;
use crate::planning::judging::facts::plan_context::PlanContext;
use crate::planning::pins_and_policy::hold::Hold;
use crate::planning::plan::Plan;
use crate::planning::surveyed::Surveyed;
use crate::planning::unit_writer::UnitWriter;
use crate::planning::units::breaking_units::BreakingUnits;
use crate::planning::units::choice::histories::Histories;
use crate::planning::units::choice::release_history::ReleaseHistory;
use crate::planning::units::clearing::Clearing;
use crate::planning::units::compatible_units::CompatibleUnits;
use crate::planning::units::concern::Concern;
use crate::planning::units::grouping::Grouping;
use crate::planning::units::plan_order::PlanOrder;
use crate::planning::units::shared_crates::SharedCrates;
use crate::planning::units::toolchain_unit::ToolchainUnit;
use crate::planning::units::unit::Unit;
use crate::planning::workspace_state::WorkspaceState;
use crate::planning::writing::step_writer::StepWriter;
use crate::values::version::Version;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// The plan, made from a survey alone: the units built, grouped and put in
/// plan order, every advisory and yanked release on the unit that clears it
/// or in a row of its own, every row judged and written, and the crates the
/// units share named.
pub struct PlanWriter<'a> {
    surveyed: &'a Surveyed,
}

impl<'a> PlanWriter<'a> {
    pub fn new(surveyed: &'a Surveyed) -> Self {
        Self { surveyed }
    }

    pub fn plan(&self) -> Plan {
        let surveyed = self.surveyed;
        let histories = Histories::new(
            surveyed
                .releases
                .iter()
                .map(|(name, releases)| {
                    (
                        name.clone(),
                        ReleaseHistory::new(releases.clone(), Version::line),
                    )
                })
                .collect(),
        );
        let toolchain =
            ReleaseHistory::new(surveyed.toolchain_releases.clone(), Version::toolchain_line);
        let context = PlanContext::new(surveyed, &histories, &toolchain);
        let concerns = self.concerns(&histories);
        let mut units = self.units(&context, &concerns);
        units.extend(ToolchainUnit::new(&toolchain).build(surveyed.pinned.as_ref()));
        let units = PlanOrder.sorted(units);
        let clearing = Clearing::new(concerns, Dependents::new(&surveyed.workspace));
        let assigned = clearing.assign(&units);
        let rows = clearing.rows(&units);
        let mut lock = surveyed
            .workspace
            .lock
            .iter()
            .map(|package| format!("{} {}", package.name, package.version))
            .collect::<Vec<String>>();
        lock.sort();
        let mut ambiguous = surveyed
            .workspace
            .lock
            .iter()
            .map(|package| &package.name)
            .filter(|name| {
                surveyed
                    .workspace
                    .lock
                    .iter()
                    .filter(|package| package.name == **name)
                    .count()
                    > 1
            })
            .cloned()
            .collect::<Vec<String>>();
        ambiguous.dedup();
        let steps = StepWriter::new(surveyed.policy.gates.clone(), ambiguous);
        let writer = UnitWriter::new(&context, &steps);
        Plan {
            date: surveyed.today,
            registry_read: surveyed.registry_read,
            workspace: WorkspaceState {
                fingerprint: surveyed.fingerprint.clone(),
                lock,
                lines: surveyed.written.clone(),
                git: surveyed
                    .git
                    .iter()
                    .map(|entry| format!("{} {}#{}", entry.name, entry.source, entry.rev))
                    .collect(),
            },
            gates: surveyed.policy.gates.clone(),
            shared: self.shared(),
            diagnostics: surveyed.diagnostics.clone(),
            units: units
                .into_iter()
                .zip(assigned)
                .chain(rows)
                .map(|(unit, concerns)| writer.planned(unit, &concerns))
                .collect(),
        }
    }

    fn concerns(&self, histories: &Histories) -> Vec<Concern> {
        let advisories = self
            .surveyed
            .advisories
            .iter()
            .cloned()
            .map(Concern::Advisory);
        let yanked = self
            .surveyed
            .workspace
            .lock
            .iter()
            .filter(|package| histories.is_yanked(&package.name, &package.version))
            .map(|package| Concern::Yanked {
                name: package.name.clone(),
                version: package.version.clone(),
            });
        advisories.chain(yanked).collect()
    }

    fn held(&self) -> Vec<String> {
        self.surveyed
            .policy
            .pins
            .iter()
            .filter(|pin| pin.hold == Hold::All)
            .map(|pin| pin.name.clone())
            .collect()
    }

    fn shared(&self) -> Vec<String> {
        let surveyed = self.surveyed;
        let claimants = surveyed
            .dependency_moves
            .iter()
            .map(|dependency| dependency.moves.as_slice())
            .chain(
                surveyed
                    .source_moves
                    .iter()
                    .map(|source| source.moves.as_slice()),
            )
            .collect::<Vec<&[LockMove]>>();
        SharedCrates::new(self.held()).of(&claimants, &surveyed.workspace_moves)
    }

    fn units(&self, context: &PlanContext<'_>, concerns: &[Concern]) -> Vec<Unit> {
        let surveyed = self.surveyed;
        let compatible = CompatibleUnits::new(self.held(), concerns.to_vec()).build_with_sources(
            surveyed.dependency_moves.clone(),
            surveyed.source_moves.clone(),
            surveyed.workspace_moves.clone(),
        );
        let maturity = context.maturity();
        let breaking = BreakingUnits::new(context.histories(), &maturity, &surveyed.toolchain)
            .build(&surveyed.direct);
        let grouping = Grouping::new(
            &surveyed.repositories,
            &surveyed.workspace.lock,
            context.histories(),
        );
        let mut units = grouping.group(compatible);
        units.extend(grouping.group(breaking));
        units
    }
}
