use umbriel::{E, LinkOne, Model, Node, RecordId, Relate, SchemaGetter, model};

#[model(node, table = "user")]
#[surreal(rename_all = "camelCase")]
pub struct User {
    pub id: RecordId<Self>,
    pub display_name: String,
    pub profile_name: String,
    #[orm(link_one = "Team")]
    pub team: LinkOne<Team>,
}

// Its `name` and `User`'s `profile_name` are distinct generated items.
#[model(node, table = "user_profile")]
pub struct UserProfile {
    pub id: RecordId<Self>,
    pub name: String,
}

// Shares `UserProfile`'s name in snake case.
#[allow(non_camel_case_types)]
#[model(node, table = "user_profile")]
pub struct User_Profile {
    pub id: RecordId<Self>,
    pub name: String,
}

// Fields and edges whose names differ only in case convention.
#[model(node, table = "task")]
#[allow(non_snake_case)]
pub struct Task {
    pub id: RecordId<Self>,
    pub page_views: i64,
    pub pageViews: i64,
    #[orm(relate(model = "TaskAssignedToTeam", connection = "->assigned_to->team"))]
    pub assignees: Relate<Team>,
    #[orm(relate(model = "TaskAssignedToCamelTeam", connection = "->assignedTo->team"))]
    pub camel_assignees: Relate<Team>,
}

#[model(edge, table = "assigned_to")]
pub struct AssignedTo<In: Node, Out: Node> {
    pub id: RecordId<Self>,
    #[orm(link_one = "In")]
    pub r#in: LinkOne<In>,
    #[orm(link_one = "Out")]
    pub out: LinkOne<Out>,
}

#[model(edge, table = "assignedTo")]
#[orm(relax_table)]
pub struct AssignedToCamel<In: Node, Out: Node> {
    pub id: RecordId<Self>,
    #[orm(link_one = "In")]
    pub r#in: LinkOne<In>,
    #[orm(link_one = "Out")]
    pub out: LinkOne<Out>,
}

pub type TaskAssignedToTeam = AssignedTo<Task, Team>;
pub type TaskAssignedToCamelTeam = AssignedToCamel<Task, Team>;

#[model(node, table = "team")]
pub struct Team {
    pub id: RecordId<Self>,
    pub name: String,
}

#[model(edge, table = "member_of")]
pub struct MemberOf<In: Node, Out: Node> {
    pub id: RecordId<Self>,
    #[orm(link_one = "In")]
    pub r#in: LinkOne<In>,
    #[orm(link_one = "Out")]
    pub out: LinkOne<Out>,
}

#[model(object)]
pub struct Address {
    pub city: String,
}

fn main() {
    // A model can also be declared inside a function.
    #[model(node, table = "pet")]
    struct Pet {
        id: RecordId<Self>,
    }
    let _ = Pet { id: Pet::id("rex") };
    let _ = User::table();
    // Each name reaches its own generated item.
    let task = Task::schema();
    let _ = (task.assigned_to__(E), task.assignedTo__(E));
    let _ = (task.page_views, task.pageViews);
}
