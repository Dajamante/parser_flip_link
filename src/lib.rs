mod lexer;
pub mod parser;
use anyhow::{bail, Error, Result};

use lexer::{Token, TokenType};
use std::{iter::Peekable, ops::RangeInclusive};

#[derive(Debug, Clone, PartialEq)]
struct MemoryEntry {
    line: usize,
    origin: u64,
    length: u64,
}

impl MemoryEntry {
    fn end(&self) -> u64 {
        self.origin + self.length
    }

    fn span(&self) -> RangeInclusive<u64> {
        self.origin..=self.end()
    }
}

// Perform addition when ORIGN or LENGTH variables contain an addition.
// If there is no addition to be performed, it will return the `u64` value.
// fn perform_addition(line: &str) -> u64 {
//     42
// }

// fn advance(tt: TokenType, tokens: &[lexer::Token]) -> usize {
//     42
// }

// fn is_origin(t: &lexer::Token) -> bool {
//     true
// }

// fn parse_expression(tokens: &[lexer::Token]) -> u64 {
//     42
// }

// fn parse_number(t: Peekable<std::slice::Iter<'_, lexer::Token>>) -> u64 {
//     42
// }

// fn parse(tokens: Vec<Token>) {}
// fn find_ram_in_linker_script(script: &str) -> Option<MemoryEntry> {
//     // call parse

//     return Some(MemoryEntry {
//         line: 0,
//         origin: 0,
//         length: 0,
//     });
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn parse() {
//         const LINKER_SCRIPT: &str = "MEMORY
// {
//   FLASH : ORIGIN = 0x00000000, LENGTH = 256K
//   RAM : ORIGIN = 0x20000000, LENGTH = 64K
// }
// ";

//         assert_eq!(
//             find_ram_in_linker_script(LINKER_SCRIPT),
//             Some(MemoryEntry {
//                 line: 3,
//                 origin: 0x20000000,
//                 length: 64 * 1024,
//             })
//         );
//     }

//     #[test]
//     fn parse_no_units() {
//         const LINKER_SCRIPT: &str = "MEMORY
// {
//   FLASH : ORIGIN = 0x00000000, LENGTH = 262144
//   RAM : ORIGIN = 0x20000000, LENGTH = 65536
// }
// ";

//         assert_eq!(
//             find_ram_in_linker_script(LINKER_SCRIPT),
//             Some(MemoryEntry {
//                 line: 3,
//                 origin: 0x20000000,
//                 length: 64 * 1024,
//             })
//         );
//     }

//     #[test]
//     fn test_perform_addition_hex_and_number() {
//         const ADDITION: &str = "0x20000000 + 1000";
//         let expected: u64 = 0x20000000 + 1000;

//         assert_eq!(perform_addition(ADDITION), expected);
//     }

//     #[test]
//     fn test_perform_addition_returns_number() {
//         const NO_ADDITION: &str = "0x20000000";
//         let expected: u64 = 536870912; //0x20000000 base 10

//         assert_eq!(perform_addition(NO_ADDITION), expected);
//     }

//     #[test]
//     fn parse_plus() {
//         const LINKER_SCRIPT: &str = "MEMORY
// {
//   FLASH : ORIGIN = 0x08000000, LENGTH = 2M
//   RAM : ORIGIN = 0x20020000, LENGTH = 368K + 16K
// }
// ";

//         assert_eq!(
//             find_ram_in_linker_script(LINKER_SCRIPT),
//             Some(MemoryEntry {
//                 line: 3,
//                 origin: 0x20020000,
//                 length: (368 + 16) * 1024,
//             })
//         );
//     }

//     #[test]
//     fn parse_plus_origin_k() {
//         const LINKER_SCRIPT: &str = "MEMORY
// {
//   FLASH : ORIGIN = 0x08000000, LENGTH = 2M
//   RAM : ORIGIN = 0x20020000 + 100K, LENGTH = 368K
// }
// ";

//         assert_eq!(
//             find_ram_in_linker_script(LINKER_SCRIPT),
//             Some(MemoryEntry {
//                 line: 3,
//                 origin: 0x20020000 + (100 * 1024),
//                 length: 368 * 1024,
//             })
//         );
//     }

//     #[test]
//     fn parse_plus_origin_no_units() {
//         const LINKER_SCRIPT: &str = "MEMORY
// {
//   FLASH : ORIGIN = 0x08000000, LENGTH = 2M
//   RAM : ORIGIN = 0x20020000 + 1000, LENGTH = 368K
// }

// ";

//         assert_eq!(
//             find_ram_in_linker_script(LINKER_SCRIPT),
//             Some(MemoryEntry {
//                 line: 3,
//                 origin: 0x20020000 + 1000,
//                 length: 368 * 1024,
//             })
//         );
//     }

//     #[test]
//     fn parse_plus_origin_m() {
//         const LINKER_SCRIPT: &str = "MEMORY
// {
//   FLASH : ORIGIN = 0x08000000, LENGTH = 2M
//   RAM : ORIGIN = 0x20020000 + 100M, LENGTH = 368K
// }

// ";

//         assert_eq!(
//             find_ram_in_linker_script(LINKER_SCRIPT),
//             Some(MemoryEntry {
//                 line: 3,
//                 origin: 0x20020000 + (100 * 1024 * 1024),
//                 length: 368 * 1024,
//             })
//         );
//     }

//     // test attributes https://sourceware.org/binutils/docs/ld/MEMORY.html
//     #[test]
//     fn parse_attributes() {
//         const LINKER_SCRIPT: &str = "MEMORY
// {
//     /* NOTE 1 K = 1 KiBi = 1024 bytes */
//     FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 1024K
//     RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 128K
// }
// ";

//         assert_eq!(
//             find_ram_in_linker_script(LINKER_SCRIPT),
//             Some(MemoryEntry {
//                 line: 4,
//                 origin: 0x20000000,
//                 length: 128 * 1024,
//             })
//         );
//     }
// }
