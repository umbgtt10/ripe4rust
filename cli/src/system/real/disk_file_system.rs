// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::system::model::file_failure::FileFailure;
use crate::system::traits::file_system::FileSystem;
use std::ffi::OsString;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::read_to_string;
use std::fs::remove_file;
use std::fs::rename;
use std::fs::write;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::process::id;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

const PARTIAL: &str = "partial";

static WRITES: AtomicU64 = AtomicU64::new(0);

/// The disk: a file written whole -- to a file of the writer's own beside it,
/// named for the process and the write, then renamed over it -- so a cache
/// entry is never read half-written, nor taken over by another round writing
/// the same answer; a temporary file whose rename failed is removed.
pub struct DiskFileSystem;

impl DiskFileSystem {
    fn failure(error: &Error) -> FileFailure {
        match error.kind() {
            ErrorKind::NotFound => FileFailure::Missing,
            _ => FileFailure::Failed {
                detail: error.to_string(),
            },
        }
    }

    fn partial(path: &Path) -> PathBuf {
        let mut name = OsString::from(".");
        name.push(path.file_name().unwrap_or_default());
        name.push(format!(
            ".{}-{}.{PARTIAL}",
            id(),
            WRITES.fetch_add(1, Ordering::Relaxed)
        ));
        path.with_file_name(name)
    }
}

impl FileSystem for DiskFileSystem {
    fn entries(&self, directory: &Path) -> Result<Vec<String>, FileFailure> {
        let mut names = read_dir(directory)
            .map_err(|error| Self::failure(&error))?
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect::<Vec<String>>();
        names.sort();
        Ok(names)
    }

    fn read(&self, path: &Path) -> Result<String, FileFailure> {
        read_to_string(path).map_err(|error| Self::failure(&error))
    }

    fn write(&self, path: &Path, contents: &str) -> Result<(), FileFailure> {
        if let Some(parent) = path.parent() {
            create_dir_all(parent).map_err(|error| Self::failure(&error))?;
        }
        let partial = Self::partial(path);
        write(&partial, contents).map_err(|error| Self::failure(&error))?;
        rename(&partial, path).map_err(|error| {
            let _ = remove_file(&partial);
            Self::failure(&error)
        })
    }
}
