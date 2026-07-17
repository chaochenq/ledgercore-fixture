//! ledgercore — double-entry payments ledger.
//!
//! SECURITY VALIDATION FIXTURE (non-agentic control) — DELIBERATELY VULNERABLE.
//! Do not deploy.

use ledgercore::ledger::engine::LedgerEngine;
use ledgercore::ledger::posting::Posting;

fn main() {
    let mut eng = LedgerEngine::new();
    let entries = vec![Posting::new("cash", 500, "sale"), Posting::new("revenue", -500, "sale")];
    match eng.post(&entries) {
        Ok(()) => println!("cash balance = {}", eng.balance("cash")),
        Err(e) => println!("error: {e}"),
    }
}
