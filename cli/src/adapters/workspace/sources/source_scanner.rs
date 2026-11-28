// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::workspace::sources::member_findings::MemberFindings;
use crate::adapters::workspace::sources::member_scanner::MemberScanner;
use crate::system::traits::file_system::FileSystem;
use crate::system::traits::layout_source::LayoutSource;
use ripe4rust_core::inputs::port_failure::PortFailure;
use ripe4rust_core::inputs::source_scan::crate_use::CrateUse;
use ripe4rust_core::inputs::source_scan::scan::Scan;
use ripe4rust_core::inputs::source_scan::unfollowed_include::UnfollowedInclude;
use ripe4rust_core::inputs::source_scan::unparsed_file::UnparsedFile;
use ripe4rust_core::traits::source_scan::SourceScan;

/// The workspace's sources parsed with `syn`, every member's targets as the
/// layout lists them: for every crates.io dependency named, every `file:line`
/// naming it and the members whose public API it reaches, the files the parse
/// could not read, the members whose public API it could not read in full,
/// and where a member includes code it could not follow. Where it read not one file of all it tried, the scan failed as a
/// whole, and nothing is known.
pub struct SourceScanner<'a> {
    source: &'a dyn LayoutSource,
    files: &'a dyn FileSystem,
}

impl<'a> SourceScanner<'a> {
    pub fn new(source: &'a dyn LayoutSource, files: &'a dyn FileSystem) -> Self {
        Self { source, files }
    }

    pub fn unfollowed(&self, members: &[(String, MemberFindings)]) -> Vec<UnfollowedInclude> {
        let mut unfollowed = members
            .iter()
            .flat_map(|(member, found)| {
                found.unfollowed.iter().map(|at| UnfollowedInclude {
                    member: member.clone(),
                    at: at.clone(),
                })
            })
            .collect::<Vec<UnfollowedInclude>>();
        unfollowed.dedup();
        unfollowed
    }

    pub fn unparsed(&self, members: &[(String, MemberFindings)]) -> Vec<UnparsedFile> {
        let mut unparsed = members
            .iter()
            .flat_map(|(_, found)| found.unparsed.iter().cloned())
            .collect::<Vec<UnparsedFile>>();
        unparsed.sort_by(|left, right| (&left.at, &left.file).cmp(&(&right.at, &right.file)));
        unparsed.dedup();
        unparsed
    }

    pub fn uses(&self, members: &[(String, MemberFindings)]) -> Vec<CrateUse> {
        let mut packages = members
            .iter()
            .flat_map(|(_, found)| {
                found
                    .named
                    .iter()
                    .map(|(package, _)| package.clone())
                    .chain(found.exposed.iter().cloned())
            })
            .collect::<Vec<String>>();
        packages.sort();
        packages.dedup();
        packages
            .into_iter()
            .map(|name| {
                let mut locations = members
                    .iter()
                    .flat_map(|(_, found)| found.named.iter())
                    .filter(|(package, _)| *package == name)
                    .map(|(_, at)| at.clone())
                    .collect::<Vec<_>>();
                locations.sort();
                locations.dedup();
                CrateUse {
                    exposed_in: members
                        .iter()
                        .filter(|(_, found)| found.exposed.contains(&name))
                        .map(|(member, _)| member.clone())
                        .collect(),
                    locations,
                    name,
                }
            })
            .collect()
    }
}

impl SourceScan for SourceScanner<'_> {
    fn scan(&self) -> Result<Option<Scan>, PortFailure> {
        let layout = self.source.layout()?;
        let scanner = MemberScanner::new(self.files, &layout.root);
        let members = layout
            .sources
            .iter()
            .map(|member| (member.name.clone(), scanner.scan(member)))
            .collect::<Vec<(String, MemberFindings)>>();
        let read = members.iter().map(|(_, found)| found.parsed).sum::<usize>();
        let unread = members.iter().any(|(_, found)| !found.unparsed.is_empty());
        if read == 0 && unread {
            return Ok(None);
        }
        Ok(Some(Scan {
            uses: self.uses(&members),
            unparsed: self.unparsed(&members),
            unread_apis: members
                .iter()
                .filter(|(_, found)| found.api_unread)
                .map(|(member, _)| member.clone())
                .collect(),
            unfollowed: self.unfollowed(&members),
        }))
    }
}
