// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::guide::exit_entry_document::ExitEntryDocument;
use crate::render::json::documents::guide::option_entry_document::OptionEntryDocument;
use serde::Serialize;

/// One command of ripe4rust, in the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CommandEntryDocument<'a> {
    pub usage: &'a str,
    pub does: &'a str,
    pub options: Vec<OptionEntryDocument<'a>>,
    pub exits: Vec<ExitEntryDocument<'a>>,
}
