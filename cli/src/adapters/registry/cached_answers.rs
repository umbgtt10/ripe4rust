// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::model::fetched::Fetched;
use crate::system::traits::answer_cache::AnswerCache;
use crate::system::traits::http::Http;
use ripe4rust_core::inputs::port_failure::PortFailure;

const FOUND: u16 = 200;
const NOT_FOUND: u16 = 404;

/// The registry's answers, cached for the day: one read from the cache, or
/// fetched -- here, or together elsewhere -- and kept there, a 404 kept as an
/// empty answer, so an offline plan reads the same, and any other status the
/// registry unreachable.
pub struct CachedAnswers<'a> {
    http: &'a dyn Http,
    cache: &'a dyn AnswerCache,
}

impl<'a> CachedAnswers<'a> {
    pub fn new(http: &'a dyn Http, cache: &'a dyn AnswerCache) -> Self {
        Self { http, cache }
    }

    pub fn cached(&self, kind: &str, name: &str) -> Result<Option<String>, PortFailure> {
        self.cache.read(kind, name)
    }

    pub fn fetched(
        &self,
        kind: &str,
        name: &str,
        url: &str,
        answering: &str,
    ) -> Result<String, PortFailure> {
        self.kept(kind, name, url, answering, self.http.get(url))
    }

    pub fn kept(
        &self,
        kind: &str,
        name: &str,
        url: &str,
        answering: &str,
        fetched: Result<Fetched, PortFailure>,
    ) -> Result<String, PortFailure> {
        let fetched = fetched?;
        let body = match fetched.status {
            FOUND => fetched.body,
            NOT_FOUND => String::new(),
            status => {
                return Err(PortFailure::RegistryUnreachable {
                    detail: format!("{answering} answered {status} for `{url}`"),
                });
            }
        };
        self.cache.write(kind, name, &body)?;
        Ok(body)
    }
}
