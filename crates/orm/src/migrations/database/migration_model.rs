use std::fmt::Display;

use crate::{
    All, Field, Operatable, Raw, Render, ReturnableSelect, Table,
    statements::{
        DefineTableStatement, RemoveTableStatement, begin_transaction, define_field, define_table,
        order, remove_table, select,
    },
};
use surrealdb::{
    Surreal,
    engine::any::Any,
    types::{Kind, RecordId, SurrealValue, ToSql},
};

use crate::migrations::*;

#[derive(SurrealValue, Clone, Debug)]
pub struct Migration {
    pub id: RecordId,
    pub name: String,
    pub timestamp: Timestamp,
    pub checksum_up: Checksum,
    pub checksum_down: Option<Checksum>,
    // pub timestamp: DateTime<Utc>,
    // status: String,
}

impl Ord for Migration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for Migration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.timestamp.cmp(&other.timestamp))
    }
}

impl PartialEq for Migration {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for Migration {}

#[derive(SurrealValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(u64);

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Timestamp(timestamp) = self;
        write!(f, "{timestamp}")
    }
}

impl From<Timestamp> for u64 {
    fn from(timestamp: Timestamp) -> Self {
        timestamp.0
    }
}

impl Timestamp {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl From<u64> for Timestamp {
    fn from(timestamp: u64) -> Self {
        Self(timestamp)
    }
}

pub struct MigrationSchema {
    pub id: Field,
    pub name: Field,
    pub timestamp: Field,
    pub checksum_up: Field,
    pub checksum_down: Field,
}

impl Migration {
    pub fn create_id(filename: &MigrationFilename) -> RecordId {
        RecordId::new(Migration::table(), filename.to_string())
    }

    pub fn create_reinitialize_table_raw_tx(
        filename: &MigrationFilename,
        checksum_up: &Checksum,
        checksum_down: Option<&Checksum>,
        migration_type: MigrationFlag,
    ) -> Raw {
        let mut tx = begin_transaction()
            .query(Self::remove_table())
            .query(Self::define_table());

        for def in Self::define_fields(migration_type) {
            tx = tx.query(def);
        }

        let tx = tx
            .query(Self::create_raw(filename, checksum_up, checksum_down))
            .commit_transaction();
        Raw::inline(&tx)
    }

    pub fn create_raw(
        filename: &MigrationFilename,
        checksum_up: &Checksum,
        checksum_down: Option<&Checksum>,
    ) -> Raw {
        let MigrationSchema {
            id: _id_field,
            name: name_field,
            timestamp: timestamp_field,
            checksum_up: checksum_up_field,
            checksum_down: checksum_down_field,
        } = Self::schema();

        // Values are written with the SDK's SurrealQL formatting, which quotes and escapes them
        // (the file name and checksums were spliced into `'…'` unescaped).
        let record_id = Self::create_id(filename).to_sql();
        let name = filename.to_string().to_sql();
        let timestamp = filename.timestamp().into_inner().to_sql();
        let checksum_up = checksum_up.to_string().to_sql();
        let checksum_down = checksum_down
            .map_or_else(|| "null".to_owned(), |c| c.to_string())
            .to_sql();
        let name_field = name_field.to_debug_sql();
        let timestamp_field = timestamp_field.to_debug_sql();
        let checksum_up_field = checksum_up_field.to_debug_sql();
        let checksum_down_field = checksum_down_field.to_debug_sql();

        Raw::new(format!(
            "CREATE {record_id} SET {name_field} = {name}, {timestamp_field} = {timestamp}, \
        {checksum_up_field} = {checksum_up}, {checksum_down_field} = {checksum_down};"
        ))
    }

    pub fn delete_raw(migration_id: &RecordId) -> Raw {
        Raw::new(format!("DELETE {};", migration_id.to_sql()))
    }

    pub fn schema() -> MigrationSchema {
        MigrationSchema {
            id: "id".into(),
            name: Field::new("name"),
            timestamp: Field::new("timestamp"),
            checksum_up: Field::new("checksum_up"),
            checksum_down: Field::new("checksum_down"),
        }
    }

    pub fn table() -> Table {
        Table::new("migration")
    }

    pub fn define_table() -> DefineTableStatement {
        define_table(Migration::table()).schemafull()
    }

    pub fn remove_table() -> RemoveTableStatement {
        remove_table(Self::table())
    }

    pub fn delete_all() -> Raw {
        Raw::new(format!("DELETE {};", Self::table().to_debug_sql()))
    }

    pub fn define_fields(migration_type: MigrationFlag) -> Vec<Raw> {
        let MigrationSchema {
            id,
            name,
            timestamp,
            checksum_up,
            checksum_down,
        } = Migration::schema();
        let define = |field: Field, kind: Kind| {
            Raw::inline(&define_field(field).type_(kind).on_table(Self::table()))
        };

        let id = define(id, Kind::Record(vec![Self::table()]));
        let name = define(name, Kind::String);
        let timestamp = define(timestamp, Kind::Int);
        let checksum_up = define(checksum_up, Kind::String);

        let mut fields = vec![id, name, timestamp, checksum_up];

        let checksum_down = define(checksum_down, Kind::String);

        if migration_type.is_twoway() {
            fields.push(checksum_down);
        }

        fields
    }

    pub async fn get_latest(db: Surreal<Any>) -> Option<Self> {
        let MigrationSchema { timestamp, .. } = Self::schema();

        select(All)
            .from(Self::table())
            .order_by(order(timestamp).desc())
            .limit(1)
            .return_one::<Self>(db.clone())
            .await
            .expect("Failed to get the latest migration")
    }

    pub async fn get_by_filename(db: Surreal<Any>, filename: &MigrationFilename) -> Option<Self> {
        let MigrationSchema { name, .. } = Self::schema();

        select(All)
            .from(Self::table())
            .where_(name.equal(filename.to_string()))
            .return_one::<Self>(db.clone())
            .await
            .expect("Failed to get migrations")
    }

    pub async fn get_all_desc(db: Surreal<Any>) -> Vec<Self> {
        let MigrationSchema { timestamp, .. } = Self::schema();

        select(All)
            .from(Self::table())
            .order_by(order(timestamp).desc())
            .return_many::<Self>(db.clone())
            .await
            .expect("Failed to get migrations")
    }
}
