use chrono::{DateTime, Utc};
use umbriel::{LinkMany, LinkOne, Node, RecordId, Relate, model};

#[model(node, table = "user")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone, Default)]
pub struct User {
    pub id: RecordId<Self>,
    pub name: String,
    pub created: DateTime<Utc>,
    pub company: String,
    pub tags: Vec<String>,
}

#[model(edge, table = "like")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone, Default)]
pub struct Like<In: Node, Out: Node> {
    pub id: RecordId<Self>,
    #[orm(link_one = "In")]
    pub r#in: LinkOne<In>,

    #[orm(link_one = "Out")]
    pub out: LinkOne<Out>,

    #[orm(nest_object = "Time")]
    pub time: Time,
}

pub type CompanyLikeUser = Like<Company, User>;

#[model(node, table = "company")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone, Default)]
pub struct Company {
    pub id: RecordId<Self>,
    pub name: String,

    #[orm(link_many = "User")]
    pub users: LinkMany<User>,

    #[orm(relate(model = "CompanyLikeUser", connection = "->like->user"))]
    pub devs: Relate<User>,
}

#[model(object)]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone, Default)]
pub struct Time {
    pub connected: DateTime<Utc>,
}

#[model(node, table = "organization")]
#[surreal(rename_all = "camelCase")]
#[derive(Debug, Clone, Default)]
pub struct Organization {
    pub id: RecordId<Self>,
    pub name: String,

    #[orm(link_many = "User")]
    pub users: LinkMany<User>,

    #[orm(nest_object = "Time")]
    pub time: Time,
    pub age: u8,
}
