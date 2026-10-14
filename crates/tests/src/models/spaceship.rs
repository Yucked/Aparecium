use chrono::{DateTime, Utc};
use umbriel::{RecordId, model};

#[model(node, table = "space_ship")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone, Default)]
pub struct SpaceShip {
    pub id: RecordId<Self>,
    pub name: String,
    pub created: DateTime<Utc>,
}
