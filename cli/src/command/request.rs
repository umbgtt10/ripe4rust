// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command::format::Format;
use ripe4rust_core::values::date::Date;

/// What the command line asks for: the command, the format, the day that
/// stands for today, and whether to read the cache alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub command: String,
    pub format: Format,
    pub date: Option<Date>,
    pub offline: bool,
}
