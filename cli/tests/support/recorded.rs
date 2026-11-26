// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::outputs::printed;
use crate::support::registry::curl;
use crate::support::replay_runner::ReplayRunner;
use crate::support::workspace::LAYOUT;
use cargo_ripe4rust::system::model::command_output::CommandOutput;

const CRATES: [(&str, &str); 4] = [
    ("itertools", "0.10.0"),
    ("itoa", "0.4.7"),
    ("memchr", "2.5.0"),
    ("smallvec", "1.6.0"),
];

fn api(name: &str) -> &'static str {
    match name {
        "itertools" => include_str!("../fixtures/workspace/registry/api/itertools.txt"),
        "itoa" => include_str!("../fixtures/workspace/registry/api/itoa.txt"),
        "memchr" => include_str!("../fixtures/workspace/registry/api/memchr.txt"),
        _ => include_str!("../fixtures/workspace/registry/api/smallvec.txt"),
    }
}

fn cargo(offline: bool, words: &[&str]) -> Vec<String> {
    let mut command = vec![String::from("cargo")];
    let mut words = words.iter().map(|word| String::from(*word));
    command.extend(words.by_ref().take(2));
    if offline {
        command.push(String::from("--offline"));
    }
    command.extend(words);
    command
}

fn deny(offline: bool) -> Vec<String> {
    let mut command = ["cargo", "deny", "--format", "json", "--locked"]
        .map(String::from)
        .to_vec();
    if offline {
        command.push(String::from("--offline"));
    }
    command.extend(
        [
            "--workspace",
            "check",
            "--audit-compatible-output",
            "advisories",
        ]
        .map(String::from),
    );
    command
}

fn index(name: &str) -> &'static str {
    match name {
        "either" => include_str!("../fixtures/workspace/registry/index/either.txt"),
        "itertools" => include_str!("../fixtures/workspace/registry/index/itertools.txt"),
        "itoa" => include_str!("../fixtures/workspace/registry/index/itoa.txt"),
        "memchr" => include_str!("../fixtures/workspace/registry/index/memchr.txt"),
        _ => include_str!("../fixtures/workspace/registry/index/smallvec.txt"),
    }
}

/// Every command a plan of the recorded workspace runs, answered as it was
/// recorded; offline, the cargo commands carry `--offline` and no request
/// reaches the registry or the Rust distribution.
pub fn recorded(offline: bool) -> ReplayRunner {
    let mut runner = ReplayRunner::new()
        .reply(
            &LAYOUT,
            printed(
                include_str!("../fixtures/workspace/cargo/metadata-no-deps.json"),
                "",
            ),
        )
        .reply(
            &["rustc", "--print", "target-list"],
            printed(
                include_str!("../fixtures/workspace/rustc/target-list.txt"),
                "",
            ),
        )
        .reply(
            &[
                ["cargo", "metadata", "--format-version", "1", "--locked"].as_slice(),
                if offline { &["--offline"] } else { &[] },
            ]
            .concat(),
            printed(
                include_str!("../fixtures/workspace/cargo/metadata.json"),
                "",
            ),
        )
        .reply(
            &[
                ["cargo", "metadata", "--format-version", "1", "--locked"].as_slice(),
                if offline { &["--offline"] } else { &[] },
                &["--filter-platform", "thumbv7em-none-eabihf"],
            ]
            .concat(),
            printed(
                include_str!("../fixtures/workspace/cargo/metadata-thumbv7em-none-eabihf.json"),
                "",
            ),
        )
        .reply(
            &deny(offline),
            CommandOutput {
                status: Some(1),
                stdout: String::from(include_str!("../fixtures/workspace/cargo-deny/audit.json")),
                stderr: String::from(include_str!("../fixtures/workspace/cargo-deny/log.jsonl")),
            },
        )
        .reply(
            &cargo(offline, &["update", "--dry-run"]),
            printed("", include_str!("../fixtures/workspace/cargo/update.txt")),
        )
        .reply(
            &curl("https://static.rust-lang.org/manifests.txt"),
            printed(
                include_str!("../fixtures/workspace/toolchain/manifests.txt"),
                "",
            ),
        );
    for (name, version) in CRATES {
        let spec = format!("{name}@{version}");
        runner = runner.reply(
            &cargo(
                offline,
                &["update", "--dry-run", "--recursive", "-p", &spec],
            ),
            printed("", updates(name)),
        );
        runner = runner.reply(
            &curl(&format!("https://crates.io/api/v1/crates/{name}?include=")),
            printed(api(name), ""),
        );
    }
    for (path, name) in [
        ("ei/th", "either"),
        ("it/er", "itertools"),
        ("it/oa", "itoa"),
        ("me/mc", "memchr"),
        ("sm/al", "smallvec"),
    ] {
        runner = runner.reply(
            &curl(&format!("https://index.crates.io/{path}/{name}")),
            printed(index(name), ""),
        );
    }
    runner
}

fn updates(name: &str) -> &'static str {
    match name {
        "itertools" => include_str!("../fixtures/workspace/cargo/update-itertools.txt"),
        "itoa" => include_str!("../fixtures/workspace/cargo/update-itoa.txt"),
        "memchr" => include_str!("../fixtures/workspace/cargo/update-memchr.txt"),
        _ => include_str!("../fixtures/workspace/cargo/update-smallvec.txt"),
    }
}
