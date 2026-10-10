//! Run several writes atomically with the SDK 3 transaction handle.
//!
//! Draft for milestone 2 (see `docs/api.md`); not compiled yet.

use std::env;

use aparecium::{Node, RecordId};

#[derive(Node, Debug, Clone)]
#[orm(table = "account")]
pub struct Account {
    pub id: RecordId<Account>,
    pub owner: String,
    pub balance: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = aparecium::connect(env::var("SURREAL_ENDPOINT")?).await?;
    db.use_ns(env::var("SURREAL_NS")?)
        .use_db(env::var("SURREAL_DB")?)
        .await?;

    let alice = Account {
        id: Account::id("alice"),
        owner: "Alice".into(),
        balance: 100,
    };
    let bob = Account {
        id: Account::id("bob"),
        owner: "Bob".into(),
        balance: 0,
    };

    // `begin` takes a client (clone to keep using `db`); model methods accept the transaction.
    let tx = aparecium::begin(db.clone()).await?;
    let outcome = async {
        Account::create(&tx, &alice).await?;
        Account::create(&tx, &bob).await?;
        aparecium::Result::Ok(())
    }
    .await;

    // Nothing is persisted until `commit` succeeds; a failed statement cancels the whole batch.
    match outcome {
        Ok(()) => tx.commit().await?,
        Err(err) => {
            tx.cancel().await?;
            return Err(err.into());
        }
    };
    Ok(())
}
