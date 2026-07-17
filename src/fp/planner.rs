//! FALSE-POSITIVE TRAP (intentional).
//!
//! `RebalancePlanner` "plans" a rebalance, but it is a deterministic algorithm
//! (greedy min-cost transfers), NOT an LLM planner. No model call, no agency.
//! A scan must NOT classify it as an agent.

pub struct RebalancePlanner;

impl RebalancePlanner {
    /// Compute transfers to level balances. Pure arithmetic.
    pub fn plan(balances: &[i64]) -> Vec<(usize, usize, i64)> {
        let avg = if balances.is_empty() { 0 } else { balances.iter().sum::<i64>() / balances.len() as i64 };
        let mut moves = Vec::new();
        for (i, &b) in balances.iter().enumerate() {
            if b > avg {
                moves.push((i, 0usize, b - avg));
            }
        }
        moves
    }
}
