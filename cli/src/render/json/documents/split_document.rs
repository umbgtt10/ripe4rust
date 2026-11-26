// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::step_document::StepDocument;
use serde::Serialize;

/// One move of a compatible unit, applied alone.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SplitDocument<'a> {
    pub id: &'a str,
    pub apply: Vec<StepDocument<'a>>,
}
