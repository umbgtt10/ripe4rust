// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use ripe4rust_core::inputs::release::Release;
use ripe4rust_core::values::date::Date;
use ripe4rust_core::values::partial_version::PartialVersion;
use ripe4rust_core::values::version::Version;

pub fn date(text: &str) -> Date {
    Date::parse(text).expect("a date")
}

pub fn needing(text: &str, published: &str, rust: &str) -> Release {
    Release {
        rust_version: Some(PartialVersion::parse(rust).expect("a rust-version")),
        ..release(text, published)
    }
}

pub fn release(text: &str, published: &str) -> Release {
    Release {
        version: version(text),
        yanked: false,
        rust_version: None,
        published: Some(date(published)),
    }
}

pub fn version(text: &str) -> Version {
    Version::parse(text).expect("a version")
}

pub fn yanked(text: &str, published: &str) -> Release {
    Release {
        yanked: true,
        ..release(text, published)
    }
}
