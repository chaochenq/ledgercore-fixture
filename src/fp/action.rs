//! FALSE-POSITIVE TRAP (intentional).
//!
//! `ReconAction` is an enum that is `match`ed in the reconciliation state
//! machine, but its variants are RECONCILIATION states, NOT a tool/action
//! vocabulary an LLM chooses from. It is constructed and matched by deterministic
//! code, never emitted by a model. A scan must NOT treat these variants as agent
//! tools (the warp action-enum precision class, inverted: here the enum is NOT a
//! tool vocabulary).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconAction {
    Match,
    FlagDiscrepancy,
    Carry,
    WriteOff,
    Escalate,
}

/// The reconciliation step chooses a ReconAction from the numbers — no model.
pub fn decide(expected: i64, actual: i64) -> ReconAction {
    match (expected - actual).abs() {
        0 => ReconAction::Match,
        d if d < 100 => ReconAction::Carry,
        d if d < 10_000 => ReconAction::FlagDiscrepancy,
        d if d < 1_000_000 => ReconAction::WriteOff,
        _ => ReconAction::Escalate,
    }
}
