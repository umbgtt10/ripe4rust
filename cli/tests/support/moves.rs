// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::git::git_move::GitMove;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::values::version::Version;

pub fn added(name: &str, to: &str) -> LockMove {
    LockMove {
        name: String::from(name),
        from: None,
        to: Some(Version::parse(to).expect("a version")),
    }
}

pub fn git(name: &str, version: &str, from: Option<&str>, to: Option<&str>) -> GitMove {
    GitMove {
        name: String::from(name),
        source: String::from("https://github.com/example/upstream?branch=main"),
        version: Version::parse(version).expect("a version"),
        from: from.map(String::from),
        to: to.map(String::from),
    }
}

pub fn removed(name: &str, from: &str) -> LockMove {
    LockMove {
        name: String::from(name),
        from: Some(Version::parse(from).expect("a version")),
        to: None,
    }
}

pub fn updated(name: &str, from: &str, to: &str) -> LockMove {
    LockMove {
        name: String::from(name),
        from: Some(Version::parse(from).expect("a version")),
        to: Some(Version::parse(to).expect("a version")),
    }
}
