//! Update some fields and leave the rest untouched, distinguishing omitted, `NONE` and `NULL`.
//!
//! Draft for milestone 2 (see `docs/api.md`); not compiled yet.

use std::env;

use aparecium::{Node, Patch, RecordId};

#[derive(Node, Debug, Clone)]
#[orm(table = "user")]
pub struct User {
    pub id: RecordId<User>,
    pub name: String,
    pub email: String,
    /// `option<string | null>` in the schema, so both `NONE` and `NULL` are allowed.
    pub nickname: Option<String>,
    pub team: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = aparecium::connect(env::var("SURREAL_ENDPOINT")?).await?;
    db.use_ns(env::var("SURREAL_NS")?)
        .use_db(env::var("SURREAL_DB")?)
        .await?;

    let bob = User::id("bob");

    // `#[derive(Node)]` generates `UserPatch`, every field defaulting to `Patch::Keep`.
    User::patch(
        &db,
        &bob,
        UserPatch {
            email: Patch::Set("bob@example.com".into()), // email = $param
            nickname: Patch::Null,                       // nickname = NULL
            team: Patch::None,                           // team = NONE (removed)
            ..Default::default()                         // name: not in the statement
        },
    )
    .await?;
    Ok(())
}
