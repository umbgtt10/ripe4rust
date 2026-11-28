// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serge4rust::crap::crap_report_parser::CrapReportParser;
use serge4rust::gates::boundary::boundary_rule::BoundaryRule;
use serge4rust::gates::boundary::crate_root_attributes::CrateRootAttributes;
use serge4rust::gates::boundary::disk_source_files::DiskSourceFiles;
use serge4rust::gates::boundary::package_dependencies::PackageDependencies;
use serge4rust::gates::boundary_gate::BoundaryGate;
use serge4rust::gates::crap_gate::CrapGate;
use serge4rust::gates::deny_gate::DenyGate;
use serge4rust::gates::dry_gate::DryGate;
use serge4rust::gates::gate::Gate;
use serge4rust::gates::iceberg_gate::IcebergGate;
use serge4rust::gates::shear_gate::ShearGate;
use serge4rust::gates::stage::Stage;
use serge4rust::gates::stern_gate::SternGate;
use serge4rust::gates::twin_gate::TwinGate;
use serge4rust::process::system_command_runner::SystemCommandRunner;
use serge4rust::scores::scoring_hook::ScoringHook;
use std::env::args;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

const CLI_PACKAGE: &str = "cargo-ripe4rust";
const CORE_DIRECTORY: &str = "core";
const CORE_PACKAGE: &str = "ripe4rust-core";
const XTASK_PACKAGE: &str = "xtask";
const ALLOWED_CRATES: [&str; 0] = [];
const CORE_DIRS: [[&str; 2]; 2] = [["core", "src"], ["core", "tests"]];
const CORE_ROOT: [&str; 3] = ["core", "src", "lib.rs"];
const DRY_EXCLUDES: [&[&str]; 3] = [&["xtask"], &["core", "tests"], &["cli", "tests"]];
const FORBIDDEN_MODULES: [&str; 7] = [
    "std::env",
    "std::fs",
    "std::io",
    "std::net",
    "std::os",
    "std::process",
    "std::thread",
];
const FORBIDDEN_MACROS: [&str; 5] = ["dbg", "eprint", "eprintln", "print", "println"];
const REQUIRED_ATTRIBUTES: [&str; 2] = ["no_std", "forbid(unsafe_code)"];
const FORBIDDEN_METHODS: [&str; 10] = [
    "canonicalize",
    "exists",
    "is_dir",
    "is_file",
    "is_symlink",
    "metadata",
    "read_dir",
    "read_link",
    "symlink_metadata",
    "try_exists",
];
const CRAP_THRESHOLD: &str = "15";
const ICEBERG_THRESHOLD: &str = "7";
const DRY_BASELINE: &str = "dry4rust-baseline.json";
const DRY_MIN_NODES: &str = "25";
const DENY_FETCH_ATTEMPTS: usize = 3;

// Which gates each stage runs, in what order, built from this workspace's
// constants: the gates, the runner and the scoring are serge4rust's, and are
// tested there, so reading argv and wiring them is all this binary does.
fn main() -> ExitCode {
    match args().nth(1).as_deref() {
        Some("stage2") => run_stage2(),
        Some("grip-braintax") => run_grip_braintax(),
        _ => {
            eprintln!("usage: cargo xtask <stage2|grip-braintax>");
            ExitCode::FAILURE
        }
    }
}

// core's grip and braintax as the commit stages it, written into the commit
// message whose file the commit-msg hook is handed -- or printed, given none.
// core is the one crate scored, at the workspace's root, so its block holds
// core's row alone.
fn run_grip_braintax() -> ExitCode {
    let runner = SystemCommandRunner::new();
    match ScoringHook::single_crate(&runner, String::from(CORE_DIRECTORY)).run(args().nth(2)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(reason) => {
            eprintln!("\nFailed: {reason}");
            ExitCode::FAILURE
        }
    }
}

