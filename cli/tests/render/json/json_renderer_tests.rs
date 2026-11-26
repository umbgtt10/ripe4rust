// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::plans::breaking;
use crate::support::plans::compatible;
use crate::support::plans::plan;
use crate::support::reports::as_planned;
use crate::support::reports::held;
use cargo_ripe4rust::render::document_renderer::DocumentRenderer;
use cargo_ripe4rust::render::json::json_renderer::JsonRenderer;
use ripe4rust_core::diagnostics::diagnostic_code::DiagnosticCode;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::round::guiding::guide::Guide;
use serde_json::Value;
use serde_json::from_str;
use serde_json::json;

fn compact(text: &str) -> String {
    text.split_whitespace().collect()
}

fn parsed(text: &str) -> Value {
    from_str(text).expect("a JSON document")
}

#[test]
fn check_prints_schema_and_command_before_the_report() {
    // Arrange & Act
    let text = JsonRenderer::new("check").check(&as_planned());

    // Assert
    assert!(
        compact(&text).starts_with(
            r#"{"schema":1,"command":"check","unit":"breaking:gadget","as_planned":true,"#
        ),
        "{text}"
    );
    assert!(text.ends_with("}\n"));
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
fn guide_prints_schema_and_command_before_the_guide() {
    // Arrange & Act
    let text = JsonRenderer::new("guide").guide(&Guide::new("0.0.1").text());

    // Assert
    assert!(
        compact(&text).starts_with(r#"{"schema":1,"command":"guide","version":"0.0.1","round":["#),
        "{text}"
    );
}

#[test]
fn hold_prints_schema_and_command_before_the_pins() {
    // Arrange & Act
    let text = JsonRenderer::new("hold").hold(&held(true));

    // Assert
    assert!(
        compact(&text).starts_with(
            r#"{"schema":1,"command":"hold","unit":"compatible:widget","section":"[workspace.metadata.ripe4rust.pins]","where":"Cargo.toml:40","pins":[{"crate":"widget","line":"#
        ),
        "{text}"
    );
}

#[test]
fn plan_prints_every_object_s_keys_in_the_design_s_order() {
    // Arrange
    let mut planned = plan(vec![compatible(), breaking()]);
    planned.diagnostics = vec![DiagnosticCode::GatesUndeclared.diagnose(
        None,
        String::from("no gates"),
        String::from("declare them"),
    )];

    // Act
    let text = compact(&JsonRenderer::new("plan").plan(&planned));

    // Assert
    for ordered in [
        r#""workspace":{"fingerprint":"9f2c41d0e3b5a7c9","lock":["#,
        r#""lines":[{"where":"Cargo.toml:22","key":"gadget","value":"0.13"},"#,
        r#"{"severity":"warning","code":"gates-undeclared","where":null,"what":"nogates","do":"declarethem","needs_person":true}"#,
        r#"{"id":"compatible:widget","kind":"compatible","crate":"widget","members":["widget"],"from":"4.6.6","to":"4.6.7","moves":[{"crate":"widget","from":"4.6.6","to":"4.6.7"}"#,
        r#""outcome":"ripe","urgent":false,"rule":10,"suggestion":null,"reasons":["compatible"],"summary":"#,
        r#""steps":{"apply":[{"run":"cargoupdate--recursive-pwidget"}],"check":"#,
        r#""revert":{"restore":["Cargo.lock"],"run":"gitrestore--Cargo.lock"},"split":[{"id":"compatible:widget/widget","apply":["#,
        r#""question":null,"evidence":{"line_in_use":{"line":"4.x","first_release":"2022-01-01","last_release":"2026-10-02","releases":41},"rust_version":"1.74","advisories":[],"yanked_in_use":false,"footprint":[],"held_back":[],"held":[]}"#,
        r#"{"edit":"Cargo.toml:22","key":"gadget","value":"0.14.0"}"#,
        r#""question":{"title":"Movegadgetto0.14.0?","why":"#,
        r#""options":[{"answer":"wait","means":"notnow","then":"#,
    ] {
        assert!(text.contains(ordered), "{ordered} in {text}");
    }
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
        json!({
            "fingerprint": "9f2c41d0e3b5a7c9",
            "lock": ["gadget 0.13.1", "widget 4.6.6"],
            "lines": [
                { "where": "Cargo.toml:22", "key": "gadget", "value": "0.13" },
                { "where": "rust-toolchain.toml:2", "key": "channel", "value": "1.99.1" }
            ]
        })
    );
    assert_eq!(document["gates"], json!(["just stage1", "just stage2"]));
    assert_eq!(document["diagnostics"], json!([]));
    assert_eq!(document["units"][0]["id"], json!("compatible:widget"));
}
