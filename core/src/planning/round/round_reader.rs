// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::port_failure::PortFailure;
use crate::inputs::recorded_plan::RecordedPlan;
use crate::planning::pins_and_policy::policy_reader::PolicyReader;
use crate::planning::plan_failure::PlanFailure;
use crate::planning::round::current_state::CurrentState;
use crate::planning::round::round_inputs::RoundInputs;
use crate::planning::stopping::Stopping;
use crate::traits::workspace_files::WorkspaceFiles;

/// The workspace read again for `check` and `hold`: the policy first, its
/// errors stopping the command before anything else is read; then the
/// fingerprint, and where it is the plan's, nothing has changed and the lock
/// and the lines are the plan's own; otherwise the lock and the lines now.
pub struct RoundReader<'a> {
    files: &'a dyn WorkspaceFiles,
}

impl<'a> RoundReader<'a> {
    pub fn new(files: &'a dyn WorkspaceFiles) -> Self {
        Self { files }
    }

    pub fn read(&self, plan: &RecordedPlan) -> Result<RoundInputs, PlanFailure> {
        let tree = self
            .files
            .policy()
            .map_err(|failure| Stopping.with(&[], &failure))?;
        let policy = PolicyReader.read(&tree);
        Stopping.at_errors(&policy.diagnostics)?;
        let current = self
            .current(plan)
            .map_err(|failure| Stopping.with(&policy.diagnostics, &failure))?;
        Ok(RoundInputs {
            policy: policy.value,
            pins_at: tree
                .entries
                .iter()
                .find(|entry| entry.key == "pins")
                .map(|entry| entry.at.clone()),
            current,
            diagnostics: policy.diagnostics,
        })
    }

    fn current(&self, plan: &RecordedPlan) -> Result<CurrentState, PortFailure> {
        if self.files.fingerprint()? == plan.fingerprint {
            return Ok(CurrentState {
                lock: plan.lock.clone(),
                lines: plan.lines.clone(),
                git: plan.git.clone(),
                edges: plan.edges.clone().unwrap_or_default(),
            });
        }
        Ok(CurrentState {
            lock: self.files.lock()?,
            lines: self.files.written()?,
            git: self.files.git()?,
            edges: self.files.edges()?,
        })
    }
}
