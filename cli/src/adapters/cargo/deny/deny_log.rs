// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::deny::advisory_hit::AdvisoryHit;
use crate::adapters::cargo::deny::deny_findings::DenyFindings;
use crate::adapters::complaint::Complaint;
use ripe4rust_core::values::version::Version;
use serde_json::Value;
use serde_json::from_str;

const ADVISORY_CODES: [&str; 4] = ["notice", "unmaintained", "unsound", "vulnerability"];
const ERROR: &str = "error";
const ERROR_LEVEL: &str = "ERROR";

/// What `cargo deny --format json` writes to stderr, a JSON object a line:
/// every advisory diagnostic in force, once for each crate it names -- an
/// advisory `deny.toml` ignores is not among them -- and every error, of its
/// log or of a check that could not run. A line that is not JSON is skipped.
pub struct DenyLog;

impl DenyLog {
    pub fn read(&self, text: &str) -> DenyFindings {
        let mut findings = DenyFindings {
            hits: Vec::new(),
            errors: Vec::new(),
        };
        for line in text.lines().filter_map(|line| from_str::<Value>(line).ok()) {
            let fields = &line["fields"];
            let message = Complaint.line(fields["message"].as_str().unwrap_or_default());
            let advisory = fields["advisory"]["id"]
                .as_str()
                .filter(|_| ADVISORY_CODES.contains(&fields["code"].as_str().unwrap_or_default()));
            match (line["type"].as_str(), advisory) {
                (Some("diagnostic"), Some(id)) => findings.hits.extend(Self::hits(id, fields)),
                (Some("diagnostic"), None) if fields["severity"] == ERROR => {
                    findings.errors.push(message);
                }
                (Some("log"), _) if fields["level"] == ERROR_LEVEL => findings.errors.push(message),
                _ => {}
            }
        }
        findings
    }

    fn hits(id: &str, fields: &Value) -> Vec<AdvisoryHit> {
        fields["graphs"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|graph| {
                let krate = &graph["Krate"];
                Some(AdvisoryHit {
                    id: String::from(id),
                    name: String::from(krate["name"].as_str()?),
                    version: Version::parse(krate["version"].as_str()?).ok()?,
                })
            })
            .collect()
    }
}
