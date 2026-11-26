// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use cargo_ripe4rust::adapters::clock::system_clock::SystemClock;
use cargo_ripe4rust::adapters::registry::cache_root::CacheRoot;
use cargo_ripe4rust::command::ripe4rust::Ripe4rust;
use cargo_ripe4rust::command::seams::Seams;
use cargo_ripe4rust::system::real::disk_file_system::DiskFileSystem;
use cargo_ripe4rust::system::real::system_command_runner::SystemCommandRunner;
use cargo_ripe4rust::system::real::thread_pause::ThreadPause;
use std::env::args_os;
use std::env::current_dir;
use std::env::var;
use std::io::Write;
use std::io::stdout;
use std::path::PathBuf;
use std::process::ExitCode;

// The shim: the real seams, the environment's cache directory, the run, its
// one document on stdout and its exit code. Everything else is the library's.
fn main() -> ExitCode {
    let cache = CacheRoot::new(
        var("XDG_CACHE_HOME").ok(),
        var("LOCALAPPDATA").ok(),
        var("HOME").ok(),
        cfg!(windows),
    )
    .path();
    let seams = Seams {
        runner: &SystemCommandRunner,
        files: &DiskFileSystem,
        pause: &ThreadPause,
        clock: &SystemClock,
        dir: current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        cache,
    };
    let args = args_os()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect::<Vec<String>>();
    let ran = Ripe4rust::new(seams).run(&args);
    let _ = stdout().write_all(ran.output.as_bytes());
    ExitCode::from(ran.code)
}
