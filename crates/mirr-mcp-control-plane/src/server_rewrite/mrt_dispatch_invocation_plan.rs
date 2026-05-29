#![forbid(unsafe_code)]

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MrtDispatchInvocationPlan {
    pub args: Vec<String>,
    pub stdin_data: Option<String>,
}

impl MrtDispatchInvocationPlan {
    pub fn new(args: Vec<String>) -> Self {
        Self { args, stdin_data: None }
    }

    pub fn with_stdin(args: Vec<String>, stdin_data: String) -> Self {
        Self { args, stdin_data: Some(stdin_data) }
    }
}
