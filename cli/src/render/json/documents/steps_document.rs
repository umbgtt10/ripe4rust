// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::revert_document::RevertDocument;
use crate::render::json::documents::split_document::SplitDocument;
use crate::render::json::documents::step_document::StepDocument;
use serde::Serialize;

/// A unit's steps, in the design's order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct StepsDocument<'a> {
    pub apply: Vec<StepDocument<'a>>,
    pub check: &'a str,
    pub gates: &'a [String],
    pub revert: RevertDocument<'a>,
    pub split: Vec<SplitDocument<'a>>,
}
