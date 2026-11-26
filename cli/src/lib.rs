// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

//! The `cargo-ripe4rust` command's own code: the real implementations of
//! the core's ports, the seams they reach the system through, and the
//! renderers.

pub mod cargo;
pub mod render;
pub mod seams;
pub mod system;
pub mod traits;
