#![forbid(unsafe_code)]
#![deny(warnings)]

pub mod flashrank;

pub use flashrank::{FlashRankReranker, RerankedCandidate, RerankedResult};
