// ---------------------------------------------------------------------------
//! Signal fingerprinting for anomaly detection.
//!
//! Computes compact, deterministic signatures of signal behavior over
//! configurable time windows. Suitable for hardware realization in the
//! R-SPU Monitor/Analyze stages. All window sizes are strictly bounded
//! to satisfy NASA Power-of-10 rules.
// ---------------------------------------------------------------------------

#![forbid(unsafe_code)]

use crate::emit::rspu_tagged::TaggedWord;

/// Maximum sliding window size for signal fingerprinting.
pub const MAX_FINGERPRINT_WINDOW: usize = 64;

/// FNV-1a 64-bit offset basis.
const FNV_OFFSET_BASIS: u64 = 14695981039346656037;
/// FNV-1a 64-bit prime.
const FNV_PRIME: u64 = 1099511628211;

/// Computes a deterministic 64-bit FNV-1a hash signature over a window of values.
///
/// Bounded to `MAX_FINGERPRINT_WINDOW` elements to prevent unbounded loops.
pub fn fingerprint_u64(window: &[u64]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    let limit = window.len().min(MAX_FINGERPRINT_WINDOW);
    for &val in window.iter().take(limit) {
        // Hash value byte by byte (little endian)
        let bytes = val.to_le_bytes();
        for byte in bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
    }
    hash
}

/// Computes a deterministic signature over a window of tagged words (value + tag).
///
/// Bounded to `MAX_FINGERPRINT_WINDOW` elements.
pub fn fingerprint_tagged(window: &[TaggedWord]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS;
    let limit = window.len().min(MAX_FINGERPRINT_WINDOW);
    for word in window.iter().take(limit) {
        // Hash value (8 bytes)
        let val_bytes = word.value.to_le_bytes();
        for byte in val_bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }
        // Hash type tag representation (1 byte from tag_to_byte equivalent)
        let tag_byte = match word.tag {
            crate::emit::rspu_tagged::TypeTag::Bool => 0,
            crate::emit::rspu_tagged::TypeTag::Unsigned { .. } => 1,
            crate::emit::rspu_tagged::TypeTag::Signed { .. } => 2,
            crate::emit::rspu_tagged::TypeTag::Uninitialized => 3,
            crate::emit::rspu_tagged::TypeTag::Interval { .. } => 4,
        };
        hash ^= tag_byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Rolling fingerprint generator for real-time signal monitoring.
///
/// Maintains a bounded internal history buffer to produce fingerprints
/// incrementally.
#[derive(Debug, Clone)]
pub struct RollingFingerprint {
    history: Vec<u64>,
    window_size: usize,
}

impl RollingFingerprint {
    /// Create a new rolling fingerprint generator with the given window size.
    ///
    /// The window size is capped at `MAX_FINGERPRINT_WINDOW`.
    pub fn new(window_size: usize) -> Self {
        let size = window_size.clamp(1, MAX_FINGERPRINT_WINDOW);
        Self { history: Vec::with_capacity(size), window_size: size }
    }

    /// Add a new signal value to the history, sliding the window if necessary.
    pub fn push(&mut self, val: u64) {
        if self.history.len() >= self.window_size {
            self.history.remove(0);
        }
        self.history.push(val);
    }

    /// Compute the current fingerprint of the active history window.
    pub fn compute(&self) -> u64 {
        fingerprint_u64(&self.history)
    }

    /// Clear the history buffer.
    pub fn reset(&mut self) {
        self.history.clear();
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::rspu_tagged::{Provenance, TypeTag};

    #[test]
    fn test_fingerprint_u64_empty_and_deterministic() {
        let h1 = fingerprint_u64(&[]);
        let h2 = fingerprint_u64(&[]);
        assert_eq!(h1, h2);
        assert_eq!(h1, FNV_OFFSET_BASIS);
    }

    #[test]
    fn test_fingerprint_u64_distinct() {
        let w1 = vec![1, 2, 3];
        let w2 = vec![1, 2, 4];
        let w3 = vec![3, 2, 1];

        let h1 = fingerprint_u64(&w1);
        let h2 = fingerprint_u64(&w2);
        let h3 = fingerprint_u64(&w3);

        assert_ne!(h1, h2);
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_fingerprint_u64_respects_max_bound() {
        let mut large_w = vec![0; MAX_FINGERPRINT_WINDOW + 10];
        let h1 = fingerprint_u64(&large_w);

        large_w[MAX_FINGERPRINT_WINDOW] = 999; // Beyond the window limit
        let h2 = fingerprint_u64(&large_w);

        // Modification beyond MAX_FINGERPRINT_WINDOW should have no effect.
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_rolling_fingerprint() {
        let mut rolling = RollingFingerprint::new(3);
        assert_eq!(rolling.compute(), FNV_OFFSET_BASIS);

        rolling.push(1);
        rolling.push(2);
        rolling.push(3);

        let h1 = rolling.compute();
        assert_eq!(h1, fingerprint_u64(&[1, 2, 3]));

        rolling.push(4); // Slides window, drops 1
        let h2 = rolling.compute();
        assert_eq!(h2, fingerprint_u64(&[2, 3, 4]));
    }

    #[test]
    fn test_fingerprint_tagged() {
        let w = vec![
            TaggedWord {
                value: 42,
                tag: TypeTag::Unsigned { width: 8 },
                provenance: Provenance::Literal,
            },
            TaggedWord { value: 1, tag: TypeTag::Bool, provenance: Provenance::Computed },
        ];
        let h = fingerprint_tagged(&w);
        assert_ne!(h, FNV_OFFSET_BASIS);
    }
}
