//! A consumer that names `umbriel` only as `apa`: generated code must resolve `apa::…` paths, not
//! `umbriel::…`. The models cover renamed and raw-named fields, a nullable field, a flattened
//! object and a generic edge; the functions use what `#[model]` generates for them.

use apa::{
    All, FlatPatch, Field, LinkOne, Model, ModelPatch, Node, Nullable, Operatable, Patch, Query,
    Raw, RecordId, Render, Result, SchemaGetter, Setter, model, statements::select,
};

/// Stored as the fields of the model that flattens it.
#[model(object)]
#[surreal(rename_all = "camelCase")]
pub struct Contact {
    pub email_address: String,
    pub phone: Option<String>,
}

#[model(node, table = "user")]
#[surreal(rename_all = "camelCase")]
pub struct User {
    pub id: RecordId<Self>,
    pub display_name: String,
    #[surreal(rename = "years")]
    pub age: u8,
    /// Absent (`NONE`), `NULL`, or a name.
    pub nickname: Option<Nullable<String>>,
    pub r#type: String,
    #[surreal(flatten)]
    pub contact: Contact,
}

#[model(node, table = "team")]
pub struct Team {
    pub id: RecordId<Self>,
    pub name: String,
}

#[model(edge, table = "member_of")]
pub struct MemberOf<In: Node, Out: Node> {
    pub id: RecordId<Self>,
    #[orm(link_one = In)]
    pub r#in: LinkOne<In>,
    #[orm(link_one = Out)]
    pub out: LinkOne<Out>,
    pub since: u16,
}

pub type UserMemberOfTeam = MemberOf<User, Team>;

/// Renamed fields and a flattened object's field, at the model's own path.
pub fn adults_at_example() -> Result<Query> {
    let user = User::schema();
    select(All)
        .from(User::table())
        .where_(
            user.years
                .greater_than_or_equal(18)
                .and(user.contact.emailAddress.like("%@example.com")),
        )
        .to_query()
}

/// `NULL` for the nullable field; every field of the flattened object assigned.
pub fn null_nickname_and_set_contact() -> Vec<Setter> {
    UserPatch {
        nickname: Patch::Null,
        contact: FlatPatch::Set(Contact {
            email_address: "alice@example.com".to_owned(),
            phone: None,
        }),
        ..Default::default()
    }
    .into_setters()
}

/// The flattened object's definitions are the user table's own.
pub fn user_definitions() -> Vec<Raw> {
    User::define_fields()
}

/// A generic model's schema, which checks its stored names when used.
pub fn membership_since() -> Field {
    Field::from(UserMemberOfTeam::schema().since)
}

pub fn raw() -> &'static str {
    apa::query_raw!("SELECT * FROM user")
}
