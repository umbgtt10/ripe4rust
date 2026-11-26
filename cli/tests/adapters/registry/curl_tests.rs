// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::outputs::failed;
use crate::support::outputs::printed;
use crate::support::registry::curl;
use crate::support::replay_runner::ReplayRunner;
use cargo_ripe4rust::adapters::registry::curl::Curl;
use cargo_ripe4rust::system::model::fetched::Fetched;
use cargo_ripe4rust::system::model::launch_failure::LaunchFailure;
use cargo_ripe4rust::system::traits::http::Http;
use ripe4rust_core::inputs::port_failure::PortFailure;
use std::path::PathBuf;

const URL: &str = "https://index.crates.io/sm/al/smallvec";

fn root() -> PathBuf {
    PathBuf::from("/work/ws")
}

#[test]
fn get_all_of_several_urls_asks_for_them_together_answering_each_in_their_order() {
    // Arrange
    let missing = "https://index.crates.io/it/oa/itoa";
    let runner = ReplayRunner::new()
        .reply(&curl(URL), printed("an answer\n200", ""))
        .reply(&curl(missing), printed("not here\n404", ""));

    // Act
    let fetched = Curl::new(&runner, root()).get_all(&[String::from(URL), String::from(missing)]);

    // Assert
    assert_eq!(
        fetched,
        vec![
            Ok(Fetched {
                status: 200,
                body: String::from("an answer"),
            }),
            Ok(Fetched {
                status: 404,
                body: String::from("not here"),
            }),
        ]
    );
    assert_eq!(*runner.batches.borrow(), vec![2]);
}

#[test]
fn get_of_a_crate_the_index_does_not_hold_is_its_404() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &curl(URL),
        printed(
            include_str!("../../fixtures/registry/index-missing.txt"),
            "",
        ),
    );

    // Act
    let fetched = Curl::new(&runner, root()).get(URL);

    // Assert
    assert_eq!(fetched.map(|fetched| fetched.status), Ok(404));
}

#[test]
fn get_of_a_recorded_answer_is_its_status_and_body() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &curl(URL),
        printed(
            include_str!("../../fixtures/workspace/registry/index/smallvec.txt"),
            "",
        ),
    );

    // Act
    let fetched = Curl::new(&runner, root()).get(URL).expect("an answer");

    // Assert
    assert_eq!(fetched.status, 200);
    assert!(
        fetched
            .body
            .starts_with(r#"{"name":"smallvec","vers":"0.1.0""#),
        "{}",
        &fetched.body[..80]
    );
    assert!(
        fetched
            .body
            .ends_with("\"pubtime\":\"2026-11-18T08:09:40Z\"}\n")
    );
}

#[test]
fn get_of_an_answer_with_no_status_is_internal() {
    // Arrange
    let runner = ReplayRunner::new().reply(&curl(URL), printed("{}", ""));

    // Act
    let fetched = Curl::new(&runner, root()).get(URL);

    // Assert
    assert_eq!(
        fetched,
        Err(PortFailure::Internal {
            doing: String::from("reading what curl printed"),
            detail: String::from("`{}` ends in no HTTP status"),
        })
    );
}

#[test]
fn get_of_an_unreachable_host_says_what_curl_said() {
    // Arrange
    let runner = ReplayRunner::new().reply(
        &curl(URL),
        failed(
            6,
            include_str!("../../fixtures/registry/index-unreachable.txt"),
        ),
    );

    // Act
    let fetched = Curl::new(&runner, root()).get(URL);

    // Assert
    assert_eq!(
        fetched,
        Err(PortFailure::RegistryUnreachable {
            detail: String::from("curl: (6) Could not resolve host: index.invalid"),
        })
    );
}

#[test]
fn get_with_curl_refused_says_what_the_system_said() {
    // Arrange
    let runner = ReplayRunner::new().refuse(
        &curl(URL),
        LaunchFailure::Failed {
            program: String::from("curl"),
            detail: String::from("permission denied"),
        },
    );

    // Act
    let fetched = Curl::new(&runner, root()).get(URL);

    // Assert
    assert_eq!(
        fetched,
        Err(PortFailure::RegistryUnreachable {
            detail: String::from("`curl` could not be started: permission denied"),
        })
    );
}

#[test]
fn get_without_curl_is_curl_missing() {
    // Arrange
    let runner = ReplayRunner::new().refuse(
        &curl(URL),
        LaunchFailure::Missing {
            program: String::from("curl"),
        },
    );

    // Act
    let fetched = Curl::new(&runner, root()).get(URL);

    // Assert
    assert_eq!(fetched, Err(PortFailure::CurlMissing));
}
