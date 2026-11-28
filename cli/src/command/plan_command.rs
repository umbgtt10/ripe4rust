// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::cargo_metadata::CargoMetadata;
use crate::adapters::cargo::cargo_update::CargoUpdate;
use crate::adapters::cargo::deny::cargo_deny::CargoDeny;
use crate::adapters::clock::fixed_clock::FixedClock;
use crate::adapters::registry::crates_api::CratesApi;
use crate::adapters::registry::curl::Curl;
use crate::adapters::registry::day_cache::DayCache;
use crate::adapters::registry::lookup::Lookup;
use crate::adapters::registry::sparse_index::SparseIndex;
use crate::adapters::toolchain::channel_manifests::ChannelManifests;
use crate::adapters::toolchain::rustc_toolchain::RustcToolchain;
use crate::adapters::workspace::cargo_layout::CargoLayout;
use crate::adapters::workspace::manifest_files::ManifestFiles;
use crate::adapters::workspace::sources::source_scanner::SourceScanner;
use crate::command::request::Request;
use crate::command::seams::Seams;
use ripe4rust_core::planning::plan::Plan;
use ripe4rust_core::planning::plan_failure::PlanFailure;
use ripe4rust_core::planning::planner::Planner;
use ripe4rust_core::planning::ports::Ports;
use ripe4rust_core::traits::clock::Clock;

/// `plan`: every port's adapter set up over the run's seams -- the day
/// `--date` names standing for today, `--offline` reaching every adapter
/// that could reach the network, `--refresh` the day cache, which then reads
/// no answer -- and the core's planner run over them.
pub struct PlanCommand<'a> {
    seams: Seams<'a>,
}

impl<'a> PlanCommand<'a> {
    pub fn new(seams: Seams<'a>) -> Self {
        Self { seams }
    }

    pub fn plan(&self, request: &Request) -> Result<Plan, PlanFailure> {
        let seams = &self.seams;
        let (dir, offline) = (seams.dir.clone(), request.lookup == Lookup::Offline);
        let asked = request.date.map(FixedClock::new);
        let clock: &dyn Clock = asked.as_ref().map_or(seams.clock, |asked| asked);
        let cache = DayCache::new(
            seams.files,
            seams.cache.clone(),
            clock.today(),
            request.lookup,
        );
        let curl = Curl::new(seams.runner, dir.clone());
        let manifests = ChannelManifests::new(&curl, &cache);
        let layout = CargoLayout::new(seams.runner, dir.clone());
        Planner::new(Ports {
            clock,
            index: &SparseIndex::new(&curl, &cache),
            repositories: &CratesApi::new(&curl, &cache, seams.pause),
            lock_moves: &CargoUpdate::new(seams.runner, dir.clone(), offline),
            metadata: &CargoMetadata::new(seams.runner, dir.clone(), offline),
            files: &ManifestFiles::new(&layout, seams.files),
            advisories: &CargoDeny::new(seams.runner, dir.clone(), offline),
            toolchain: &RustcToolchain::new(seams.runner, dir, &manifests),
            scan: &SourceScanner::new(&layout, seams.files),
        })
        .plan()
    }
}
