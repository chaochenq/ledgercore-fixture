//! The double-entry ledger engine — the product's core. Deterministic
//! accounting, NOT an agent (despite the "engine" + posting "loop").

use std::collections::HashMap;

use crate::ledger::account::Account;
use crate::ledger::posting::Posting;

pub struct LedgerEngine {
    accounts: HashMap<String, Account>,
}

impl LedgerEngine {
    pub fn new() -> Self {
        Self { accounts: HashMap::new() }
    }

    /// Post a balanced set of entries (must sum to zero). Pure accounting.
    pub fn post(&mut self, entries: &[Posting]) -> Result<(), String> {
        let sum: i64 = entries.iter().map(|p| p.amount_cents).sum();
        if sum != 0 {
            return Err(format!("unbalanced: {sum}"));
        }
        for p in entries {
            self.accounts.entry(p.account_id.clone()).or_insert_with(|| Account::new(&p.account_id)).apply(p.amount_cents);
        }
        Ok(())
    }

    pub fn balance(&self, account_id: &str) -> i64 {
        self.accounts.get(account_id).map(|a| a.balance_cents).unwrap_or(0)
    }
}

impl Default for LedgerEngine {
    fn default() -> Self {
        Self::new()
    }
}
