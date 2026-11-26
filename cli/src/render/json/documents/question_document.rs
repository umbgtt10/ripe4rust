// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::option_document::OptionDocument;
use serde::Serialize;

/// The question of a unit that asks, in the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct QuestionDocument<'a> {
    pub title: &'a str,
    pub why: &'a str,
    pub recommendation: &'a str,
    pub options: Vec<OptionDocument<'a>>,
}
