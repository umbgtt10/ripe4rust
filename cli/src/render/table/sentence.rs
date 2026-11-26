// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

const ENDINGS: [char; 3] = ['.', '!', '?'];
const FULL_STOP: char = '.';

/// A sentence closed once: a full stop after a text that has no ending of
/// its own, and none after one that has, so what a tool said ending in a
/// full stop never reads `..`.
pub struct Sentence;

impl Sentence {
    pub fn ended(&self, text: &str) -> String {
        let mut ended = String::from(text);
        if !text.ends_with(ENDINGS) {
            ended.push(FULL_STOP);
        }
        ended
    }
}
