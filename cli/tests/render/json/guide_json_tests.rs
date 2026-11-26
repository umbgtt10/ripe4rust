// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::json::paths;
use crate::support::json::value;
use crate::support::reports::as_planned;
use crate::support::reports::held;
use crate::support::reports::not_as_planned;
use cargo_ripe4rust::render::document_renderer::DocumentRenderer;
use cargo_ripe4rust::render::json::guide_json::GuideJson;
use cargo_ripe4rust::render::json::json_renderer::JsonRenderer;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::guiding::guide::Guide;
use serde_json::Value;
use serde_json::from_str;
use serde_json::json;

fn parsed(text: &str) -> Value {
    from_str(text).expect("a JSON document")
}

#[test]
fn document_of_the_guide_has_an_entry_for_every_code_the_core_can_produce() {
    // Arrange
    let text = Guide::new("0.0.1").text();

    // Act
    let document = value(&GuideJson.document(&text));

    // Assert
    let codes = document["codes"]
        .as_array()
        .expect("the codes")
        .iter()
        .map(|entry| entry["code"].as_str().unwrap_or_default())
        .collect::<Vec<&str>>();
    assert_eq!(codes, DiagnosticCode::ALL.map(|code| code.label()).to_vec());
    assert_eq!(
        document["codes"][0],
        json!({
            "code": "cargo-deny-failed",
            "severity": "error",
            "needs_person": false,
            "means": DiagnosticCode::CargoDenyFailed.entry().means,
            "do": DiagnosticCode::CargoDenyFailed.entry().remedy
        })
    );
}

#[test]
fn document_of_the_guide_is_versioned_and_carries_the_round_and_the_commands() {
    // Arrange
    let text = Guide::new("0.0.1").text();

    // Act
    let document = value(&GuideJson.document(&text));

    // Assert
    assert_eq!(document["version"], "0.0.1");
    assert_eq!(
        document["round"][0],
        "1. Run `cargo ripe4rust plan --format json > plan.json`."
    );
    assert_eq!(
        document["commands"][1]["exits"][3],
        json!({ "code": 3, "means": "the plan is stale: plan again, and go on with the new plan" })
    );
    assert_eq!(
        document["fields"][0],
        json!({
            "document": "every",
            "field": "schema",
            "means": "the shape of the document: 1; it moves when the shape does"
        })
    );
    assert_eq!(document["diagnostics"], json!([]));
}

#[test]
fn document_of_the_guide_names_every_field_each_document_prints() {
    // Arrange
    let guide = Guide::new("0.0.1").text();
    let renderer = |command: &str| JsonRenderer::new(command);
    let failure = PlanFailure {
        diagnostics: vec![DiagnosticCode::PlanStale.diagnose(
            None,
            String::from("moved"),
            String::from("plan again"),
        )],
    };
    let mut checked = not_as_planned();
    checked.transitive = as_planned().transitive;
    let documents = [
        (
            "plan",
            parsed(include_str!("../../fixtures/workspace/plan.json")),
        ),
        ("check", parsed(&renderer("check").check(&checked))),
        ("hold", parsed(&renderer("hold").hold(&held(true)))),
        ("guide", parsed(&renderer("guide").guide(&guide))),
        ("every", parsed(&renderer("check").failure(&failure))),
    ];

    // Act
    let named = guide
        .fields
        .iter()
        .map(|field| (field.document.as_str(), field.field.as_str()))
        .collect::<Vec<(&str, &str)>>();

    // Assert
    for (document, printed) in &documents {
        for path in paths(printed) {
            assert!(
                named.contains(&(document, path.as_str()))
                    || named.contains(&("every", path.as_str())),
                "`{path}` of `{document}` has no entry in the guide"
            );
        }
    }
}
