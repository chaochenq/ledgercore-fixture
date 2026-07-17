//! FALSE-POSITIVE TRAP (intentional).
//!
//! `SettlementAgent` is named "agent" but is a plain background *worker* that
//! settles posting batches on a timer. There is NO LLM, no tool loop, no model
//! call, no agency — it is a domain worker in a payments ledger. A scan must NOT
//! classify it as an agent runtime. (This is the stratus/`*Agent`-named-struct
//! precision class, by construction.)

use crate::ledger::posting::Posting;

pub struct SettlementAgent {
    batch_size: usize,
}

impl SettlementAgent {
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }

    /// Settle a batch of postings (pure domain logic, no model in sight).
    pub fn settle(&self, postings: &[Posting]) -> u64 {
        postings
            .chunks(self.batch_size)
            .map(|c| c.iter().map(|p| p.amount_cents).sum::<i64>().unsigned_abs())
            .sum()
    }
}
