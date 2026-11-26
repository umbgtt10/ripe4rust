// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::adapters::cargo::crates_io::CratesIo;
use crate::system::model::member_sources::MemberSources;

const HYPHEN: char = '-';
const UNDERSCORE: &str = "_";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CodeName {
    pub code: String,
    pub package: Option<String>,
}

/// The names a member's code calls its dependencies by: the name a `package =
/// ...` renames it to, or the package's, hyphens written as underscores. A
/// crates.io dependency's name stands for its package; a path, git or other
/// registry's, and the member's own library's, are known but out of scope.
pub struct CodeNames {
    names: Vec<CodeName>,
}

impl CodeNames {
    pub fn new(member: &MemberSources) -> Self {
        let mut names = member
            .dependencies
            .iter()
            .map(|dependency| CodeName {
                code: dependency
                    .rename
                    .as_deref()
                    .unwrap_or(&dependency.name)
                    .replace(HYPHEN, UNDERSCORE),
                package: CratesIo
                    .holds(dependency.source.as_deref())
                    .then(|| dependency.name.clone()),
            })
            .collect::<Vec<CodeName>>();
        names.push(CodeName {
            code: member.name.replace(HYPHEN, UNDERSCORE),
            package: None,
        });
        names.sort();
        names.dedup();
        Self { names }
    }

    pub fn known(&self, code: &str) -> bool {
        self.names.iter().any(|name| name.code == code)
    }

    pub fn package(&self, code: &str) -> Option<&str> {
        self.names
            .iter()
            .filter(|name| name.code == code)
            .find_map(|name| name.package.as_deref())
    }
}
