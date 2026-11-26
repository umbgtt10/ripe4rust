// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::lock_move::LockMove;
use crate::planning::judging::facts::plan_context::PlanContext;
use crate::planning::judging::raise::Raise;
use crate::planning::units::unit::Unit;
use crate::unit_kind::UnitKind;
use crate::values::partial_version::PartialVersion;
use alloc::string::String;
use alloc::vec::Vec;

/// What a breaking unit means for the workspace's own users: the published
/// members whose public API its crates reach -- by the parse, or where the
/// policy overrides it, by the policy --, those using a crate the policy is
/// silent on while the sources are not parsed, and the members'
/// `rust-version`s its targets need raised.
pub struct Exposure<'a> {
    context: &'a PlanContext<'a>,
}

impl<'a> Exposure<'a> {
    pub fn new(context: &'a PlanContext<'a>) -> Self {
        Self { context }
    }

    pub fn exposed_in(&self, unit: &Unit) -> Vec<String> {
        if unit.kind != UnitKind::Breaking {
            return Vec::new();
        }
        let surveyed = self.context.surveyed();
        let mut exposed = unit
            .members
            .iter()
            .flat_map(|member| {
                let overridden = surveyed
                    .policy
                    .exposure
                    .iter()
                    .find(|exposure| exposure.name == *member);
                match overridden {
                    Some(exposure) if exposure.public => self.users(member),
                    Some(_) => Vec::new(),
                    None => surveyed
                        .scan
                        .iter()
                        .flat_map(|scan| scan.uses.iter())
                        .find(|used| used.name == *member)
                        .map(|used| used.exposed_in.clone())
                        .unwrap_or_default(),
                }
            })
            .filter(|member| surveyed.policy.published.contains(member))
            .collect::<Vec<String>>();
        exposed.sort();
        exposed.dedup();
        exposed
    }

    pub fn raises(&self, unit: &Unit) -> Vec<Raise> {
        let surveyed = self.context.surveyed();
        let mut raises = Vec::new();
        for lock_move in unit
            .moves
            .iter()
            .filter(|_| unit.kind == UnitKind::Breaking)
        {
            let Some(needs) = self.needs(lock_move) else {
                continue;
            };
            let users = self.users(&lock_move.name);
            for member in surveyed
                .workspace
                .members
                .iter()
                .filter(|member| users.contains(&member.name))
            {
                if member
                    .rust_version
                    .as_ref()
                    .is_some_and(|declared| declared.floor() < needs.floor())
                {
                    raises.push(Raise {
                        member: member.name.clone(),
                        to: needs.clone(),
                        published: surveyed.policy.published.contains(&member.name),
                    });
                }
            }
        }
        raises
    }

    pub fn unknown_in(&self, unit: &Unit) -> Vec<String> {
        let surveyed = self.context.surveyed();
        if unit.kind != UnitKind::Breaking || surveyed.scan.is_some() {
            return Vec::new();
        }
        let mut unknown = unit
            .members
            .iter()
            .filter(|member| {
                !surveyed
                    .policy
                    .exposure
                    .iter()
                    .any(|exposure| exposure.name == **member)
            })
            .flat_map(|member| self.users(member))
            .filter(|member| surveyed.policy.published.contains(member))
            .collect::<Vec<String>>();
        unknown.sort();
        unknown.dedup();
        unknown
    }

    fn needs(&self, lock_move: &LockMove) -> Option<PartialVersion> {
        self.context
            .histories()
            .get(&lock_move.name)?
            .rust_version_of(lock_move.to.as_ref()?)
    }

    fn users(&self, name: &str) -> Vec<String> {
        self.context
            .surveyed()
            .direct
            .iter()
            .find(|dependency| dependency.name == name)
            .map(|dependency| dependency.members.clone())
            .unwrap_or_default()
    }
}
