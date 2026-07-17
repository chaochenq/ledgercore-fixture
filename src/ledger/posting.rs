//! Double-entry posting model. Pure domain type, non-agentic.

#[derive(Debug, Clone)]
pub struct Posting {
    pub account_id: String,
    pub amount_cents: i64, // positive = debit, negative = credit
    pub memo: String,
}

impl Posting {
    pub fn new(account_id: &str, amount_cents: i64, memo: &str) -> Self {
        Self { account_id: account_id.to_string(), amount_cents, memo: memo.to_string() }
    }
}
