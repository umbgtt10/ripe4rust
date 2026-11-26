// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::port_failure::PortFailure;
use crate::inputs::release::Release;
use crate::values::version::Version;
use alloc::string::String;
use alloc::vec::Vec;

/// The toolchain: the stable releases and their dates -- `None` where they
/// are not read yet --, the `rustc` in use where no channel pins one, and the
/// targets it knows.
pub trait Toolchain {
    fn releases(&self) -> Result<Option<Vec<Release>>, PortFailure>;
    fn rustc(&self) -> Result<Version, PortFailure>;
    fn targets(&self) -> Result<Vec<String>, PortFailure>;
}
