// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const PRIME: u64 = 0x0000_0100_0000_01b3;
const ABSENT: u8 = 0xff;
const SEPARATOR: u8 = 0;

/// The workspace's fingerprint: FNV-1a, 64 bits, over every file by name --
/// its name, then its length and its bytes, or a mark where it is absent --
/// written as sixteen hex digits. A change detector, not a seal: by hand, so
/// the cli takes no hashing crate.
pub struct Fingerprint;

impl Fingerprint {
    pub fn of(&self, files: &[(String, Option<String>)]) -> String {
        let mut sorted = files.iter().collect::<Vec<&(String, Option<String>)>>();
        sorted.sort_by(|left, right| left.0.cmp(&right.0));
        let mut bytes = Vec::new();
        for (name, contents) in sorted {
            bytes.extend(name.as_bytes());
            bytes.push(SEPARATOR);
            match contents {
                Some(contents) => {
                    bytes.extend(contents.len().to_string().as_bytes());
                    bytes.push(SEPARATOR);
                    bytes.extend(contents.as_bytes());
                }
                None => bytes.push(ABSENT),
            }
        }
        let hash = bytes.iter().fold(OFFSET_BASIS, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(PRIME)
        });
        format!("{hash:016x}")
    }
}
