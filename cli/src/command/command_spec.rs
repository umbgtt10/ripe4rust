// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

/// What one command of the line takes: its name, the usage its refusals
/// show, the options it knows, whether it takes a unit, and the options it
/// cannot go without.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandSpec {
    pub name: &'static str,
    pub usage: &'static str,
    pub options: &'static [&'static str],
    pub unit: bool,
    pub required: &'static [&'static str],
}
