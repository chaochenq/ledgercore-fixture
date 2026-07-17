//! Conventional REST handlers (non-agentic web tier).

use crate::data::pg::Pg;

/// GET /v1/accounts/{id}/ledger
/// VULN (CWE-639): tenant taken from a request header, not a verified session.
pub fn get_ledger(tenant_header: &str, account_id: &str) -> String {
    let pg = Pg::new("postgres://ledger:ledger@db/ledgercore");
    pg.account_ledger(tenant_header, account_id).unwrap_or_else(|e| e.0)
}

/// POST /admin/report — runs a caller-supplied report query. VULN: no authz.
pub fn admin_report(sql: &str) -> String {
    let pg = Pg::new("postgres://ledger:ledger@db/ledgercore");
    pg.raw(sql).unwrap_or_else(|e| e.0)
}
