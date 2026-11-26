// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use proc_macro2::Span;

/// The line a span of the parse starts on, counted from 1.
pub struct SpanLine;

impl SpanLine {
    pub fn of(&self, span: Span) -> u32 {
        u32::try_from(span.start().line).unwrap_or(u32::MAX)
    }
}
