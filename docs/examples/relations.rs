//! Relate two nodes through an edge and traverse it.
//!
//! Draft for milestone 2 (see `docs/api.md`); not compiled yet.

use std::env;

use aparecium::{Edge, Node, RecordId};

#[derive(Node, Debug, Clone)]
#[orm(table = "user")]
pub struct User {
    pub id: RecordId<User>,
    pub name: String,
    /// Companies reached through `->works_at->company`.
    #[orm(relate(model = "WorksAt", connection = "->works_at->company"))]
    pub employers: Vec<Company>,
}

#[derive(Node, Debug, Clone)]
#[orm(table = "company")]
pub struct Company {
    pub id: RecordId<Company>,
    pub name: String,
}

#[derive(Edge, Debug, Clone)]
#[orm(table = "works_at")]
pub struct WorksAt {
    pub role: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = aparecium::connect(env::var("SURREAL_ENDPOINT")?).await?;
    db.use_ns(env::var("SURREAL_NS")?)
        .use_db(env::var("SURREAL_DB")?)
        .await?;

    let alice = User {
        id: User::id("alice"),
        name: "Alice".into(),
        employers: Vec::new(),
    };
    let acme = Company {
        id: Company::id("acme"),
        name: "Acme Corp".into(),
    };
    User::create(&db, &alice).await?;
    Company::create(&db, &acme).await?;

    // RELATE user:alice->works_at->company:acme CONTENT $edge
    alice
        .relate(
            &db,
            &WorksAt {
                role: "Engineer".into(),
            },
            &acme,
        )
        .await?;

    // Typed traversal: SELECT ->works_at->company AS employers FROM user:alice
    let employers: Vec<Company> = User::employers(&db, &alice.id).await?;
    assert_eq!(employers[0].name, "Acme Corp");
    Ok(())
}
