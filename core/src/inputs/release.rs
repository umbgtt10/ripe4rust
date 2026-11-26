// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::values::date::Date;
use crate::values::partial_version::PartialVersion;
use crate::values::version::Version;

/// One release of a crate as the index records it: `published` is `None`
/// where the index carries no `pubtime`, as for releases older than it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Release {
    pub version: Version,
    pub yanked: bool,
    pub rust_version: Option<PartialVersion>,
    pub published: Option<Date>,
}
