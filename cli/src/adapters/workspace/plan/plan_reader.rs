// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::plan::plan_text::PlanText;
use crate::adapters::workspace::plan::unit_reader::UnitReader;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::recorded_plan::RecordedPlan;
use ripe4rust_core::inputs::written_line::WrittenLine;
use serde::Deserialize;
use serde_json::Value;
use serde_json::from_str;
use serde_json::from_value;

const SCHEMA: &str = "1";
const PLAN: &str = "plan";

#[derive(Deserialize)]
struct PlanJson {
    pub date: String,
    pub workspace: WorkspaceJson,
    #[serde(default)]
    pub shared: Vec<String>,
    pub units: Vec<Value>,
}

#[derive(Deserialize)]
struct WorkspaceJson {
    pub fingerprint: String,
    pub lock: Vec<String>,
    #[serde(default)]
    pub git: Vec<String>,
    pub lines: Vec<LineJson>,
}

#[derive(Deserialize)]
struct LineJson {
    #[serde(rename = "where")]
    pub at: String,
    pub key: String,
    pub value: String,
}

/// A plan document read back: of schema 1, written by `plan`, and holding
/// a plan -- the date, the workspace, the crates its units share, none for a
/// plan written before they were named, and the units -- or `plan-unreadable`,
/// and `plan-schema-unknown` for a schema this release does not read. What
/// the document holds beyond what the round's commands read is left alone.
pub struct PlanReader;

impl PlanReader {
    pub fn plan(&self, file: &str, text: &str) -> Result<RecordedPlan, PortFailure> {
        let read = PlanText::new(file);
        let document =
            from_str::<Value>(text).map_err(|error| read.unreadable(&error.to_string()))?;
        let schema = document
            .get("schema")
            .map(ToString::to_string)
            .ok_or_else(|| {
                read.unreadable("it carries no `schema`, so it is no document ripe4rust wrote")
            })?;
        if schema != SCHEMA {
            return Err(PortFailure::PlanSchemaUnknown {
                file: String::from(file),
                schema,
            });
        }
        let command = document
            .get("command")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if command != PLAN {
            return Err(read.unreadable(&format!(
                "it is the document of `{command}`, not of `{PLAN}`"
            )));
        }
        if document.get("units").is_none() {
            return Err(read.unreadable("it holds no units: the run that wrote it made no plan"));
        }
        let plan = from_value::<PlanJson>(document)
            .map_err(|error| read.unreadable(&error.to_string()))?;
        let units = UnitReader::new(&read);
        Ok(RecordedPlan {
            date: read.date(&plan.date)?,
            fingerprint: plan.workspace.fingerprint,
            lock: plan
                .workspace
                .lock
                .iter()
                .map(|entry| read.entry(entry))
                .collect::<Result<_, _>>()?,
            lines: plan
                .workspace
                .lines
                .into_iter()
                .map(|line| {
                    Ok(WrittenLine {
                        at: read.location(&line.at)?,
                        key: line.key,
                        value: line.value,
                    })
                })
                .collect::<Result<_, PortFailure>>()?,
            git: plan
                .workspace
                .git
                .iter()
                .map(|entry| read.git_entry(entry))
                .collect::<Result<_, _>>()?,
            shared: plan.shared,
            units: plan
                .units
                .iter()
                .map(|unit| units.unit(unit))
                .collect::<Result<_, _>>()?,
        })
    }
}
