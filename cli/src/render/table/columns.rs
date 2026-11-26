// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

const MINIMUMS: [usize; 5] = [22, 12, 10, 10, 17];
const GAP: usize = 2;

/// The table's columns: each but the last as wide as its longest cell and a
/// gap, and never narrower than the plan's own layout.
pub struct Columns;

impl Columns {
    pub fn lines(&self, rows: &[[String; 6]]) -> Vec<String> {
        let widths = MINIMUMS
            .iter()
            .enumerate()
            .map(|(column, minimum)| {
                rows.iter()
                    .map(|row| row[column].chars().count() + GAP)
                    .max()
                    .unwrap_or(0)
                    .max(*minimum)
            })
            .collect::<Vec<usize>>();
        rows.iter()
            .map(|row| {
                let mut line = String::new();
                for (cell, width) in row.iter().zip(&widths) {
                    line.push_str(cell);
                    line.push_str(&" ".repeat(width.saturating_sub(cell.chars().count())));
                }
                line.push_str(&row[5]);
                line
            })
            .collect()
    }
}
