# AGENTS.md — ledgercore

Instruction file for coding agents (Codex, Cursor, aider, etc.) operating in this
repo. Mirrors `CLAUDE.md`.

- Build/test: `cargo build --locked` / `cargo test --locked`.
- ledgercore is a non-agentic accounting product; do not add LLM agents/tools.
- Preserve tenant scoping (verified tenant, never a request header).
- Parameterized SQL only; no hardcoded secrets.
