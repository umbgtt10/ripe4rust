// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::plan::plan_text::PlanText;
use ripe4rust_core::inputs::git::git_source::GitSource;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::recorded_unit::RecordedUnit;
use ripe4rust_core::inputs::written_line::WrittenLine;
use ripe4rust_core::unit_kind::UnitKind;
use serde::Deserialize;
use serde_json::Value;
use serde_json::from_value;

#[derive(Deserialize)]
struct UnitFields {
    pub id: String,
    pub kind: String,
    #[serde(rename = "crate")]
    pub name: Option<String>,
    pub members: Vec<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    #[serde(default)]
    pub source: Option<String>,
    pub moves: Vec<MoveFields>,
    pub urgent: bool,
    pub steps: Option<StepsFields>,
}

#[derive(Deserialize)]
struct MoveFields {
    #[serde(rename = "crate")]
    pub name: String,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Deserialize)]
struct StepsFields {
    pub apply: Vec<Value>,
    pub revert: RevertFields,
    pub split: Vec<SplitFields>,
}

#[derive(Deserialize)]
struct RevertFields {
    pub run: String,
}

#[derive(Deserialize)]
struct SplitFields {
    pub id: String,
}

#[derive(Deserialize)]
struct EditFields {
    pub edit: String,
    pub key: String,
    pub value: String,
}

/// One unit of a plan document read back: its id, kind, crates and
/// versions -- a git unit's source and revs --, its moves, whether it is urgent, the edits its `apply` makes,
/// the ids of its split and its `revert` -- none for an advisory row, which
/// has no steps. An `apply` step that runs a command is read as nothing:
/// what it moves is the unit's moves.
pub struct UnitReader<'a> {
    read: &'a PlanText,
}

impl<'a> UnitReader<'a> {
    pub fn new(read: &'a PlanText) -> Self {
        Self { read }
    }

    pub fn unit(&self, value: &Value) -> Result<RecordedUnit, PortFailure> {
        let read = self.read;
        let unit = from_value::<UnitFields>(value.clone())
            .map_err(|error| read.unreadable(&error.to_string()))?;
        let kind = UnitKind::parse(&unit.kind).ok_or_else(|| {
            read.unreadable(&format!(
                "`{}` is of a kind ripe4rust does not know, `{}`",
                unit.id, unit.kind
            ))
        })?;
        let moves = unit
            .moves
            .iter()
            .map(|each| {
                Ok(LockMove {
                    name: each.name.clone(),
                    from: read.optional_version(each.from.as_deref())?,
                    to: read.optional_version(each.to.as_deref())?,
                })
            })
            .collect::<Result<Vec<LockMove>, PortFailure>>()?;
        let edits = unit
            .steps
            .iter()
            .flat_map(|steps| steps.apply.iter())
            .filter_map(|step| from_value::<EditFields>(step.clone()).ok())
            .map(|edit| {
                Ok(WrittenLine {
                    at: read.location(&edit.edit)?,
                    key: edit.key,
                    value: edit.value,
                })
            })
            .collect::<Result<Vec<WrittenLine>, PortFailure>>()?;
        let git = match (kind, &unit.source, &unit.from, &unit.to) {
            (UnitKind::Git, Some(source), Some(from), Some(to)) => Some(GitSource {
                source: source.clone(),
                from: from.clone(),
                to: to.clone(),
            }),
            (UnitKind::Git, ..) => {
                return Err(read.unreadable(&format!(
                    "`{}` is a git unit with no source and revs",
                    unit.id
                )));
            }
            _ => None,
        };
        let (from, to) = if git.is_some() {
            (None, None)
        } else {
            (
                read.optional_version(unit.from.as_deref())?,
                read.optional_version(unit.to.as_deref())?,
            )
        };
        Ok(RecordedUnit {
            kind,
            name: unit.name,
            members: unit.members,
            from,
            to,
            moves,
            urgent: unit.urgent,
            edits,
            splits: unit
                .steps
                .iter()
                .flat_map(|steps| steps.split.iter().map(|split| split.id.clone()))
                .collect(),
            revert: unit.steps.map(|steps| steps.revert.run),
            git,
            id: unit.id,
        })
    }
}
