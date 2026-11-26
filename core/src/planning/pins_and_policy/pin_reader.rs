// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use crate::diagnostics::diagnostic_code::DiagnosticCode;
use crate::inputs::policy_entry::PolicyEntry;
use crate::inputs::policy_value::PolicyValue;
use crate::planning::pins_and_policy::hold::Hold;
use crate::planning::pins_and_policy::pin::Pin;
use crate::planning::pins_and_policy::policy_values::PolicyValues;
use crate::planning::pins_and_policy::reading::Reading;
use crate::planning::pins_and_policy::release_condition::ReleaseCondition;
use crate::planning::pins_and_policy::until_reader::UntilReader;
use crate::values::date::Date;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

const KEYS: [&str; 4] = ["hold", "reason", "review", "until"];

/// One pin of `[workspace.metadata.ripe4rust.pins]`: a reason, and a review
/// date or a condition that can be checked, or it is refused.
pub struct PinReader;

impl PinReader {
    pub fn read(&self, entry: &PolicyEntry) -> Reading<Option<Pin>> {
        let within = format!("the pin on `{}`", entry.key);
        let values = PolicyValues::new(&within);
        let keys = match values.table(entry) {
            Ok(keys) => keys,
            Err(diagnostic) => {
                return Reading {
                    value: None,
                    diagnostics: vec![diagnostic],
                };
            }
        };
        let reason = Self::reason(&values, entry, keys);
        let review = Self::find(keys, "review")
            .map(|found| values.date(found))
            .transpose();
        let hold = Self::find(keys, "hold")
            .map(|found| Self::hold(&values, found))
            .transpose();
        let until = Self::find(keys, "until").map_or_else(
            || Reading {
                value: None,
                diagnostics: Vec::new(),
            },
            |found| UntilReader::new(&within).read(found),
        );
        let mut diagnostics = values.unknown(keys, &KEYS);
        diagnostics.extend(reason.clone().err());
        diagnostics.extend(review.clone().err());
        diagnostics.extend(hold.clone().err());
        let release = Self::released(entry, &review, &until);
        diagnostics.extend(until.diagnostics);
        diagnostics.extend(release);
        let pin = match (reason, review, hold, diagnostics.is_empty()) {
            (Ok(reason), Ok(review), Ok(hold), true) => Some(Pin {
                name: entry.key.clone(),
                reason,
                review,
                until: until.value,
                hold: hold.unwrap_or(Hold::Line),
                at: entry.at.clone(),
            }),
            _ => None,
        };
        Reading {
            value: pin,
            diagnostics,
        }
    }

    fn find<'a>(keys: &'a [PolicyEntry], key: &str) -> Option<&'a PolicyEntry> {
        keys.iter().find(|entry| entry.key == key)
    }

    fn hold(values: &PolicyValues, entry: &PolicyEntry) -> Result<Hold, Diagnostic> {
        match &entry.value {
            PolicyValue::Text(word) if word == "line" => Ok(Hold::Line),
            PolicyValue::Text(word) if word == "all" => Ok(Hold::All),
            _ => Err(values.invalid(entry, "`line` or `all`")),
        }
    }

    fn reason(
        values: &PolicyValues,
        pin: &PolicyEntry,
        keys: &[PolicyEntry],
    ) -> Result<String, Diagnostic> {
        match Self::find(keys, "reason").map(|found| values.text(found)) {
            Some(Ok(reason)) if !reason.trim().is_empty() => Ok(reason),
            Some(Err(diagnostic)) => Err(diagnostic),
            _ => Err(DiagnosticCode::PinWithoutReason.diagnose(
                Some(pin.at.clone()),
                format!("the pin on `{}` gives no reason", pin.key),
                String::from("add `reason = \"...\"` to it: why the crate is held"),
            )),
        }
    }

    fn released(
        pin: &PolicyEntry,
        review: &Result<Option<Date>, Diagnostic>,
        until: &Reading<Option<ReleaseCondition>>,
    ) -> Option<Diagnostic> {
        let checkable = matches!(until.value, Some(ReleaseCondition::Release { .. }));
        let unreleased = matches!(review, Ok(None)) && !checkable && until.diagnostics.is_empty();
        unreleased.then(|| {
            DiagnosticCode::PinWithoutRelease.diagnose(
                Some(pin.at.clone()),
                format!(
                    "the pin on `{}` has neither a review date nor a release condition that can be checked",
                    pin.key
                ),
                String::from(
                    "add `review = \"YYYY-MM-DD\"` to it, or `until = { crate = \"...\", version = \">=...\" }`: when to look at it again",
                ),
            )
        })
    }
}
