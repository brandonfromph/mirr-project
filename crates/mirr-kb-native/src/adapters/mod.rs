#![allow(unsafe_code)]
#![deny(warnings)]

/// Adapter for embedding providers (cloud or deterministic fallback).
pub mod embedding;
pub mod embedding_native;

/// Adapter for vector stores (local SQLite-backed retrieval first).
pub mod vector_store;
