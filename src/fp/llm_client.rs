//! FALSE-POSITIVE TRAP (intentional).
//!
//! `BedrockClient` is an LLM transport wrapper that exists in the codebase (a
//! team once prototyped an "explain this transaction" feature) but is NOT wired
//! into any runtime — there are no agents here. It has no tool loop and no
//! agency. A scan must NOT classify it as an agent runtime; at most it is an
//! unused LLM-client dependency.

pub struct BedrockClient {
    model_id: String,
}

impl BedrockClient {
    pub fn new(model_id: &str) -> Self {
        Self { model_id: model_id.to_string() }
    }

    pub fn model(&self) -> &str {
        &self.model_id
    }

    /// Single completion call. No loop, no tools.
    pub fn complete(&self, prompt: &str) -> String {
        format!("[{}] would answer: {}", self.model_id, prompt)
    }
}
