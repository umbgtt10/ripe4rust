// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::location::Location;
use alloc::string::String;

/// An `include!` the parse could not follow -- its path no literal, or a
/// macro that includes generated code, as `include_proto!` --, in the member
/// whose sources hold it, where it is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnfollowedInclude {
    pub member: String,
    pub at: Location,
}
