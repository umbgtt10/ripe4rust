// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::sources::parsed;
use cargo_ripe4rust::adapters::workspace::sources::naming::span_line::SpanLine;
use syn::Item;

#[test]
fn of_a_span_is_the_line_it_starts_on_counted_from_one() {
    // Arrange
    let file = parsed("fn first() {}\n\nfn third() {}\n");
    let Some(Item::Fn(third)) = file.items.get(1) else {
        panic!("a function");
    };

    // Act
    let line = SpanLine.of(third.sig.ident.span());

    // Assert
    assert_eq!(line, 3);
}
