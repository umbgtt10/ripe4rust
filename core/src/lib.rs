// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

//! The rules behind `cargo-ripe4rust`: for every upgrade a Rust workspace
//! could take, whether it is ripe, worth asking about, or not yet. See
//! <https://github.com/umbgtt10/ripe4rust/blob/main/docs/DESIGN.md>.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod diagnostics;
pub mod inputs;
pub mod outcome;
pub mod planning;
pub mod unit_kind;
pub mod values;
