#![forbid(unsafe_code)]

use mirrc::emit::rspu_tagged::{Provenance, TaggedWord, TypeTag};
use mirrc::symbolic::fingerprint::{
    fingerprint_tagged, fingerprint_u64, RollingFingerprint, MAX_FINGERPRINT_WINDOW,
};

#[test]
fn test_integration_fingerprint_matching() {
    // Deterministic FNV-1a matching
    let window1 = vec![100, 200, 300];
    let window2 = vec![100, 200, 300];
    assert_eq!(fingerprint_u64(&window1), fingerprint_u64(&window2));

    let window3 = vec![300, 200, 100];
    assert_ne!(fingerprint_u64(&window1), fingerprint_u64(&window3));
}

#[test]
fn test_integration_rolling_fingerprint_sliding() {
    let mut rolling = RollingFingerprint::new(4);

    rolling.push(10);
    rolling.push(20);
    rolling.push(30);
    rolling.push(40);
    assert_eq!(rolling.compute(), fingerprint_u64(&[10, 20, 30, 40]));

    rolling.push(50); // Dropped 10
    assert_eq!(rolling.compute(), fingerprint_u64(&[20, 30, 40, 50]));

    rolling.reset();
    rolling.push(99);
    assert_eq!(rolling.compute(), fingerprint_u64(&[99]));
}

#[test]
fn test_integration_fingerprint_respects_nasa_bounds() {
    let mut super_large = vec![1; MAX_FINGERPRINT_WINDOW + 20];
    let h1 = fingerprint_u64(&super_large);

    // Modify a value strictly beyond the window limit
    super_large[MAX_FINGERPRINT_WINDOW + 5] = 9999;
    let h2 = fingerprint_u64(&super_large);

    // Hash should remain identical because indices >= MAX_FINGERPRINT_WINDOW are ignored
    assert_eq!(h1, h2);
}

#[test]
fn test_integration_fingerprint_tagged_types() {
    let w1 = vec![TaggedWord {
        value: 0xFF,
        tag: TypeTag::Unsigned { width: 8 },
        provenance: Provenance::Literal,
    }];
    let w2 = vec![TaggedWord {
        value: 0xFF,
        tag: TypeTag::Signed { width: 8 },
        provenance: Provenance::Literal,
    }];

    // Same value, but different tags should produce different fingerprints
    assert_ne!(fingerprint_tagged(&w1), fingerprint_tagged(&w2));
}
