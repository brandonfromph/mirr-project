/// TDD: StringInterner + InternId
///
/// Written BEFORE the implementation per project TDD mandate.
/// These tests fully specify the intern contract that the production
/// code in `src/ecs/intern.rs` must satisfy.
#[cfg(test)]
mod ecs_intern_tests {
    use mirrc::ecs::intern::{InternId, StringInterner, INTERN_INVALID, MAX_INTERN_ENTRIES};

    // -----------------------------------------------------------------------
    // T1: Same string → same InternId (deduplication).
    // -----------------------------------------------------------------------
    #[test]
    fn test_intern_dedup() {
        let mut interner = StringInterner::new();
        let a = interner.intern("clk");
        let b = interner.intern("clk");
        assert_eq!(a, b, "Interning the same string twice must return the same id");
    }

    // -----------------------------------------------------------------------
    // T2: Different strings → different InternIds.
    // -----------------------------------------------------------------------
    #[test]
    fn test_intern_distinct() {
        let mut interner = StringInterner::new();
        let a = interner.intern("clk");
        let b = interner.intern("rst_n");
        assert_ne!(a, b, "Different strings must return different ids");
    }

    // -----------------------------------------------------------------------
    // T3: Round-trip: resolve(intern(s)) == s.
    // -----------------------------------------------------------------------
    #[test]
    fn test_intern_resolve_roundtrip() {
        let mut interner = StringInterner::new();
        let names = ["clk", "rst_n", "data_valid", "out_ready", "_assign_42"];
        let mut ids = [InternId(0); 5];

        let mut i = 0usize;
        while i < names.len() {
            ids[i] = interner.intern(names[i]);
            i += 1;
        }

        let mut i = 0usize;
        while i < names.len() {
            let resolved = interner.resolve(ids[i]);
            assert_eq!(resolved, names[i], "resolve must return the original string");
            i += 1;
        }
    }

    // -----------------------------------------------------------------------
    // T4: Comparison is integer equality — no string allocation at compare time.
    // -----------------------------------------------------------------------
    #[test]
    fn test_intern_compare_is_integer() {
        let mut interner = StringInterner::new();
        let id_a = interner.intern("token");
        let id_b = interner.intern("token");
        // InternId is Copy — comparison is u32 == u32.
        assert_eq!(id_a, id_b);
        // Verify it's not accidentally comparing strings.
        assert_eq!(id_a.0, id_b.0, "InternId.0 must be equal integers");
    }

    // -----------------------------------------------------------------------
    // T5: Capacity cap — intern beyond MAX_INTERN_ENTRIES returns INTERN_INVALID.
    // -----------------------------------------------------------------------
    #[test]
    fn test_intern_cap_returns_invalid() {
        let mut interner = StringInterner::new();

        // Fill to the cap using distinct short strings.
        // We cannot use i.to_string() directly as a bounded loop,
        // so we track fill count explicitly.
        let mut filled = 0usize;
        let mut n = 0u32;
        while filled < MAX_INTERN_ENTRIES {
            // Generate a unique string without format! heap churn:
            // use a fixed-size buffer approach.
            let s = n.to_string(); // allowed in test setup (not hot path)
            interner.intern(&s);
            filled += 1;
            n += 1;
        }

        // The (MAX_INTERN_ENTRIES + 1)th unique string must return INTERN_INVALID.
        let overflow_id = interner.intern("THIS_OVERFLOWS_THE_CAP");
        assert_eq!(
            overflow_id, INTERN_INVALID,
            "Intern beyond MAX_INTERN_ENTRIES must return INTERN_INVALID"
        );
    }

    // -----------------------------------------------------------------------
    // T6: Empty interner has len 0.
    // -----------------------------------------------------------------------
    #[test]
    fn test_intern_empty_state() {
        let interner = StringInterner::new();
        assert_eq!(interner.len(), 0);
        assert!(interner.is_empty());
    }

    // -----------------------------------------------------------------------
    // T7: len grows correctly and de-duplication does not inflate it.
    // -----------------------------------------------------------------------
    #[test]
    fn test_intern_len_dedup_does_not_inflate() {
        let mut interner = StringInterner::new();
        interner.intern("a");
        interner.intern("b");
        interner.intern("a"); // duplicate — must NOT increase len
        assert_eq!(interner.len(), 2, "Deduplication must not inflate len");
    }

    // -----------------------------------------------------------------------
    // T8: InternId(u32::MAX) is the canonical INTERN_INVALID sentinel.
    // -----------------------------------------------------------------------
    #[test]
    fn test_intern_invalid_sentinel_value() {
        assert_eq!(INTERN_INVALID.0, u32::MAX, "INTERN_INVALID must be u32::MAX");
    }

    // -----------------------------------------------------------------------
    // T9: InternId is Copy — assignment does not move.
    // -----------------------------------------------------------------------
    #[test]
    fn test_intern_id_is_copy() {
        let mut interner = StringInterner::new();
        let id = interner.intern("copy_me");
        let copy = id; // would fail to compile if InternId were not Copy
        assert_eq!(id, copy);
    }

    // -----------------------------------------------------------------------
    // T10: resolve on a valid id from a populated interner is correct
    //      after many interments.
    // -----------------------------------------------------------------------
    #[test]
    fn test_intern_resolve_after_many_entries() {
        let mut interner = StringInterner::new();
        let names = [
            "clk", "rst_n", "data_in", "data_out", "valid", "ready", "token", "counter", "state",
            "mode",
        ];
        let mut ids = [INTERN_INVALID; 10];
        let mut i = 0usize;
        while i < names.len() {
            ids[i] = interner.intern(names[i]);
            i += 1;
        }
        // Verify resolve is stable — interning more strings does not corrupt earlier entries.
        interner.intern("extra_1");
        interner.intern("extra_2");

        let mut i = 0usize;
        while i < names.len() {
            assert_eq!(
                interner.resolve(ids[i]),
                names[i],
                "resolve must be stable after further interments"
            );
            i += 1;
        }
    }
}
