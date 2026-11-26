// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::diagnostics::diagnostic::Diagnostic;
use alloc::vec::Vec;

/// What a reader made of its part of the policy, and the problems it met.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reading<T> {
    pub value: T,
    pub diagnostics: Vec<Diagnostic>,
}
