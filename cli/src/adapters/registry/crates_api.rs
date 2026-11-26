// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::registry::api_reader::ApiReader;
use crate::adapters::registry::cached_answers::CachedAnswers;
use crate::system::traits::answer_cache::AnswerCache;
use crate::system::traits::http::Http;
use crate::system::traits::pause::Pause;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::traits::repositories::Repositories;
use std::cell::Cell;

const API: &str = "api";
const BASE: &str = "https://crates.io/api/v1/crates";
const SECONDS_BETWEEN: u64 = 1;

/// The crates.io API, asked only for a direct dependency's repository and
/// at most once a second: a pause before every request after the first, and
/// none for an answer cached for the day. `?include=` asks for the crate
/// alone, without its versions.
pub struct CratesApi<'a> {
    http: &'a dyn Http,
    cache: &'a dyn AnswerCache,
    pause: &'a dyn Pause,
    asked: Cell<bool>,
}

impl<'a> CratesApi<'a> {
    pub fn new(http: &'a dyn Http, cache: &'a dyn AnswerCache, pause: &'a dyn Pause) -> Self {
        Self {
            http,
            cache,
            pause,
            asked: Cell::new(false),
        }
    }
}

impl Repositories for CratesApi<'_> {
    fn repository(&self, name: &str) -> Result<Option<String>, PortFailure> {
        let answers = CachedAnswers::new(self.http, self.cache);
        let body = if let Some(body) = answers.cached(API, name)? {
            body
        } else {
            if self.asked.replace(true) {
                self.pause.pause(SECONDS_BETWEEN);
            }
            answers.fetched(API, name, &format!("{BASE}/{name}?include="), "the API")?
        };
        ApiReader.repository(name, &body)
    }
}
