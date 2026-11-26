// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::walking::declared_module::DeclaredModule;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

const MOD_RS: &str = "mod.rs";

/// The files a `mod name;` item can name, in the order the compiler tries
/// them: `name.rs`, then `name/mod.rs`, in the declaring file's directory
/// where that file owns it -- a root, a `mod.rs`, a file a `#[path]` names
/// -- and in a directory of the file's own name otherwise, inline modules
/// adding theirs; or the one file a `#[path]` names, beside the declaring
/// file, or under its inline modules' directories. `..` is resolved.
pub struct ModuleFiles;

impl ModuleFiles {
    pub fn candidates(
        &self,
        file: &Path,
        owns_directory: bool,
        module: &DeclaredModule,
    ) -> Vec<PathBuf> {
        let beside = file.parent().map(Path::to_path_buf).unwrap_or_default();
        let own = if owns_directory {
            beside.clone()
        } else {
            beside.join(file.file_stem().unwrap_or_default())
        };
        let inline = module
            .inline
            .iter()
            .fold(own, |directory, name| directory.join(name));
        match &module.path {
            Some(path) if module.inline.is_empty() => vec![self.resolved(&beside.join(path))],
            Some(path) => vec![self.resolved(&inline.join(path))],
            None => vec![
                inline.join(format!("{}.rs", module.name)),
                inline.join(&module.name).join(MOD_RS),
            ],
        }
    }

    pub fn resolved(&self, path: &Path) -> PathBuf {
        path.components()
            .fold(PathBuf::new(), |resolved, part| match part {
                Component::ParentDir if resolved.file_name().is_some() => {
                    resolved.parent().map(Path::to_path_buf).unwrap_or_default()
                }
                _ => resolved.join(part),
            })
    }
}
