// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::releases::date;
use ripe4rust_core::planning::judging::footprint::Footprint;
use ripe4rust_core::planning::judging::line_standing::LineStanding;
use ripe4rust_core::planning::judging::pin_standing::PinStanding;
use ripe4rust_core::planning::judging::unit_facts::UnitFacts;
use ripe4rust_core::planning::pins_and_policy::hold::Hold;
use ripe4rust_core::planning::pins_and_policy::pin::Pin;
use ripe4rust_core::planning::units::choice::line_stats::LineStats;
use ripe4rust_core::unit_kind::UnitKind;
use ripe4rust_core::values::line::Line;
use ripe4rust_core::values::location::Location;

pub fn facts(kind: UnitKind) -> UnitFacts {
    UnitFacts {
        kind,
        name: String::from("widget"),
        rest: None,
        concerns: Vec::new(),
        downgrades: Vec::new(),
        reaches: Vec::new(),
        pins: Vec::new(),
        fully_held: false,
        freeze: None,
        exposed_in: Vec::new(),
        exposure_unknown_in: Vec::new(),
        raises: Vec::new(),
        in_use_line: Some(standing(1, false, true)),
        new_line: Some(standing(2, false, true)),
        newer_active: true,
        lines_behind: 1,
        footprint: Some(Footprint {
            lines: 42,
            files: 7,
            complete: true,
        }),
        removes_copy: false,
        held_by: Vec::new(),
        held_back: 0,
        gates: 2,
        small_footprint: 10,
        moderate_footprint: 100,
    }
}

pub fn pinned(released: Option<&str>) -> PinStanding {
    PinStanding {
        pin: Pin {
            name: String::from("widget"),
            reason: String::from("a migration of every visitor"),
            review: Some(date("2027-01-31")),
            until: None,
            hold: Hold::Line,
            at: Location {
                file: String::from("Cargo.toml"),
                line: 41,
            },
        },
        released: released.map(String::from),
    }
}

pub fn standing(major: u64, young: bool, active: bool) -> LineStanding {
    LineStanding {
        stats: LineStats {
            line: Line { major, minor: None },
            first_release: Some(date("2026-11-20")),
            last_release: Some(date("2026-11-20")),
            releases: 1,
        },
        young,
        active,
    }
}
