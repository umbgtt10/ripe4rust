// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::version;
use ripe4rust_core::inputs::git::git_source::GitSource;
use ripe4rust_core::inputs::lock_move::LockMove;
use ripe4rust_core::planning::units::direct_dependency::DirectDependency;
use ripe4rust_core::planning::units::unit::Unit;
use ripe4rust_core::planning::units::unit_id::UnitId;
use ripe4rust_core::unit_kind::UnitKind;

pub const SOURCE: &str = "https://github.com/example/upstream?branch=main";

pub fn added(name: &str, to: &str) -> LockMove {
    LockMove {
        name: String::from(name),
        from: None,
        to: Some(version(to)),
    }
}

pub fn breaking(name: &str, from: &str, to: &str) -> Unit {
    Unit {
        id: UnitId::of(UnitKind::Breaking, name),
        kind: UnitKind::Breaking,
        name: Some(String::from(name)),
        members: vec![String::from(name)],
        from: Some(version(from)),
        to: Some(version(to)),
        moves: vec![moved(name, from, to)],
        held: Vec::new(),
        git: None,
    }
}

pub fn compatible(name: &str, moves: Vec<LockMove>) -> Unit {
    let own = moves.iter().find(|each| each.name == name).cloned();
    Unit {
        id: UnitId::of(UnitKind::Compatible, name),
        kind: UnitKind::Compatible,
        name: Some(String::from(name)),
        members: vec![String::from(name)],
        from: own.as_ref().and_then(|each| each.from.clone()),
        to: own.and_then(|each| each.to),
        moves,
        held: Vec::new(),
        git: None,
    }
}

pub fn dependency(name: &str, in_use: &str) -> DirectDependency {
    DirectDependency {
        name: String::from(name),
        version: version(in_use),
        members: vec![String::from("app")],
    }
}

pub fn git(name: &str, members: &[&str], moves: Vec<LockMove>) -> Unit {
    Unit {
        id: UnitId::of(UnitKind::Git, name),
        kind: UnitKind::Git,
        name: Some(String::from(name)),
        members: members.iter().map(|member| String::from(*member)).collect(),
        from: None,
        to: None,
        moves,
        held: Vec::new(),
        git: Some(source("9625c3d2", "ce129c85")),
    }
}

pub fn moved(name: &str, from: &str, to: &str) -> LockMove {
    LockMove {
        name: String::from(name),
        from: Some(version(from)),
        to: Some(version(to)),
    }
}

pub fn removed(name: &str, from: &str) -> LockMove {
    LockMove {
        name: String::from(name),
        from: Some(version(from)),
        to: None,
    }
}

pub fn source(from: &str, to: &str) -> GitSource {
    GitSource {
        source: String::from(SOURCE),
        from: String::from(from),
        to: String::from(to),
    }
}
