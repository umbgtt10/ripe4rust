// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::table::sentence::Sentence;
use ripe4rust_core::planning::round::guiding::guide_text::GuideText;

/// The guide, for a person: the release, the round, each command with what
/// it takes and exits with, every code with its remedy, and the documents'
/// fields.
pub struct GuideTable;

impl GuideTable {
    pub fn text(&self, text: &GuideText) -> String {
        let mut lines = vec![
            format!("ripe4rust {}: the guide", text.version),
            String::new(),
            String::from("The round"),
        ];
        lines.extend(text.round.iter().cloned());
        lines.extend([String::new(), String::from("Commands")]);
        for command in &text.commands {
            lines.push(command.usage.clone());
            lines.push(format!("  {}", command.does));
            lines.extend(
                command
                    .options
                    .iter()
                    .map(|option| format!("  {}: {}", option.option, option.means)),
            );
            lines.extend(
                command
                    .exits
                    .iter()
                    .map(|exit| format!("  exit {}: {}", exit.code, exit.means)),
            );
        }
        lines.extend([String::new(), String::from("Diagnostics")]);
        lines.extend(text.codes.iter().map(|code| {
            let person = if code.needs_person() {
                ", needs a person"
            } else {
                ""
            };
            format!(
                "{} ({}{person}): {} Do: {}",
                code.label(),
                code.severity().label(),
                Sentence.ended(code.entry().means),
                code.entry().remedy
            )
        }));
        lines.extend([
            String::new(),
            String::from("The documents' fields, schema 1"),
        ]);
        lines.extend(
            text.fields
                .iter()
                .map(|field| format!("{} {}: {}", field.document, field.field, field.means)),
        );
        let mut joined = lines.join("\n");
        joined.push('\n');
        joined
    }
}
