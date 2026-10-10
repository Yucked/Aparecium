//! Connect once and reuse the connection across tasks.
//!
//! Draft for milestone 2 (see `docs/api.md`); not compiled yet.
//! Configuration: `SURREAL_ENDPOINT` (e.g. `ws://localhost:8000`, or `mem://` with the `kv-mem`
//! feature), `SURREAL_NS`, `SURREAL_DB`.

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = aparecium::connect(env::var("SURREAL_ENDPOINT")?).await?;
    db.use_ns(env::var("SURREAL_NS")?)
        .use_db(env::var("SURREAL_DB")?)
        .await?;

    // Clones share the connection; each starts with this session's namespace and database.
    let tasks = ["alice", "bob"].map(|name| {
        let db = db.clone();
        tokio::spawn(async move { db.query("RETURN $name").bind(("name", name)).await })
    });
    for task in tasks {
        task.await??;
    }
    Ok(())
}
