// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::date::Date;

/// Today's date, injected so a plan is reproducible.
pub trait Clock {
    fn today(&self) -> Date;
}
