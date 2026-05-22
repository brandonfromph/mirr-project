#![forbid(unsafe_code)]
#![deny(warnings)]

use mirr_kb_native::{default_golden_qa_set, evaluate_pair, passes_quality_gate_for_set};

#[test]
fn golden_qa_set_passes_quality_gate_for_perfect_matches() {
    let results = default_golden_qa_set()
        .into_iter()
        .map(|pair| {
            let retrieved_chunks = pair.expected_chunks.clone();
            evaluate_pair(&pair, &retrieved_chunks, &pair.expected_answer)
        })
        .collect::<Vec<_>>();

    assert!(passes_quality_gate_for_set(&results));
    assert!(results.iter().all(|result| result.context_precision >= 0.7));
    assert!(results.iter().all(|result| result.faithfulness >= 0.8));
}

#[test]
fn golden_qa_set_fails_when_retrieval_is_off_target() {
    let pair = &default_golden_qa_set()[0];
    let result = evaluate_pair(pair, &["unrelated.chunk".to_string()], "irrelevant answer");

    assert!(!passes_quality_gate_for_set(std::slice::from_ref(&result)));
    assert!(result.context_precision < 0.7);
    assert!(result.faithfulness < 0.8);
}
