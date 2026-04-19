#![forbid(unsafe_code)]

pub mod budget;

pub use budget::{
    estimate_token_count, estimate_token_count as estimate_budget_tokens, validate_query_size,
    ContextBudget,
};
