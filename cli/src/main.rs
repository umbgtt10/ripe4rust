// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::outcome::Outcome;
use std::process::ExitCode;

const DESIGN: &str = "https://github.com/umbgtt10/ripe4rust/blob/main/docs/DESIGN.md";

// Design stage: nothing decides yet, so the command says so and exits 2, the
// code the design gives to "no plan could be produced".
fn main() -> ExitCode {
    eprintln!(
        "cargo-ripe4rust {}: design stage, no plan can be produced yet. Every upgrade will be {}, {} or {}; the design is at {DESIGN}",
        env!("CARGO_PKG_VERSION"),
        Outcome::Ripe.label(),
        Outcome::Ask.label(),
        Outcome::Ripening.label(),
    );
    ExitCode::from(2)
}
