// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::toolchain::manifest_list::ManifestList;
use crate::system::traits::answer_cache::AnswerCache;
use crate::system::traits::http::Http;
use crate::system::traits::release_source::ReleaseSource;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::release::Release;

const FOUND: u16 = 200;
const KIND: &str = "toolchain";
const NAME: &str = "manifests.txt";
const URL: &str = "https://static.rust-lang.org/manifests.txt";

/// The toolchain's stable releases, from the list of every channel manifest
/// the Rust distribution published, kept in the day cache beside the
/// registry's answers: read from the cache, or fetched and kept there once
/// it reads. Any status but 200, curl failing, or nothing cached `--offline`,
/// is the toolchain unreachable.
pub struct ChannelManifests<'a> {
    http: &'a dyn Http,
    cache: &'a dyn AnswerCache,
}

impl<'a> ChannelManifests<'a> {
    pub fn new(http: &'a dyn Http, cache: &'a dyn AnswerCache) -> Self {
        Self { http, cache }
    }

    fn fetched(&self) -> Result<String, PortFailure> {
        let fetched = self.http.get(URL).map_err(Self::unreachable)?;
        if fetched.status == FOUND {
            Ok(fetched.body)
        } else {
            Err(PortFailure::ToolchainUnreachable {
                detail: format!(
                    "the Rust distribution answered {} for `{URL}`",
                    fetched.status
                ),
            })
        }
    }

    fn unreachable(failure: PortFailure) -> PortFailure {
        match failure {
            PortFailure::RegistryUnreachable { detail } => {
                PortFailure::ToolchainUnreachable { detail }
            }
            other => other,
        }
    }
}

impl ReleaseSource for ChannelManifests<'_> {
    fn releases(&self) -> Result<Vec<Release>, PortFailure> {
        if let Some(body) = self.cache.read(KIND, NAME).map_err(Self::unreachable)? {
            return ManifestList.releases(&body);
        }
        let body = self.fetched()?;
        let releases = ManifestList.releases(&body)?;
        self.cache.write(KIND, NAME, &body)?;
        Ok(releases)
    }
}
