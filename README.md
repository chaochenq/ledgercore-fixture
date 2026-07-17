# ledgercore — double-entry payments ledger (security validation fixture — non-agentic control)

> ## ⚠️ DELIBERATELY VULNERABLE — DO NOT USE
> This repository is a **synthetic security-scanner validation fixture** written
> in Rust. It contains intentionally planted vulnerabilities (SQL injection,
> cross-tenant IDOR, public S3 bucket, over-broad IAM, committed credentials,
> insecure CI/CD coding-agent workflows) and only fake/placeholder secrets and
> fake AWS accounts. **Do not deploy it, do not copy any code or workflow from
> it, and do not treat any pattern here as guidance.** It exists only to be
> scanned.

## What this is — the NON-AGENTIC CONTROL

**ledgercore** is a fictional double-entry payments ledger: an accounting engine
+ a REST API + a Postgres data tier. It is a **conventional, non-agentic
product** — there are **no LLM agents, tools, MCP servers, or skills** in the
product code. It is the precision anchor for the benchmark: the correct
agentic-inventory result is **zero product agents**.

The only genuine agentic surface is:
1. **CI/CD coding agents** (`.github/workflows/`) — Codex, aider, Claude Code
   (a mix of vulnerable + positive-control).
2. **Coding-agent instruction files** — `CLAUDE.md`, `AGENTS.md`, `.cursorrules`.

It validates:
1. **Threat model** — classical threats (SQLi, IDOR, IAM, encryption, secrets).
2. **Agentic inventory posture** — must be MINIMAL: zero product runtimes/tools/
   MCP servers; only the CI coding agents + instruction files.
3. **Agentic deployment-automation posture** — the CI coding agents, graded.

## False-positive traps (`src/fp/`) — must ALL be ABSENT from the agent inventory

Every type here is named to LOOK agentic but is ordinary domain/infra code:

| Type | Named like | Actually is |
|---|---|---|
| `SettlementAgent` | an agent | a batch-settlement worker (no LLM) |
| `RebalancePlanner` | an LLM planner | a greedy rebalancing algorithm |
| `MigrationTool` | an agent tool | a DB schema-migration helper |
| `BatchExecutor` | an agent loop | a thread-pool work runner |
| `BedrockClient` | an agent | an unused LLM-transport prototype |
| `ReconAction` (enum) | a tool vocabulary | reconciliation state-machine states |

A scan must mint **0** agents/tools from these.

## Layer → threat map

| Layer | Carries threats of type |
|---|---|
| `src/api/` + `src/data/` | SQL injection, cross-tenant IDOR, authorization |
| `infra/` | public buckets, encryption-at-rest, IAM least-privilege, committed creds |
| `.github/workflows/` | CI/CD coding-agent posture (the only agentic surface) |

## Note for maintainers

The ground-truth label set lives with the Trent design docs, not in this repo.

## Build / test

```bash
cargo build --locked && cargo test --locked
```
