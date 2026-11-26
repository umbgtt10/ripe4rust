// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::values::version::Version;

pub fn added(name: &str, to: &str) -> LockMove {
    LockMove {
        name: String::from(name),
        from: None,
        to: Some(Version::parse(to).expect("a version")),
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
