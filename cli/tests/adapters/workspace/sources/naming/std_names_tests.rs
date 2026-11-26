// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::workspace::sources::naming::std_names::StdNames;

#[test]
fn attribute_of_a_built_in_is_true_and_of_a_crate_s_false() {
    // Arrange & Act
    let built_in = [StdNames.attribute("test"), StdNames.attribute("serde")];

    // Assert
    assert_eq!(built_in, [true, false]);
}

#[test]
fn derive_of_a_built_in_is_true_and_of_a_crate_s_false() {
    // Arrange & Act
    let built_in = [StdNames.derive("Debug"), StdNames.derive("Serialize")];

    // Assert
    assert_eq!(built_in, [true, false]);
}

#[test]
fn macro_name_of_the_standard_library_s_is_true_and_of_a_crate_s_false() {
    // Arrange & Act
    let built_in = [StdNames.macro_name("println"), StdNames.macro_name("info")];

    // Assert
    assert_eq!(built_in, [true, false]);
}

#[test]
fn prelude_of_a_prelude_name_a_primitive_and_std_is_true_and_of_a_crate_s_false() {
    // Arrange & Act
    let prelude = ["Option", "usize", "std", "Either"].map(|name| StdNames.prelude(name));

    // Assert
    assert_eq!(prelude, [true, true, true, false]);
}
