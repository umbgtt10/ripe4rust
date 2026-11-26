// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::exposure::public_surface::PublicSurface;
use crate::adapters::workspace::sources::exposure::scoped_file::ScopedFile;
use crate::adapters::workspace::sources::member_findings::MemberFindings;
use crate::adapters::workspace::sources::naming::code_names::CodeNames;
use crate::adapters::workspace::sources::naming::declaration_collector::DeclarationCollector;
use crate::adapters::workspace::sources::naming::declarations::Declarations;
use crate::adapters::workspace::sources::naming::import_scope::ImportScope;
use crate::adapters::workspace::sources::naming::name_finder::NameFinder;
use crate::adapters::workspace::sources::naming::target_macros::TargetMacros;
use crate::adapters::workspace::sources::walking::module_walker::ModuleWalker;
use crate::system::model::member_sources::MemberSources;
use crate::system::model::source_target::SourceTarget;
use crate::system::traits::file_system::FileSystem;
use ripe4rust_core::values::location::Location;
use std::path::Path;
use syn::visit::Visit;

const LIBRARIES: [&str; 3] = ["dylib", "lib", "rlib"];

/// One member's sources parsed: each of its targets walked from its root --
/// the library, binaries, tests, benches, examples, build script --, every
/// line of each file naming a dependency, and its library's public API read
/// for the dependencies in it. A proc-macro or C library exports no Rust API.
pub struct MemberScanner<'a> {
    files: &'a dyn FileSystem,
    root: &'a Path,
}

impl<'a> MemberScanner<'a> {
    pub fn new(files: &'a dyn FileSystem, root: &'a Path) -> Self {
        Self { files, root }
    }

    pub fn named(&self, scoped: &ScopedFile<'_>) -> Vec<(String, Location)> {
        let mut finder = NameFinder::new(scoped.scope);
        finder.visit_file(&scoped.file.syntax);
        finder
            .found()
            .into_iter()
            .map(|named| {
                (
                    named.package,
                    Location {
                        file: scoped.file.name.clone(),
                        line: named.line,
                    },
                )
            })
            .collect()
    }

    pub fn scan(&self, member: &MemberSources) -> MemberFindings {
        let codes = CodeNames::new(member);
        let targets = member
            .targets
            .iter()
            .map(|target| self.target(target, &codes))
            .collect::<Vec<MemberFindings>>();
        MemberFindings {
            named: targets
                .iter()
                .flat_map(|found| found.named.clone())
                .collect(),
            exposed: targets
                .iter()
                .flat_map(|found| found.exposed.clone())
                .collect(),
            unparsed: targets
                .iter()
                .flat_map(|found| found.unparsed.clone())
                .collect(),
            api_unread: targets.iter().any(|found| found.api_unread),
            parsed: targets.iter().map(|found| found.parsed).sum(),
        }
    }

    pub fn target(&self, target: &SourceTarget, codes: &CodeNames) -> MemberFindings {
        let walk = ModuleWalker::new(self.files, self.root).walk(&target.root);
        let declarations = walk
            .parsed
            .iter()
            .map(|file| DeclarationCollector::of(&file.syntax))
            .collect::<Vec<Declarations>>();
        let macros = TargetMacros::new(&declarations, codes);
        let scopes = declarations
            .iter()
            .map(|declared| ImportScope::new(declared, codes, &macros))
            .collect::<Vec<ImportScope<'_>>>();
        let library = walk
            .parsed
            .iter()
            .zip(&scopes)
            .map(|(file, scope)| ScopedFile { file, scope })
            .collect::<Vec<ScopedFile<'_>>>();
        let api = target
            .kinds
            .iter()
            .any(|kind| LIBRARIES.contains(&kind.as_str()));
        MemberFindings {
            named: library
                .iter()
                .flat_map(|scoped| self.named(scoped))
                .collect(),
            exposed: if api {
                PublicSurface.packages(&library)
            } else {
                Vec::new()
            },
            api_unread: api && !walk.unparsed.is_empty(),
            parsed: walk.parsed.len(),
            unparsed: walk.unparsed,
        }
    }
}
