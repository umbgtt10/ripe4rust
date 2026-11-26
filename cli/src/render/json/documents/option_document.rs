// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// One answer to a question: what it means and the step that carries it out.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct OptionDocument<'a> {
    pub answer: &'a str,
    pub means: &'a str,
    pub then: &'a str,
}
