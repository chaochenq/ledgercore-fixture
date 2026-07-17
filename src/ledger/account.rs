//! Account model. Pure domain type, non-agentic.

#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,
    pub balance_cents: i64,
}

impl Account {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string(), balance_cents: 0 }
    }

    pub fn apply(&mut self, amount_cents: i64) {
        self.balance_cents += amount_cents;
    }
}
