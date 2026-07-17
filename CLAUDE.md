# CLAUDE.md — ledgercore

Guidance for Claude Code (and other coding agents) working in this repository.

## Product

ledgercore is a double-entry payments ledger service. It is a **conventional,
non-agentic** product: an accounting engine + a REST API + a Postgres data tier.
There are no LLM agents, tools, MCP servers, or skills in the product.

## Architecture

- `src/ledger/` — the double-entry engine (`LedgerEngine`), accounts, postings.
- `src/api/` — REST handlers.
- `src/data/` — the Postgres client.
- `src/fp/` — internal helpers with domain names (settlement worker, rebalance
  planner, migration tool, batch executor, a prototype LLM client). NONE of
  these is an agent — they are ordinary domain/infra code.
- `infra/` — Terraform for AWS resources.

## Conventions

- Rust 2021. Use `cargo build --locked` / `cargo test --locked`.
- **Tenant scoping is the most important invariant.** Every data-tier read/write
  MUST be scoped to the verified tenant, never a request header.
- Never build SQL with `format!` — use parameterized queries.
- Secrets come from the environment / a secrets manager, never hardcoded.
