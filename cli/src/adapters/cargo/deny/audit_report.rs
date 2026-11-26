// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::deny::advisory_hit::AdvisoryHit;
use crate::adapters::cargo::unreadable::Unreadable;
use ripe4rust_core::inputs::advisory::Advisory;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::requirement::Requirement;
use serde_json::Value;
use serde_json::from_str;

const DOING: &str = "reading what `cargo deny` printed";

/// What `--audit-compatible-output` writes to stdout, a report a line, one
/// for each advisory database: where each advisory in force is clear of it,
/// its `patched` and `unaffected` ranges, read from the vulnerabilities and
/// from every kind of warning. An advisory the report does not hold clears
/// at no version.
pub struct AuditReport;

impl AuditReport {
    pub fn advisories(
        &self,
        text: &str,
        hits: &[AdvisoryHit],
    ) -> Result<Vec<Advisory>, PortFailure> {
        let reports = text
            .lines()
            .filter_map(|line| from_str::<Value>(line).ok())
            .collect::<Vec<Value>>();
        let entries = reports
            .iter()
            .flat_map(|report| {
                let vulnerabilities = report["vulnerabilities"].as_array().into_iter().flatten();
                let warnings = report["warnings"]
                    .as_object()
                    .into_iter()
                    .flat_map(|kinds| kinds.values())
                    .filter_map(Value::as_array)
                    .flatten();
                vulnerabilities.chain(warnings)
            })
            .collect::<Vec<&Value>>();
        hits.iter()
            .map(|hit| {
                let entry = entries.iter().find(|entry| {
                    entry["advisory"]["id"] == hit.id.as_str()
                        && entry["package"]["name"] == hit.name.as_str()
                        && entry["package"]["version"] == hit.version.to_string().as_str()
                });
                let ranges = |kind: &str| {
                    entry
                        .and_then(|entry| entry["versions"][kind].as_array())
                        .into_iter()
                        .flatten()
                        .filter_map(Value::as_str)
                        .map(|range| {
                            Requirement::parse(range).map_err(|failure| {
                                Unreadable::new(DOING).failure(&format!(
                                    "`{}` is not {}",
                                    failure.text, failure.expected
                                ))
                            })
                        })
                        .collect::<Result<Vec<Requirement>, PortFailure>>()
                };
                Ok(Advisory {
                    id: hit.id.clone(),
                    name: hit.name.clone(),
                    version: hit.version.clone(),
                    patched: ranges("patched")?,
                    unaffected: ranges("unaffected")?,
                })
            })
            .collect()
    }
}
