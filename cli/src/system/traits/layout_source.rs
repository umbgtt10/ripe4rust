// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::model::layout::Layout;
use ripe4rust_core::inputs::port_failure::PortFailure;

/// Where the workspace's files are: its root and its members' manifests.
pub trait LayoutSource {
    fn layout(&self) -> Result<Layout, PortFailure>;
}
