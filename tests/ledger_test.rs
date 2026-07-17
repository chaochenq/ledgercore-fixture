//! Wiring contract: the product is a ledger, not an agent. These tests exercise
//! the accounting core and assert the FP-trap types behave as ordinary domain
//! code (no agency), matching the by-construction "zero agents" ground truth.

use ledgercore::fp::action::{decide, ReconAction};
use ledgercore::fp::agent::SettlementAgent;
use ledgercore::ledger::engine::LedgerEngine;
use ledgercore::ledger::posting::Posting;

#[test]
fn balanced_posting_updates_accounts() {
    let mut eng = LedgerEngine::new();
    eng.post(&[Posting::new("cash", 500, "sale"), Posting::new("revenue", -500, "sale")]).unwrap();
    assert_eq!(eng.balance("cash"), 500);
    assert_eq!(eng.balance("revenue"), -500);
}

#[test]
fn unbalanced_posting_rejected() {
    let mut eng = LedgerEngine::new();
    assert!(eng.post(&[Posting::new("cash", 500, "oops")]).is_err());
}

#[test]
fn settlement_agent_is_just_a_worker() {
    // Named "agent" but it only sums batches — no agency.
    let a = SettlementAgent::new(2);
    let total = a.settle(&[Posting::new("a", 100, ""), Posting::new("b", -100, "")]);
    assert_eq!(total, 0);
}

#[test]
fn recon_action_is_a_state_enum_not_a_tool_vocab() {
    assert_eq!(decide(100, 100), ReconAction::Match);
    assert_eq!(decide(1_000_000_000, 0), ReconAction::Escalate);
}
