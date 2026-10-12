use umbriel::migrations::{self, embed_migrations};

const _IGRATIONS_ONEWAY: migrations::EmbeddedMigrationsOneWay =
    embed_migrations!("tests/migrator-oneway");
const _MIGRATIONS_ONEWAY: migrations::EmbeddedMigrationsOneWay =
    embed_migrations!("tests/migrator-oneway");

// const MIGRATIONS2: migrations::EmbeddedMigrationsTwoWay = embed_migrations!("../migrator/migrations");
// const MIGRATIONS2: migrations::EmbeddedMigrationsTwoWay =
const _MIGRATIONS2_TWOWAY: migrations::EmbeddedMigrationsTwoWay =
    embed_migrations!("tests/migrations-twoway");

#[test]
fn test_embed_migrations() {
    // insta::assert_snapshot!(MIGRATIONS.migrations.to_vec());
}
