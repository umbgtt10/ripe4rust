// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::inputs::port_failure::PortFailure;
use crate::inputs::release::Release;
use crate::inputs::scan::Scan;
use crate::inputs::workspace::Workspace;
use crate::planning::findings::Findings;
use crate::planning::pins_and_policy::policy::Policy;
use crate::planning::pins_and_policy::policy_reader::PolicyReader;
use crate::planning::pins_and_policy::reading::Reading;
use crate::planning::pins_and_policy::release_condition::ReleaseCondition;
use crate::planning::plan_failure::PlanFailure;
use crate::planning::ports::Ports;
use crate::planning::stopping::Stopping;
use crate::planning::surveyed::Surveyed;
use crate::planning::toolchain_survey::ToolchainSurvey;
use crate::planning::units::crate_repository::CrateRepository;
use crate::planning::units::dependency_moves::DependencyMoves;
use crate::planning::units::direct_dependencies::DirectDependencies;
use crate::planning::units::direct_dependency::DirectDependency;
use crate::values::date::Date;
use crate::values::location::Location;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;

const RUST: &str = "rust";

/// Every port read once, in an order that stops early: the policy first, as
/// its errors are reported before any rule runs, then the toolchain, then
/// the rest. A port that cannot answer stops the plan, with every
/// diagnostic found so far.
pub struct Survey<'a> {
    ports: Ports<'a>,
}

impl<'a> Survey<'a> {
    pub fn new(ports: Ports<'a>) -> Self {
        Self { ports }
    }

    pub fn moves(&self, direct: &[DirectDependency]) -> Result<Vec<DependencyMoves>, PortFailure> {
        direct
            .iter()
            .map(|dependency| {
                Ok(DependencyMoves {
                    dependency: dependency.clone(),
                    moves: self
                        .ports
                        .lock_moves
                        .of_crate(&dependency.name, &dependency.version)?,
                })
            })
            .collect()
    }

    pub fn policy(&self) -> Result<Reading<Policy>, PortFailure> {
        Ok(PolicyReader.read(&self.ports.files.policy()?))
    }

    pub fn reached(&self, policy: &Policy) -> Result<Vec<(String, Vec<String>)>, PortFailure> {
        policy
            .validate_beyond_gates
            .iter()
            .map(|target| Ok((target.clone(), self.ports.metadata.reached_by(target)?)))
            .collect()
    }

    pub fn releases(
        &self,
        workspace: &Workspace,
        policy: &Policy,
    ) -> Result<Vec<(String, Vec<Release>)>, PortFailure> {
        let mut names = workspace
            .lock
            .iter()
            .map(|package| package.name.clone())
            .chain(policy.pins.iter().filter_map(|pin| match &pin.until {
                Some(ReleaseCondition::Release { name, .. }) => Some(name.clone()),
                _ => None,
            }))
            .collect::<Vec<String>>();
        names.sort();
        names.dedup();
        names
            .into_iter()
            .map(|name| {
                let releases = self.ports.index.releases(&name)?;
                Ok((name, releases))
            })
            .collect()
    }

    pub fn repositories(
        &self,
        direct: &[DirectDependency],
    ) -> Result<Vec<CrateRepository>, PortFailure> {
        let mut known = Vec::new();
        for dependency in direct {
            if let Some(repository) = self.ports.repositories.repository(&dependency.name)? {
                known.push(CrateRepository {
                    name: dependency.name.clone(),
                    repository,
                });
            }
        }
        Ok(known)
    }

    pub fn requirement_lines(
        &self,
        direct: &[DirectDependency],
    ) -> Result<Vec<(String, Vec<Location>)>, PortFailure> {
        direct
            .iter()
            .map(|dependency| {
                Ok((
                    dependency.name.clone(),
                    self.ports.files.requirement_lines(&dependency.name)?,
                ))
            })
            .collect()
    }

    pub fn rust_version_lines(
        &self,
        workspace: &Workspace,
    ) -> Result<Vec<(String, Location)>, PortFailure> {
        let mut lines = Vec::new();
        for member in &workspace.members {
            if let Some(at) = self.ports.files.rust_version_line(&member.name)? {
                lines.push((member.name.clone(), at));
            }
        }
        Ok(lines)
    }

