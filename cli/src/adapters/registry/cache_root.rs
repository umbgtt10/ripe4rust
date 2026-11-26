// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

const DIRECTORY: &str = "ripe4rust";

/// Where the day cache lives: `%LOCALAPPDATA%` on Windows; elsewhere
/// `$XDG_CACHE_HOME`, or `~/.cache` where it is not set; `None` where none
/// of them is. An empty variable is not set.
pub struct CacheRoot {
    xdg_cache_home: Option<String>,
    local_app_data: Option<String>,
    home: Option<String>,
    windows: bool,
}

impl CacheRoot {
    pub fn new(
        xdg_cache_home: Option<String>,
        local_app_data: Option<String>,
        home: Option<String>,
        windows: bool,
    ) -> Self {
        Self {
            xdg_cache_home,
            local_app_data,
            home,
            windows,
        }
    }

    pub fn path(&self) -> Option<PathBuf> {
        let set = |value: &Option<String>| value.clone().filter(|value| !value.is_empty());
        let base = if self.windows {
            set(&self.local_app_data)
        } else {
            set(&self.xdg_cache_home)
                .or_else(|| set(&self.home).map(|home| format!("{home}/.cache")))
        };
        base.map(|base| PathBuf::from(format!("{base}/{DIRECTORY}")))
    }
}