fn run_stage2() -> ExitCode {
    let manifest_path = workspace_manifest_path();
    let runner = SystemCommandRunner::new();
    let parser = CrapReportParser::new();

    // Every member, so the crate that enforces the bar is held to it too.
    let packages = vec![
        String::from(CORE_PACKAGE),
        String::from(CLI_PACKAGE),
        String::from(XTASK_PACKAGE),
    ];

    // First: core stays free of technology -- no dependency in its manifest,
    // `no_std` and `forbid(unsafe_code)` on its root, and no file system,
    // process, thread, environment or stdout in its sources or its tests -- so
    // every input arrives through a trait, and every rule is tested against
    // fakes, the date included. A breach is fixed by moving code into cli,
    // which changes what every later gate, stern4rust's included, measures; and
    // it costs a second.
    let root = workspace_root();
    let sources = DiskSourceFiles::new();
    let dependencies = PackageDependencies::new();
    let attributes = CrateRootAttributes::new();
    let boundary = BoundaryGate::new(
        &runner,
        &sources,
        &dependencies,
        &attributes,
        manifest_path.clone(),
        BoundaryRule {
            package: String::from(CORE_PACKAGE),
            source_dirs: CORE_DIRS.iter().map(|dir| below(&root, dir)).collect(),
            allowed_crates: ALLOWED_CRATES.map(String::from).to_vec(),
            crate_root: below(&root, &CORE_ROOT),
            required_attributes: REQUIRED_ATTRIBUTES.map(String::from).to_vec(),
            forbidden_modules: FORBIDDEN_MODULES.map(String::from).to_vec(),
            forbidden_macros: FORBIDDEN_MACROS.map(String::from).to_vec(),
            forbidden_methods: FORBIDDEN_METHODS.map(String::from).to_vec(),
        },
    );

    // Second: its corrections are renames, file moves and directory splits,
    // so a layout it is about to reject is a layout the rest would measure
    // for nothing. Every rule holds, so there is no baseline; it runs the
    // release on PATH, as every other gate here does.
    let stern = SternGate::new(&runner, manifest_path.clone(), packages.clone());

    // Third: every declared dependency used, in the section that uses it. Its
    // corrections edit manifests only, so they move nothing the gates behind it
    // measure, and it costs a second.
    let shear = ShearGate::new(&runner, manifest_path.clone());

    // Fourth, for the same reason stern4rust runs early: removing a duplicate
    // moves code between files. One run over both crates' sources, so a copy
    // from one into the other is seen -- tests repeat their arrangement by
    // design, and xtask is not the tool -- with zero ceilings against a
    // baseline of what was already duplicated when the gate arrived.
    let dry = DryGate::excluding(
        &runner,
        root.to_string_lossy().into_owned(),
        DRY_EXCLUDES
            .iter()
            .map(|exclude| below(&root, exclude).to_string_lossy().into_owned())
            .collect(),
        root.join(DRY_BASELINE).to_string_lossy().into_owned(),
        String::from(DRY_MIN_NODES),
    );

    let crap = CrapGate::new(
        &runner,
        &parser,
        manifest_path.clone(),
        packages.clone(),
        String::from(CRAP_THRESHOLD),
    );

    // Every source file has its mirrored test file.
    let twin = TwinGate::new(&runner, manifest_path.clone(), packages.clone());

    // A ratchet just above the worst file -- this one: 6.89 when the gates
    // arrived, 6.76 since it took them from serge4rust --, lowered as it
    // improves and never raised.
    let iceberg = IcebergGate::new(
        &runner,
        manifest_path.clone(),
        packages,
        String::from(ICEBERG_THRESHOLD),
    );

    // Last in stage 2: its advisories change without the code changing (see
    // serge4rust's DenyGate).
    let deny = DenyGate::new(&runner, manifest_path, DENY_FETCH_ATTEMPTS);

    let gates: Vec<&dyn Gate> = vec![
        &boundary, &stern, &shear, &dry, &crap, &twin, &iceberg, &deny,
    ];

    match Stage::new(gates).run() {
        Ok(()) => {
            println!("\nripe4rust Stage 2 passed!");
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("\nFailed: {reason}");
            ExitCode::FAILURE
        }
    }
}

fn workspace_manifest_path() -> String {
    workspace_root()
        .join("Cargo.toml")
        .to_string_lossy()
        .into_owned()
}

// Joined a component at a time, so the path carries the platform's separator
// throughout -- dry4rust matches its exclusions as text.
fn below(root: &Path, components: &[&str]) -> PathBuf {
    components
        .iter()
        .fold(root.to_path_buf(), |path, component| path.join(component))
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask lives one directory below the workspace root")
        .to_path_buf()
}
