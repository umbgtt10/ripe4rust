// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

pub fn curl(url: &str) -> Vec<String> {
    [
        "curl",
        "--silent",
        "--show-error",
        "--location",
        "--max-time",
        "60",
        "--user-agent",
        &format!(
            "cargo-ripe4rust/{} (+https://github.com/umbgtt10/ripe4rust)",
            env!("CARGO_PKG_VERSION")
        ),
        "--write-out",
        "\\n%{http_code}",
        url,
    ]
    .map(String::from)
    .to_vec()
}
