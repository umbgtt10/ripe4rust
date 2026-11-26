// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::compatible;
use crate::support::plans::plan;
use cargo_ripe4rust::render::json::json_renderer::JsonRenderer;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use serde_json::Value;
use serde_json::from_str;
use serde_json::json;

fn parsed(text: &str) -> Value {
    from_str(text).expect("a JSON document")
}

#[test]
fn failure_prints_schema_command_and_diagnostics() {
    // Arrange
    let failure = PlanFailure {
        diagnostics: vec![DiagnosticCode::CargoDenyMissing.diagnose(
            None,
            String::from("cargo-deny is not installed"),
            String::from("install it"),
        )],
    };

    // Act
    let text = JsonRenderer::new("plan").failure(&failure);

    // Assert
    assert_eq!(
        parsed(&text),
        json!({
            "schema": 1,
            "command": "plan",
            "diagnostics": [{
                "severity": "error",
                "code": "cargo-deny-missing",
                "where": null,
                "what": "cargo-deny is not installed",
                "do": "install it",
                "needs_person": false,
            }],
        })
    );
}

#[test]
fn plan_prints_its_keys_in_the_design_s_order() {
    // Arrange & Act
    let text = JsonRenderer::new("plan").plan(&plan(Vec::new()));

    // Assert
    let keys = [
        "\"schema\"",
        "\"command\"",
        "\"date\"",
        "\"registry_read\"",
        "\"workspace\"",
        "\"gates\"",
        "\"diagnostics\"",
        "\"units\"",
    ]
    .map(|key| text.find(key).expect("the key"));
    assert!(keys.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn plan_prints_one_document_ending_in_a_newline() {
    // Arrange & Act
    let text = JsonRenderer::new("plan").plan(&plan(Vec::new()));

    // Assert
    assert!(text.ends_with("}\n"));
    assert_eq!(text.matches("\"schema\"").count(), 1);
}

#[test]
fn plan_prints_the_workspace_and_every_unit() {
    // Arrange & Act
    let text = JsonRenderer::new("plan").plan(&plan(vec![compatible()]));

    // Assert
    let document = parsed(&text);
    assert_eq!(document["schema"], json!(1));
    assert_eq!(document["command"], json!("plan"));
    assert_eq!(document["date"], json!("2026-10-30"));
    assert_eq!(document["registry_read"], json!("2026-10-30"));
    assert_eq!(
        document["workspace"],
        json!({ "fingerprint": "9f2c41d0e3b5a7c9", "lock": ["gadget 0.13.1", "widget 4.6.6"] })
    );
    assert_eq!(document["gates"], json!(["just stage1", "just stage2"]));
    assert_eq!(document["diagnostics"], json!([]));
    assert_eq!(document["units"][0]["id"], json!("compatible:widget"));
}
