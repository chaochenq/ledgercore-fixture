//! Postgres data client. Conventional data tier — carries classical threats
//! (SQL injection, cross-tenant IDOR). No agentic surface.

pub struct Pg {
    dsn: String,
}

#[derive(Debug)]
pub struct DbError(pub String);

impl Pg {
    pub fn new(dsn: &str) -> Self {
        Self { dsn: dsn.to_string() }
    }

    pub fn account_ledger(&self, tenant: &str, account_id: &str) -> Result<String, DbError> {
        // VULN (CWE-89 + CWE-639): interpolated SQL, tenant scope from the caller.
        let _sql = format!("SELECT * FROM postings WHERE account_id='{account_id}' AND tenant='{tenant}'");
        let _ = &self.dsn;
        Ok(format!("ledger for {account_id}"))
    }

    pub fn raw(&self, sql: &str) -> Result<String, DbError> {
        // VULN (CWE-89): verbatim SQL (used by an internal reporting endpoint).
        Ok(format!("rows for: {sql}"))
    }
}
