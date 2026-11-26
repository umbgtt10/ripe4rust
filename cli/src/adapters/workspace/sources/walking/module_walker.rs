// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::root_names::RootNames;
use crate::adapters::workspace::sources::naming::span_line::SpanLine;
use crate::adapters::workspace::sources::walking::include_finder::IncludeFinder;
use crate::adapters::workspace::sources::walking::module_declarations::ModuleDeclarations;
use crate::adapters::workspace::sources::walking::module_files::ModuleFiles;
use crate::adapters::workspace::sources::walking::module_source::ModuleSource;
use crate::adapters::workspace::sources::walking::parsed_file::ParsedFile;
use crate::adapters::workspace::sources::walking::walk::Walk;
use crate::system::model::file_failure::FileFailure;
use crate::system::traits::file_system::FileSystem;
use ripe4rust_core::inputs::unparsed_file::UnparsedFile;
use ripe4rust_core::values::location::Location;
use std::path::Path;
use syn::parse_file;
use syn::visit::Visit;

const MOD_RS: &str = "mod.rs";
const NOT_THERE: &str = "it is not there";

/// A target's modules, from its root through every `mod` item it and its
/// modules declare and every file an `include!` of a literal path pulls in --
/// from the including file's directory, as the compiler finds it, its items in
/// the module the `include!` is in --, each file read once and parsed with
/// `syn`; and where it includes code it cannot follow. A file that cannot be
/// read or parsed, or that a `mod` item or an `include!` names but is not
/// there, is unparsed, where to look and why, and its own modules go unread.
pub struct ModuleWalker<'a> {
    files: &'a dyn FileSystem,
    root: &'a Path,
}

impl<'a> ModuleWalker<'a> {
    pub fn new(files: &'a dyn FileSystem, root: &'a Path) -> Self {
        Self { files, root }
    }

    pub fn children(&self, parsed: &ParsedFile, source: &ModuleSource) -> Vec<ModuleSource> {
        let owns_directory =
            source.owns_directory || parsed.path.file_name().is_some_and(|name| name == MOD_RS);
        ModuleDeclarations
            .of(&parsed.syntax.items, &[], parsed.public)
            .into_iter()
            .map(|declared| ModuleSource {
                candidates: ModuleFiles.candidates(&parsed.path, owns_directory, &declared),
                owns_directory: declared.path.is_some(),
                public: declared.public,
                declared_at: Some(Location {
                    file: parsed.name.clone(),
                    line: declared.line,
                }),
                name: Some(declared.name),
            })
            .collect()
    }

    pub fn included(&self, parsed: &ParsedFile) -> Vec<ModuleSource> {
        let mut finder = IncludeFinder::new(parsed.public, parsed.module.clone());
        finder.visit_file(&parsed.syntax);
        let beside = parsed
            .path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default();
        finder
            .found()
            .into_iter()
            .map(|included| ModuleSource {
                candidates: vec![ModuleFiles.resolved(&beside.join(&included.path))],
                owns_directory: false,
                public: included.public,
                declared_at: Some(Location {
                    file: parsed.name.clone(),
                    line: included.line,
                }),
                name: included.module,
            })
            .collect()
    }

    pub fn read(&self, source: &ModuleSource) -> Result<ParsedFile, UnparsedFile> {
        let names = RootNames::new(self.root);
        for candidate in &source.candidates {
            let name = names.name(candidate);
            let unparsed = |line: u32, detail: String| UnparsedFile {
                file: name.clone(),
                at: Location {
                    file: name.clone(),
                    line: line.max(1),
                },
                detail,
            };
            match self.files.read(candidate) {
                Ok(text) => {
                    let syntax = parse_file(&text)
                        .map_err(|error| unparsed(SpanLine.of(error.span()), error.to_string()))?;
                    return Ok(ParsedFile {
                        path: candidate.clone(),
                        name,
                        syntax,
                        public: source.public,
                        module: source.name.clone(),
                    });
                }
                Err(FileFailure::Failed { detail }) => return Err(unparsed(1, detail)),
                Err(FileFailure::Missing) => {}
            }
        }
        Err(self.missing(source))
    }

    pub fn unfollowed(&self, parsed: &ParsedFile) -> Vec<Location> {
        let mut finder = IncludeFinder::new(parsed.public, parsed.module.clone());
        finder.visit_file(&parsed.syntax);
        finder
            .unfollowed()
            .into_iter()
            .map(|line| Location {
                file: parsed.name.clone(),
                line,
            })
            .collect()
    }

    pub fn walk(&self, target: &Path) -> Walk {
        let mut pending = vec![ModuleSource {
            candidates: vec![target.to_path_buf()],
            owns_directory: true,
            public: true,
            declared_at: None,
            name: None,
        }];
        let mut walk = Walk {
            parsed: Vec::new(),
            unparsed: Vec::new(),
            unfollowed: Vec::new(),
        };
        while let Some(source) = pending.pop() {
            match self.read(&source) {
                Ok(parsed) if walk.parsed.iter().all(|read| read.path != parsed.path) => {
                    pending.extend(self.children(&parsed, &source));
                    pending.extend(self.included(&parsed));
                    walk.unfollowed.extend(self.unfollowed(&parsed));
                    walk.parsed.push(parsed);
                }
                Ok(_) => {}
                Err(unparsed) => walk.unparsed.push(unparsed),
            }
        }
        walk.parsed
            .sort_by(|left, right| left.name.cmp(&right.name));
        walk
    }

    fn missing(&self, source: &ModuleSource) -> UnparsedFile {
        let names = source
            .candidates
            .iter()
            .map(|candidate| RootNames::new(self.root).name(candidate))
            .collect::<Vec<String>>();
        let file = names.first().cloned().unwrap_or_default();
        let detail = match names.get(1) {
            Some(other) => format!("{NOT_THERE}, nor `{other}`"),
            None => String::from(NOT_THERE),
        };
        UnparsedFile {
            at: source.declared_at.clone().unwrap_or(Location {
                file: file.clone(),
                line: 1,
            }),
            file,
            detail,
        }
    }
}