    pub fn survey(&self) -> Result<Surveyed, PlanFailure> {
        let today = self.ports.clock.today();
        let policy = self
            .policy()
            .map_err(|failure| Stopping.with(&[], &failure))?;
        let mut found = policy.diagnostics;
        Stopping.at_errors(&found)?;
        let policy = policy.value;
        let toolchain = self
            .toolchain(&policy)
            .map_err(|failure| Stopping.with(&found, &failure))?;
        found.extend(toolchain.diagnostics);
        Stopping.at_errors(&found)?;
        let toolchain = toolchain.value;
        let stop = |failure: PortFailure| Stopping.with(&found, &failure);
        let workspace = self.ports.metadata.read().map_err(stop)?;
        let advisories = self.ports.advisories.advisories().map_err(stop)?;
        let releases = self.releases(&workspace, &policy).map_err(stop)?;
        let scan = self.ports.scan.scan().map_err(stop)?;
        let direct = DirectDependencies.of(&workspace);
        let registry_read = self.ports.index.read_on();
        let surveyed = Surveyed {
            today,
            registry_read,
            fingerprint: self.ports.files.fingerprint().map_err(stop)?,
            dependency_moves: self.moves(&direct).map_err(stop)?,
            workspace_moves: self.ports.lock_moves.of_workspace().map_err(stop)?,
            repositories: self.repositories(&direct).map_err(stop)?,
            reached: self.reached(&policy).map_err(stop)?,
            requirement_lines: self.requirement_lines(&direct).map_err(stop)?,
            rust_version_lines: self.rust_version_lines(&workspace).map_err(stop)?,
            written: self.ports.files.written().map_err(stop)?,
            diagnostics: Vec::new(),
            pinned: toolchain.pinned,
            channel_at: toolchain.channel_at,
            toolchain: toolchain.in_use,
            toolchain_releases: toolchain.releases,
            advisories,
            releases,
            direct,
            scan,
            workspace,
            policy,
        };
        found.extend(self.warnings(
            &surveyed.policy,
            &surveyed.workspace,
            surveyed.scan.as_ref(),
            registry_read,
            today,
        ));
        Ok(Surveyed {
            diagnostics: found,
            ..surveyed
        })
    }

    pub fn toolchain(&self, policy: &Policy) -> Result<Reading<ToolchainSurvey>, PortFailure> {
        let file = self.ports.files.toolchain()?;
        let pinned = file
            .as_ref()
            .and_then(|file| Version::parse(file.channel.trim()).ok())
            .filter(|version| !version.is_pre_release());
        let mut diagnostics = Vec::new();
        let (in_use, releases) = if let Some(version) = &pinned {
            let releases = self.ports.toolchain.releases()?;
            diagnostics.extend(
                file.as_ref()
                    .filter(|_| releases.is_none())
                    .map(|file| Findings.toolchain_releases_unknown(file.at.clone())),
            );
            (version.clone(), releases.unwrap_or_default())
        } else {
            let rustc = self.ports.toolchain.rustc()?;
            diagnostics.push(Findings.toolchain_unpinned(file.as_ref(), &rustc));
            (rustc, Vec::new())
        };
        if !policy.validate_beyond_gates.is_empty() {
            let known = self.ports.toolchain.targets()?;
            diagnostics.extend(
                policy
                    .validate_beyond_gates
                    .iter()
                    .filter(|target| !known.contains(target))
                    .map(|target| Findings.target_unknown(target, policy.at.clone())),
            );
        }
        Ok(Reading {
            value: ToolchainSurvey {
                pinned,
                channel_at: file.map(|file| file.at),
                in_use,
                releases,
            },
            diagnostics,
        })
    }

    pub fn warnings(
        &self,
        policy: &Policy,
        workspace: &Workspace,
        scan: Option<&Scan>,
        registry_read: Date,
        today: Date,
    ) -> Vec<Diagnostic> {
        let absent = policy
            .pins
            .iter()
            .filter(|pin| {
                pin.name != RUST
                    && !workspace
                        .lock
                        .iter()
                        .any(|package| package.name == pin.name)
            })
            .map(|pin| Findings.pin_for_absent_crate(pin));
        let old =
            (registry_read < today).then(|| Findings.registry_cache_old(registry_read, today));
        let unparsed = scan
            .iter()
            .flat_map(|scan| scan.unparsed.iter())
            .map(|file| Findings.source_unparsed(file));
        let unscanned = scan.is_none().then(|| Findings.source_unscanned());
        absent.chain(old).chain(unparsed).chain(unscanned).collect()
    }
}
