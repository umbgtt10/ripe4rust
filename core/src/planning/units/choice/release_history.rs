// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::inputs::release::Release;
use crate::planning::units::choice::line_stats::LineStats;
use crate::values::date::Date;
use crate::values::line::Line;
use crate::values::partial_version::PartialVersion;
use crate::values::requirement::Requirement;
use crate::values::version::Version;
use alloc::vec::Vec;

/// Every release of one crate, or of the toolchain, oldest first, and what
/// the version choice asks of them. A candidate is neither yanked nor a
/// pre-release, and the toolchain in use can build it.
pub struct ReleaseHistory {
    releases: Vec<Release>,
    line_of: fn(&Version) -> Line,
}

impl ReleaseHistory {
    pub fn new(releases: Vec<Release>, line_of: fn(&Version) -> Line) -> Self {
        let mut releases = releases;
        releases.sort_by(|left, right| left.version.cmp(&right.version));
        Self { releases, line_of }
    }

    pub fn candidates(&self, toolchain: &Version) -> Vec<Version> {
        self.releases
            .iter()
            .filter(|release| Self::stable(release) && Self::buildable(release, toolchain))
            .map(|release| release.version.clone())
            .collect()
    }

    pub fn held_back(&self, toolchain: &Version, after: &Version, upto: &Version) -> Vec<Version> {
        self.releases
            .iter()
            .filter(|release| {
                Self::stable(release)
                    && release.version > *after
                    && !Self::buildable(release, toolchain)
                    && Self::buildable(release, upto)
            })
            .map(|release| release.version.clone())
            .collect()
    }

    pub fn is_yanked(&self, version: &Version) -> bool {
        self.releases
            .iter()
            .any(|release| release.version == *version && release.yanked)
    }

    pub fn line_of(&self, version: &Version) -> Line {
        (self.line_of)(version)
    }

    pub fn line_stats(&self, line: Line) -> LineStats {
        let counted = self
            .releases
            .iter()
            .filter(|release| Self::stable(release) && self.line_of(&release.version) == line);
        let dates = counted
            .clone()
            .filter_map(|release| release.published)
            .collect::<Vec<Date>>();
        LineStats {
            line,
            first_release: dates.iter().min().copied(),
            last_release: dates.iter().max().copied(),
            releases: u32::try_from(counted.count()).unwrap_or(u32::MAX),
        }
    }

    pub fn newer_lines(&self, line: Line, toolchain: &Version) -> Vec<Line> {
        let mut lines = self
            .candidates(toolchain)
            .iter()
            .map(|version| self.line_of(version))
            .filter(|candidate| *candidate > line)
            .collect::<Vec<Line>>();
        lines.dedup();
        lines
    }

    pub fn newest_in(&self, line: Line, toolchain: &Version) -> Option<Version> {
        self.candidates(toolchain)
            .into_iter()
            .rev()
            .find(|version| self.line_of(version) == line)
    }

    pub fn newest_matching(&self, requirement: &Requirement) -> Option<Version> {
        self.releases
            .iter()
            .rev()
            .find(|release| !release.yanked && requirement.matches(&release.version))
            .map(|release| release.version.clone())
    }

    pub fn published_on(&self, version: &Version) -> Option<Date> {
        self.find(version).and_then(|release| release.published)
    }

    pub fn rust_version_of(&self, version: &Version) -> Option<PartialVersion> {
        self.find(version)
            .and_then(|release| release.rust_version.clone())
    }

    fn buildable(release: &Release, toolchain: &Version) -> bool {
        release
            .rust_version
            .as_ref()
            .is_none_or(|rust| rust.floor() <= *toolchain)
    }

    fn find(&self, version: &Version) -> Option<&Release> {
        self.releases
            .iter()
            .find(|release| release.version == *version)
    }

    fn stable(release: &Release) -> bool {
        !release.yanked && !release.version.is_pre_release()
    }
}
