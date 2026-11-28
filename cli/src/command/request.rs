// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::registry::lookup::Lookup;
use crate::command::format::Format;
use ripe4rust_core::values::date::Date;

/// What the command line asks for: the command, the format, and what each
/// command takes -- for `plan` the day that stands for today and how to read
/// the day cache, online, refreshed or alone; for `check` and `hold` the unit
/// and the plan file; for `hold` the reason and the review date.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub command: String,
    pub format: Format,
    pub date: Option<Date>,
    pub lookup: Lookup,
    pub unit: Option<String>,
    pub plan: Option<String>,
    pub because: Option<String>,
    pub review: Option<Date>,
}
