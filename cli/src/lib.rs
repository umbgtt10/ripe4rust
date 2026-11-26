// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

//! The `cargo-ripe4rust` command's own code: the command line and the
//! entry point `main` calls, the real implementations of the core's ports,
//! the seams they reach the system through, and the renderers.

pub mod adapters;
pub mod command;
pub mod render;
pub mod system;
