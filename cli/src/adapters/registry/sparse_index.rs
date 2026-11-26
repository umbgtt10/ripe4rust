// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::registry::cached_answers::CachedAnswers;
use crate::adapters::registry::index_reader::IndexReader;
use crate::system::traits::answer_cache::AnswerCache;
use crate::system::traits::http::Http;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::release::Release;
use ripe4rust_core::traits::index::Index;
use ripe4rust_core::values::date::Date;
use std::ops::Range;

const INDEX: &str = "index";
const BASE: &str = "https://index.crates.io";

/// The crates.io sparse index, every answer cached for the day: a crate's
/// file at the path its name's length gives it.
pub struct SparseIndex<'a> {
    http: &'a dyn Http,
    cache: &'a dyn AnswerCache,
}

impl<'a> SparseIndex<'a> {
    pub fn new(http: &'a dyn Http, cache: &'a dyn AnswerCache) -> Self {
        Self { http, cache }
    }

    pub fn url(&self, name: &str) -> String {
        let name = name.to_lowercase();
        let part = |range: Range<usize>| name.get(range).unwrap_or_default();
        match name.len() {
            1 => format!("{BASE}/1/{name}"),
            2 => format!("{BASE}/2/{name}"),
            3 => format!("{BASE}/3/{}/{name}", part(0..1)),
            _ => format!("{BASE}/{}/{}/{name}", part(0..2), part(2..4)),
        }
    }
}

impl Index for SparseIndex<'_> {
    fn read_on(&self) -> Date {
        self.cache.read_on()
    }

    fn releases(&self, name: &str) -> Result<Vec<Release>, PortFailure> {
        let answers = CachedAnswers::new(self.http, self.cache);
        let body = match answers.cached(INDEX, name)? {
            Some(body) => body,
            None => answers.fetched(INDEX, name, &self.url(name), "the index")?,
        };
        IndexReader.releases(name, &body)
    }
}
