// FALSE-POSITIVE TRAPS. Every type in this module is named to LOOK agentic
// (SettlementAgent, RebalancePlanner, MigrationTool, BatchExecutor,
// BedrockClient, ReconAction) but NONE is an LLM agent, tool, or runtime. This
// is the precision anchor: the correct agentic-inventory result for this product
// is ZERO agents, ZERO tools, ZERO MCP servers.
pub mod action;
pub mod agent;
pub mod executor;
pub mod llm_client;
pub mod planner;
pub mod tool;
