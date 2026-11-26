// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::model::file_failure::FileFailure;
use crate::system::traits::answer_cache::AnswerCache;
use crate::system::traits::file_system::FileSystem;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::values::date::Date;
use std::cell::OnceCell;
use std::path::PathBuf;

const NO_DIRECTORY: &str = "there is no cache directory: neither `XDG_CACHE_HOME` nor `HOME` is set, nor `LOCALAPPDATA` on Windows";

/// The registry's answers under `<root>/<date>/<kind>/<crate>`, the date
/// the plan's own. Online, a day's answer is read where it is there and
/// fetched where not, and written under the day; `--offline` reads the
/// latest day on or before the plan's, and an answer it lacks stops the
/// plan.
pub struct DayCache<'a> {
    files: &'a dyn FileSystem,
    root: Option<PathBuf>,
    today: Date,
    offline: bool,
    day: OnceCell<Result<Date, PortFailure>>,
}

impl<'a> DayCache<'a> {
    pub fn new(
        files: &'a dyn FileSystem,
        root: Option<PathBuf>,
        today: Date,
        offline: bool,
    ) -> Self {
        Self {
            files,
            root,
            today,
            offline,
            day: OnceCell::new(),
        }
    }

    fn chosen(&self) -> Result<Date, PortFailure> {
        if !self.offline {
            return Ok(self.today);
        }
        let days = self
            .root
            .as_ref()
            .and_then(|root| self.files.entries(root).ok())
            .unwrap_or_default();
        days.iter()
            .filter_map(|name| Date::parse(name).ok())
            .filter(|day| *day <= self.today)
            .max()
            .ok_or_else(|| PortFailure::RegistryUnreachable {
                detail: format!(
                    "`--offline`, and nothing is cached on or before {}",
                    self.today
                ),
            })
    }

    fn day(&self) -> Result<Date, PortFailure> {
        self.day.get_or_init(|| self.chosen()).clone()
    }

    fn path(&self, day: Date, kind: &str, name: &str) -> Option<PathBuf> {
        self.root
            .as_ref()
            .map(|root| PathBuf::from(format!("{}/{day}/{kind}/{name}", root.display())))
    }
}

impl AnswerCache for DayCache<'_> {
    fn read(&self, kind: &str, name: &str) -> Result<Option<String>, PortFailure> {
        let day = self.day()?;
        let Some(path) = self.path(day, kind, name) else {
            return Ok(None);
        };
        match self.files.read(&path) {
            Ok(body) => Ok(Some(body)),
            Err(FileFailure::Missing) if self.offline => Err(PortFailure::RegistryUnreachable {
                detail: format!(
                    "`--offline`, and the cache of {day} holds no {kind} answer for `{name}`"
                ),
            }),
            Err(FileFailure::Missing) => Ok(None),
            Err(FileFailure::Failed { detail }) => Err(PortFailure::RegistryCacheUnusable {
                detail: format!("`{}` could not be read: {detail}", path.display()),
            }),
        }
    }

    fn read_on(&self) -> Date {
        self.day().unwrap_or(self.today)
    }

    fn write(&self, kind: &str, name: &str, body: &str) -> Result<(), PortFailure> {
        let path = self.path(self.today, kind, name).ok_or_else(|| {
            PortFailure::RegistryCacheUnusable {
                detail: String::from(NO_DIRECTORY),
            }
        })?;
        self.files.write(&path, body).map_err(|failure| {
            let detail = match failure {
                FileFailure::Failed { detail } => detail,
                FileFailure::Missing => String::from("its directory is not there"),
            };
            PortFailure::RegistryCacheUnusable {
                detail: format!("`{}` could not be written: {detail}", path.display()),
            }
        })
    }
}
