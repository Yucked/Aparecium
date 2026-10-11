Declares a model: a struct stored in SurrealDB, with a typed schema for building queries.

```text
#[aparecium::model(node, table = "user")]     a record in its own table
#[aparecium::model(edge, table = "follows")]  a record relating two nodes
#[aparecium::model(object)]                   a value embedded in another model
```

The attribute derives the SDK's `SurrealValue` for the struct, so values convert to and from the
database without Serde, and generates:

- for every model, `<Model as SchemaGetter>::schema()`, whose fields are paths for queries
  (`User::schema().name`) and whose link methods traverse records and graph edges;
- for nodes and edges, the [`Model`] implementation (table, record id, field definitions for
  migrations) and `<Model>Patch`, a partial update with one [`Patch`] per stored field.

# Record ids

Nodes and edges carry their id as `id: RecordId<Self>`. The key is always chosen by the caller:
`RecordId::rand()`, `RecordId::ulid()`, `RecordId::uuid()` or `User::id(key)`. Decoding checks the
table, so a `RecordId<User>` never holds a `post:…` id. Embedded objects have no id.

# Field names

Stored names come from the same attributes the `SurrealValue` derive reads:
`#[surreal(rename_all = "camelCase")]` on the struct and `#[surreal(rename = "…")]` on a field.
Schema fields are named after the stored name when it is a Rust identifier, and after the Rust
field otherwise.

# Example

```rust
use aparecium::{LinkOne, Patch, RecordId, model};

#[model(node, table = "user")]
#[surreal(rename_all = "camelCase")]
pub struct User {
    pub id: RecordId<Self>,
    pub display_name: String,
    pub age: u8,
    pub nickname: Option<String>,

    #[orm(link_one = "Team")]
    pub team: LinkOne<Team>,
}

#[model(node, table = "team")]
pub struct Team {
    pub id: RecordId<Self>,
    pub name: String,
}

let alice = User {
    id: User::id("alice"),
    display_name: "Alice".to_owned(),
    age: 30,
    nickname: None,
    team: LinkOne::from(Team::id("acme")),
};

// Only `age` changes; `nickname` is removed (`NONE`); every other field is kept.
let patch = UserPatch {
    age: Patch::Set(31),
    nickname: Patch::None,
    ..Default::default()
};
```

# Settings

The attribute itself takes the model kind, `table = "…"` for nodes and edges, and
`value = manual` to keep a hand-written `SurrealValue` implementation instead of deriving one.

Database-only settings stay in `#[orm(…)]`:

| On the struct | Meaning |
| --- | --- |
| `relax_table` | Allows a table name other than the struct name in snake case. |
| `schemafull`, `flexible`, `drop` | Flags of the generated `DEFINE TABLE`. |
| `as = …` | A select statement making the table a view. |
| `permissions = …` | `full`, `none`, or an expression giving the permissions. |
| `define = …` | An expression giving the whole `DEFINE TABLE`; excludes the settings above. |

| On a field | Meaning |
| --- | --- |
| `ty = "…"` | The stored type, written as in SurrealQL (`option<array<record<user>>>`); inferred from the Rust type when omitted. |
| `link_one`, `link_self`, `link_many` | The node a record link points to. |
| `nest_object`, `nest_array` | The embedded object type of the field or of its elements. |
| `relate(model = …, connection = "->edge->node")` | A read-only relation reached through an edge; never stored. |
| `assert`, `value`, `permissions`, `item_assert` | Clauses of the generated `DEFINE FIELD`. |
| `define = …` | An expression giving the whole `DEFINE FIELD`; excludes the clauses above. |
| `old_name = "…"` | The field's previous name, for migrations that rename it. |
