// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

const STEPS: [&str; 14] = [
    "1. Run `cargo ripe4rust plan --format json > plan.json`.",
    "   - Exit `2`: carry out each error's `do`; one that `needs_person` goes to the person. Then plan again.",
    "   - A plan reads the registry's answers and the Rust distribution's list of releases from the day's cache, so one made later the same day does not see a release published since the day's first plan. Where one came out since -- a crate's, or Rust's --, plan with `--refresh`: every answer is fetched again and the day's copy overwritten.",
    "2. Take the urgent units first, then the rest, in plan order.",
    "3. For each ripe unit, one at a time:",
    "   1. carry out `apply`;",
    "   2. run `check`:",
    "      - `0`: run `gates` in order. All green: commit, one commit per unit, `summary` as its subject in the workspace's own conventions, its message naming the dependencies `repointed` lists that the check before it did not, and those it no longer lists -- the unit re-pointed them, between versions the lock already held, and the gates are what judge it. One red: carry out `revert`; a compatible unit of several moves is bisected through `split` (apply, check, gates each), its green moves committed; hold the one that failed -- or the unit, if it has no split -- with `hold --because \"<the failing gate and its first error>\"`, add the pin, commit it. A breaking unit that fails becomes ask, the failure attached; so does an urgent unit, still urgent -- it is never held, as holding it would keep what it clears.",
    "      - `1`: carry out `revert`; the unit becomes ask, with `check`'s differences attached.",
    "      - `2`: carry out each error's `do`, then run `check` again.",
    "      - `3`: plan again, and go on with the new plan.",
    "4. Report every ripening unit, with what will release it.",
    "5. Put every ask unit's question to the person in one message. Carry out each answer's `then`.",
    "6. The round is done when every unit is committed, held, answered or ripening. A last `plan` shows only those held, asked or ripening.",
];

/// The round, as the design gives it: what the agent does, every time, a
/// step a line, a sub-step indented under its step.
pub struct RoundGuide;

impl RoundGuide {
    pub fn steps(&self) -> Vec<String> {
        STEPS.iter().map(|step| String::from(*step)).collect()
    }
}
