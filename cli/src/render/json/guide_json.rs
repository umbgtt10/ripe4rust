// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::render::json::documents::guide::code_entry_document::CodeEntryDocument;
use crate::render::json::documents::guide::command_entry_document::CommandEntryDocument;
use crate::render::json::documents::guide::exit_entry_document::ExitEntryDocument;
use crate::render::json::documents::guide::field_entry_document::FieldEntryDocument;
use crate::render::json::documents::guide::guide_document::GuideDocument;
use crate::render::json::documents::guide::option_entry_document::OptionEntryDocument;
use ripe4rust_core::planning::round::guiding::guide_text::GuideText;

/// The guide as the JSON document carries it: every code with what the
/// catalogue says of it.
pub struct GuideJson;

impl GuideJson {
    pub fn document<'a>(&self, text: &'a GuideText) -> GuideDocument<'a> {
        GuideDocument {
            version: &text.version,
            round: &text.round,
            commands: text
                .commands
                .iter()
                .map(|command| CommandEntryDocument {
                    usage: &command.usage,
                    does: &command.does,
                    options: command
                        .options
                        .iter()
                        .map(|option| OptionEntryDocument {
                            option: &option.option,
                            means: &option.means,
                        })
                        .collect(),
                    exits: command
                        .exits
                        .iter()
                        .map(|exit| ExitEntryDocument {
                            code: exit.code,
                            means: &exit.means,
                        })
                        .collect(),
                })
                .collect(),
            codes: text
                .codes
                .iter()
                .map(|code| CodeEntryDocument {
                    code: code.label(),
                    severity: code.severity().label(),
                    needs_person: code.needs_person(),
                    means: code.entry().means,
                    remedy: code.entry().remedy,
                })
                .collect(),
            fields: text
                .fields
                .iter()
                .map(|field| FieldEntryDocument {
                    document: &field.document,
                    field: &field.field,
                    means: &field.means,
                })
                .collect(),
            diagnostics: Vec::new(),
        }
    }
}
