use std::ops::Deref;

use crate::{DbResources, Raw, Runnable, statements::begin_transaction};

use crate::migrations::*;

/// The generator's database holding the codebase's schema.
#[derive(Debug, Clone)]
pub struct RightDatabase(
    /// The scratch database.
    pub MigratorDatabase,
);

impl RightDatabase {
    /// Applies the migration history table and the codebase's definitions in one transaction.
    pub async fn run_codebase_schema_queries(
        &self,
        code_resources: &impl DbResources,
        migration_type: MigrationFlag,
    ) -> MigrationResult<()> {
        let mut transaction = begin_transaction().query(Migration::define_table());
        for definition in Migration::define_fields(migration_type)
            .into_iter()
            .chain(Self::get_codebase_schema_queries(code_resources))
        {
            transaction = transaction.query(definition);
        }
        transaction.commit_transaction().run(self.db()).await?;
        Ok(())
    }

    /// The codebase's definitions in creation order, so each one's prerequisites exist when it
    /// runs: an analyzer may use a function, and an index an analyzer.
    pub fn get_codebase_schema_queries(db_resources: &impl DbResources) -> Vec<Raw> {
        [
            db_resources.functions(),
            db_resources.analyzers(),
            db_resources.params(),
            db_resources.accesses(),
            db_resources.users(),
            db_resources.tables(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Deref for RightDatabase {
    type Target = MigratorDatabase;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
