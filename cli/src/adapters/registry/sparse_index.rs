// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::registry::cached_answers::CachedAnswers;
use crate::adapters::registry::index_reader::IndexReader;
use crate::adapters::unreadable::Unreadable;
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
/// file at the path its name's length gives it, those the day lacks fetched
/// together, answered in the order asked.
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

    fn releases(&self, names: &[String]) -> Result<Vec<(String, Vec<Release>)>, PortFailure> {
        let answers = CachedAnswers::new(self.http, self.cache);
        let cached = names
            .iter()
            .map(|name| answers.cached(INDEX, name))
            .collect::<Result<Vec<Option<String>>, PortFailure>>()?;
        let missing = names
            .iter()
            .zip(&cached)
            .filter(|(_, body)| body.is_none())
            .map(|(name, _)| self.url(name))
            .collect::<Vec<String>>();
        let mut fetched = self.http.get_all(&missing).into_iter();
        names
            .iter()
            .zip(cached)
            .map(|(name, body)| {
                let body = if let Some(body) = body {
                    body
                } else {
                    let url = self.url(name);
                    let answer = fetched.next().unwrap_or_else(|| {
                        Err(Unreadable::new("fetching the index")
                            .failure(&format!("no answer for `{url}`")))
                    });
                    answers.kept(INDEX, name, &url, "the index", answer)?
                };
                Ok((name.clone(), IndexReader.releases(name, &body)?))
            })
            .collect()
    }
}
