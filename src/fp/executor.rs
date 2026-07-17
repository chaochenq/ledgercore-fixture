//! FALSE-POSITIVE TRAP (intentional).
//!
//! `BatchExecutor` is named "executor" and has a "loop", but it is a plain
//! thread-pool-style work runner for posting batches — NOT an agent execution
//! loop. No model call, no tool dispatch, no agency. A scan must NOT classify it
//! as an agent runtime.

pub struct BatchExecutor {
    workers: usize,
}

impl BatchExecutor {
    pub fn new(workers: usize) -> Self {
        Self { workers }
    }

    /// Run `jobs` in fixed-size waves. Deterministic scheduling, no agency.
    pub fn run<F: Fn(u32) -> u32>(&self, jobs: &[u32], f: F) -> Vec<u32> {
        let mut out = Vec::new();
        for wave in jobs.chunks(self.workers) {
            for &j in wave {
                out.push(f(j));
            }
        }
        out
    }
}
