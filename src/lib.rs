//! ledgercore — a double-entry payments ledger service.
//!
//! SECURITY VALIDATION FIXTURE (non-agentic control) — DELIBERATELY VULNERABLE.
//! Do not deploy.
//!
//! This product has NO agent runtimes, tools, MCP servers, or skills. The only
//! genuine agentic surface is the CI/CD coding agents in `.github/workflows/`
//! plus the coding-agent instruction files (CLAUDE.md/AGENTS.md/.cursorrules).
//! The `fp` module holds agentically-NAMED types that are precision traps.

pub mod api;
pub mod data;
pub mod fp;
pub mod ledger;
