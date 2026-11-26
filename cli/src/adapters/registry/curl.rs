// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::complaint::Complaint;
use crate::adapters::unreadable::Unreadable;
use crate::system::model::command_output::CommandOutput;
use crate::system::model::fetched::Fetched;
use crate::system::model::invocation::Invocation;
use crate::system::model::launch_failure::LaunchFailure;
use crate::system::traits::command_runner::CommandRunner;
use crate::system::traits::http::Http;
use ripe4rust_core::inputs::port_failure::PortFailure;
use std::path::PathBuf;

const CURL: &str = "curl";
const STATUS: &str = "\\n%{http_code}";
const USER_AGENT: &str = concat!(
    "cargo-ripe4rust/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/umbgtt10/ripe4rust)"
);

/// HTTP through a `curl` process, as ADR-TheRegistryThroughCurl decides: the
/// body and, written after it, the status; a user agent naming the tool, its
/// version and its repository, and nothing else. Several are one batch for
/// the runner, which runs them together.
pub struct Curl<'a> {
    runner: &'a dyn CommandRunner,
    dir: PathBuf,
}

impl<'a> Curl<'a> {
    pub fn new(runner: &'a dyn CommandRunner, dir: PathBuf) -> Self {
        Self { runner, dir }
    }

    fn answer(ran: Result<CommandOutput, LaunchFailure>) -> Result<Fetched, PortFailure> {
        let output = ran.map_err(|failure| match failure {
            LaunchFailure::Missing { .. } => PortFailure::CurlMissing,
            LaunchFailure::Failed { .. } => PortFailure::RegistryUnreachable {
                detail: Complaint.unlaunched(&failure),
            },
        })?;
        if output.status != Some(0) {
            return Err(PortFailure::RegistryUnreachable {
                detail: Complaint.of(&output),
            });
        }
        output
            .stdout
            .rsplit_once('\n')
            .and_then(|(body, status)| {
                Some(Fetched {
                    status: status.trim().parse().ok()?,
                    body: String::from(body),
                })
            })
            .ok_or_else(|| {
                Unreadable::new("reading what curl printed")
                    .failure(&format!("`{}` ends in no HTTP status", output.stdout))
            })
    }

    fn invocation(&self, url: &str) -> Invocation {
        let args = [
            "--silent",
            "--show-error",
            "--location",
            "--max-time",
            "60",
            "--user-agent",
            USER_AGENT,
            "--write-out",
            STATUS,
            url,
        ]
        .map(String::from)
        .to_vec();
        Invocation {
            program: String::from(CURL),
            args,
            dir: self.dir.clone(),
        }
    }
}

impl Http for Curl<'_> {
    fn get(&self, url: &str) -> Result<Fetched, PortFailure> {
        Self::answer(self.runner.run(&self.invocation(url)))
    }

    fn get_all(&self, urls: &[String]) -> Vec<Result<Fetched, PortFailure>> {
        let invocations = urls
            .iter()
            .map(|url| self.invocation(url))
            .collect::<Vec<Invocation>>();
        self.runner
            .run_all(&invocations)
            .into_iter()
            .map(Self::answer)
            .collect()
    }
}
