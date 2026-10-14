use umbriel::{Model, RecordId, model};

#[model(node, table = "user")]
pub struct User {
    pub id: RecordId<Self>,
}

#[model(node, table = "team")]
pub struct Team {
    pub id: RecordId<Self>,
}

fn main() {
    // An id is typed by its model's table, so a user's id is not a team's.
    let _team: RecordId<Team> = User::id("alice");
}
