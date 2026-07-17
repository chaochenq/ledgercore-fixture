//! FALSE-POSITIVE TRAP (intentional).
//!
//! `MigrationTool` is named "tool" but is a database schema-migration helper,
//! NOT an agent tool. It is never registered with any agent (there are no agents
//! in this product). A scan must NOT classify it as an agent tool.

pub struct MigrationTool {
    version: u32,
}

impl MigrationTool {
    pub fn new(version: u32) -> Self {
        Self { version }
    }

    pub fn migrate(&self) -> String {
        format!("migrated to schema v{}", self.version)
    }
}
