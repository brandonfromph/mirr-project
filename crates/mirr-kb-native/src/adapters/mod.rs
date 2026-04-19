#![forbid(unsafe_code)]
#![deny(warnings)]

/// Adapter for embedding providers (cloud or deterministic fallback).
pub mod embedding;

/// Adapter for vector stores (local SQLite-backed retrieval first).
pub mod vector_store;
