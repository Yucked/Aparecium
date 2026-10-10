//! Create, read, create-or-replace and delete.
//!
//! Draft for milestone 2 (see `docs/api.md`); not compiled yet.

use std::env;

use aparecium::{Node, RecordId};

#[derive(Node, Debug, Clone)]
#[orm(table = "user")]
pub struct User {
    pub id: RecordId<User>,
    pub name: String,
    pub email: String,
    pub nickname: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = aparecium::connect(env::var("SURREAL_ENDPOINT")?).await?;
    db.use_ns(env::var("SURREAL_NS")?)
        .use_db(env::var("SURREAL_DB")?)
        .await?;

    let mut alice = User {
        id: User::id("alice"),
        name: "Alice".into(),
        email: "alice@example.com".into(),
        nickname: None,
    };

    // Fails if `user:alice` already exists.
    User::create(&db, &alice).await?;

    let found: Option<User> = User::get(&db, &alice.id).await?;
    assert_eq!(found.map(|user| user.email), Some(alice.email.clone()));

    // Create or replace the whole record (`UPSERT`); `None` stores the field as absent.
    alice.nickname = Some("Al".into());
    alice.save(&db).await?;

    User::delete(&db, &alice.id).await?;
    Ok(())
}
