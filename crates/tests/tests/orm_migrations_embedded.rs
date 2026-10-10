/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 */
use aparecium::migrations::{
    self, DatabaseConnection, EmbeddedMigrationsTwoWay, FileMetadataStatic, Migration,
    MigrationError, Mode, UpdateStrategy, embed_migrations,
};

// Embed migrations as constant
const MIGRATIONS_ONE_WAY: migrations::EmbeddedMigrationsOneWay =
    embed_migrations!("tests/migrations-oneway");

const MIGRATIONS_TWO_WAY: migrations::EmbeddedMigrationsTwoWay =
    embed_migrations!("tests/migrations-twoway");

#[test]
fn test_embedded() {
    assert_eq!(MIGRATIONS_ONE_WAY.files().len(), 2);
    // One up/down pair.
    assert_eq!(MIGRATIONS_TWO_WAY.files().len(), 2);

    let migs = MIGRATIONS_ONE_WAY.to_migrations_one_way().unwrap();
    assert_eq!(migs.len(), 2);
    // check the meta data
    assert_eq!(
        migs[0].name().to_string(),
        "20231029202315_create_new_stuff.surql"
    );

    assert_eq!(
        migs[0].name().basename(),
        "create_new_stuff".to_string().into()
    );
    insta::assert_snapshot!(migs[0].content());
    assert_eq!(
        migs[1].name().to_string(),
        "20231029224601_create_another.surql"
    );
    assert_eq!(migs[1].name().basename(), "create_another".into());
    assert_eq!(migs.len(), 2);
    insta::assert_snapshot!(migs[1].content());

    let migs = MIGRATIONS_TWO_WAY.to_migrations_two_way().unwrap();
    assert_eq!(migs.len(), 1);

    // check the meta data
    assert_eq!(
        migs[0].up.name.to_string(),
        "20231030025711_migration_name_example.up.surql"
    );
    assert_eq!(migs[0].up.name.basename(), "migration_name_example".into());
    insta::assert_snapshot!(migs[0].up.content);
    insta::assert_snapshot!(migs[0].down.content);
}

#[tokio::test]
async fn test_embedded_run_one_way() {
    let db = DatabaseConnection::default().setup().await.db().unwrap();

    MIGRATIONS_ONE_WAY
        .run(db.clone(), UpdateStrategy::Latest, Mode::Strict)
        .await
        .unwrap();

    let db_migrations_meta = Migration::get_all_desc(db.clone()).await;
    insta::assert_debug_snapshot!(db_migrations_meta);
    assert_eq!(db_migrations_meta.len(), 2);
}

#[tokio::test]
async fn test_embedded_run_two_way() {
    let db = DatabaseConnection::default().setup().await.db().unwrap();

    MIGRATIONS_TWO_WAY
        .run(db.clone(), UpdateStrategy::Latest, Mode::Strict)
        .await
        .unwrap();

    let db_migrations_meta = Migration::get_all_desc(db.clone()).await;
    insta::assert_debug_snapshot!(db_migrations_meta);
    assert_eq!(db_migrations_meta.len(), 1);
}

const PAIR_UP: FileMetadataStatic = FileMetadataStatic {
    name: "20231030025711_create_user.up.surql",
    content: "DEFINE TABLE user;",
};
const PAIR_DOWN: FileMetadataStatic = FileMetadataStatic {
    name: "20231030025711_create_user.down.surql",
    content: "REMOVE TABLE user;",
};
const ORPHAN_DOWN: FileMetadataStatic = FileMetadataStatic {
    name: "20231031000000_create_company.down.surql",
    content: "REMOVE TABLE company;",
};

fn assert_missing_up(migrations: EmbeddedMigrationsTwoWay) {
    match migrations.to_migrations_two_way() {
        Err(MigrationError::MigrationFileDoesNotExist(name)) => {
            assert_eq!(name, "20231031000000_create_company.up.surql");
        }
        other => panic!("expected MigrationFileDoesNotExist, got {other:?}"),
    }
}

#[test]
fn test_two_way_orphan_down_beside_pair() {
    assert_missing_up(EmbeddedMigrationsTwoWay::new(&[
        PAIR_DOWN,
        PAIR_UP,
        ORPHAN_DOWN,
    ]));
}

#[test]
fn test_two_way_only_orphan_down() {
    assert_missing_up(EmbeddedMigrationsTwoWay::new(&[ORPHAN_DOWN]));
}
