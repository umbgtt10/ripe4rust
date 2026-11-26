// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::planning::judging::unit_facts::UnitFacts;
use crate::planning::judging::wording::Wording;
use crate::planning::units::concern::Concern;
use crate::planning::units::unit_id::UnitId;
use crate::planning::writing::answer::Answer;
use crate::planning::writing::constants::PLAN_FILE;
use crate::planning::writing::question_option::QuestionOption;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// What each answer means for one unit, and the step that carries it out.
pub struct OptionWriter<'a> {
    id: &'a UnitId,
    facts: &'a UnitFacts,
    concerns: &'a [Concern],
}

impl<'a> OptionWriter<'a> {
    pub fn new(id: &'a UnitId, facts: &'a UnitFacts, concerns: &'a [Concern]) -> Self {
        Self {
            id,
            facts,
            concerns,
        }
    }

    pub fn option(&self, answer: Answer) -> QuestionOption {
        let hold = format!(
            "cargo ripe4rust hold {} --plan {PLAN_FILE} --because \"<the person's words>\"",
            self.id
        );
        let now = if self.facts.reaches.is_empty() {
            String::from("apply it now")
        } else {
            format!(
                "apply it now and validate on {}",
                Wording.list(&self.facts.reaches)
            )
        };
        let (means, then) = match answer {
            Answer::Accept => (
                String::from("accept it in `deny.toml`, with the person's reason"),
                format!(
                    "add {} to `[advisories] ignore` in `deny.toml`, commit it, and plan again",
                    Self::listed(self.concerns.iter().map(Concern::ignore_entry).collect())
                ),
            ),
            Answer::DeclareGates => (
                String::from("declare the commands that validate a change"),
                String::from(
                    "add `gates = [<the person's commands>]` to `[workspace.metadata.ripe4rust]` in `Cargo.toml`, commit it, and plan again",
                ),
            ),
            Answer::KeepPinned => (
                String::from("keep it, with a new review date"),
                format!(
                    "set the `review` of {} to the person's date, commit it, and plan again",
                    self.pin()
                ),
            ),
            Answer::MigrateNow => (now, self.work_list()),
            Answer::PlanIt => (
                String::from("hold it until a planned migration"),
                format!("{hold} --review <date>"),
            ),
            Answer::Remove => (
                String::from(
                    "remove the dependency: the sources name it nowhere, though a crate can be needed for its features or side effects",
                ),
                format!(
                    "remove `{}` from every manifest that declares it and run the gates: green, commit it and plan again; red, restore the manifests and put the question again, the failure attached",
                    self.facts.name
                ),
            ),
            Answer::Replace => (
                String::from("replace the crate"),
                format!(
                    "replace {} as the person decides, commit it, and plan again",
                    Self::listed(
                        self.concerns
                            .iter()
                            .map(|concern| String::from(concern.name()))
                            .collect()
                    )
                ),
            ),
            Answer::TakeIt => (now, String::from("apply it as a ripe unit")),
            Answer::Unpin => (
                String::from("remove the pin and take the upgrade"),
                format!("remove {}, commit it, and plan again", self.pin()),
            ),
            Answer::Wait => (String::from("not now"), hold),
        };
        QuestionOption {
            answer,
            means,
            then,
        }
    }

    fn listed(items: Vec<String>) -> String {
        let mut quoted = items
            .into_iter()
            .map(|item| format!("`{item}`"))
            .collect::<Vec<String>>();
        quoted.dedup();
        quoted.join(" and ")
    }

    fn pin(&self) -> String {
        self.facts
            .pins
            .iter()
            .find(|standing| standing.released.is_some())
            .or_else(|| self.facts.pins.first())
            .map_or_else(
                || String::from("its pin"),
                |standing| format!("the pin on `{}` at {}", standing.pin.name, standing.pin.at),
            )
    }

    fn work_list(&self) -> String {
        match self.facts.footprint {
            Some(footprint) if !footprint.complete => format!(
                "apply it as a ripe unit; the footprint's {} are a floor of the work list, as code is included the parse cannot follow: the build's errors name the rest",
                Wording.count(footprint.lines, "location")
            ),
            Some(footprint) => format!(
                "apply it as a ripe unit; the footprint's {} are the work list",
                Wording.count(footprint.lines, "location")
            ),
            None => String::from(
                "apply it as a ripe unit; the footprint is not known, so the build's errors are the work list",
            ),
        }
    }
}
