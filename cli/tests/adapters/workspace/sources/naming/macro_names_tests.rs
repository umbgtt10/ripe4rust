// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::lines;
use crate::support::sources::scoped;
use cargo_ripe4rust::adapters::workspace::sources::naming::macro_names::MacroNames;
use syn::Macro;
use syn::parse_str;

fn called(text: &str) -> Macro {
    parse_str(text).expect("a macro call")
}

#[test]
fn of_a_macro_a_macro_use_crate_may_define_is_that_crate_and_its_tokens_names() {
    // Arrange & Act
    let found = scoped("#[macro_use]\nextern crate log;\n", |scope| {
        MacroNames::new(scope).of(&called("info!(\"{}\", url::Url::parse(x))"))
    });

    // Assert
    assert_eq!(lines(&found), ["log:1", "url:1"]);
}

#[test]
fn of_a_standard_macro_is_its_tokens_names_alone() {
    // Arrange & Act
    let found = scoped("use itertools::*;\n", |scope| {
        MacroNames::new(scope).of(&called("println!(\"{}\", itoa::Buffer::new())"))
    });

    // Assert
    assert_eq!(lines(&found), ["itoa:1"]);
}
