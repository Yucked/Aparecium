# Consumer API

Draft target API for milestone 2. The examples in `docs/examples/` show it in use. Names marked
*proposed* do not exist yet; everything else is current `aparecium` or SurrealDB SDK 3.3 API.

## Dependencies

A consumer needs `aparecium` plus a supported async runtime. The examples use Tokio:

```toml
[dependencies]
aparecium = "0.1"                    # default features: WebSocket + rustls
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Internal crates (`aparecium-macros`, `surrealdb`) are never named by consumers. Migrations are the
`migrations` feature of `aparecium`; embedded engines are features such as `kv-mem`.

## One way per operation

| Operation | API |
| --- | --- |
| Connect | `aparecium::connect(endpoint)` (*proposed* re-export of the SDK's `engine::any::connect`), then `use_ns`/`use_db`/`signin` |
| Reuse a connection | clone the handle |
| Create | `User::create(&db, &user)` (*proposed*) |
| Read | `User::get(&db, &id)` (*proposed*) |
| Create or replace | `user.save(&db)`, emitted as `UPSERT` |
| Partial update | `User::patch(&db, &id, UserPatch { .. })` (*proposed*) |
| Delete | `User::delete(&db, &id)` (*proposed*) |
| Relate | `alice.relate(&db, &edge, &target)` (*proposed*) |
| Transaction | `aparecium::begin(db)` → model methods take the transaction → `commit()`/`cancel()` (*proposed*, wraps the SDK 3 transaction handle) |

Model methods accept anything that can execute a statement: the client or an open transaction.
They return `aparecium::Result<T>` with one `aparecium::Error` (*proposed* names for today's
`SurrealOrmError`).
Overlapping methods in the inherited API (for example `update` vs `merge` vs `save` for the same
effect) are candidates for removal in milestone 2.

## Connections

`Surreal<C>` holds the connection behind an `Arc`, so cloning is cheap and shares one connection.
Each clone starts with a copy of the session state at the time of cloning (namespace, database,
authentication); later `use_ns`/`use_db`/`signin` calls on one clone do not affect the others.
Configure once, then clone into tasks. Endpoints, namespaces, databases and credentials come from
configuration, never literals.

## Record IDs

Model IDs are typed record IDs (`RecordId<User>`, *proposed*) over the SDK's
`surrealdb::types::RecordId`, so an ID for one table cannot be used where another table is
expected.

## Omitted, `NONE` and `NULL`

SurrealDB distinguishes an absent field (`NONE`) from a field present with an empty value (`NULL`).
The SDK maps Rust `Option<T>` as `None` ⇄ `NONE`, so `Option` alone cannot express `NULL` or "leave
unchanged". Writes therefore follow these rules.

**Whole-record writes** (`create`, `save`):

| Rust value | Stored |
| --- | --- |
| `Some(v)` | `v` |
| `None` | field absent (`NONE`) |

**Partial updates** use a generated `*Patch` struct whose fields are `Patch<T>` (*proposed*):

| `Patch<T>` | Emitted | Effect |
| --- | --- | --- |
| `Patch::Keep` (default, i.e. omitted) | nothing | field unchanged |
| `Patch::None` | `field = NONE` | field removed |
| `Patch::Null` | `field = NULL` | field present, empty |
| `Patch::Set(v)` | `field = $param` | field set |

Every value is bound as a parameter; no value is interpolated into the query text.

Open question for milestone 2: reading `NULL` back. `Option<T>` decodes only `NONE` as `None`, so
a field that may hold `NULL` needs a nullable Rust representation; choose one before #5.

Schema interaction (SurrealDB 3.3 `DEFINE FIELD`):

- `DEFAULT` applies only when a record is created. Removing a field later with `Patch::None` does
  not restore its default.
- `DEFAULT ALWAYS` also applies on update when the value is `NONE`, so `Patch::None` stores the
  default. It is not applied to `NULL`; a `NULL` the type does not allow is an error.
- `option<T>` accepts `NONE` and rejects `NULL`; `T | null` accepts `NULL` and rejects `NONE`;
  `option<T | null>` accepts both. A patch that violates the field type or an `ASSERT` fails the
  statement, and the error is returned rather than ignored.
