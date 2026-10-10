# Aparecium migration plan

Updated: October 10, 2026.

## What we are building

**Aparecium is the rebuilt successor to `surreal_orm`: a pure Rust ORM for the latest SurrealDB.**

The public package and import name are `aparecium`; `crates/orm` is its directory, not its published name. Victoria remains the reference for simple setup and an API that is easy to discover.

It runs on the latest Rust and SurrealDB, supports what current SurrealDB offers at the ORM level, and stays lean, fast and comprehensible. The old codebase is a starting point, not a specification: keep what works, rewrite what doesn't.

Scope is the ORM: models, queries, schema, graph, transactions, streaming and migrations. Server-side and operational features (wire protocols, clustering, health endpoints) are not part of the library.

## Principles

- **Latest everything.** Track current Rust, SurrealDB and dependency releases. No pinned or frozen versions.
- **Lean.** Only the crates, dependencies and abstractions that earn their place. Delete rather than preserve.
- **Fast.** No needless allocation, cloning or string re-processing in the query path.
- **Comprehensible.** One obvious way to do each thing. Consumers need one dependency and never touch internal crates.
- **Base SurrealDB features first.** Core data, schema and graph support before newer or niche additions. GQL is on the back burner.

`Cargo.lock` records what was last verified; dependencies still track latest.

## Milestones

| # | Milestone | Done when |
| --- | --- | --- |
| 1 | Upgrade and restructure | Workspace is on the latest toolchain and dependencies, cut down to the target crates |
| 2 | Migrate the codebase | Old code runs on modern Rust and SurrealDB 3.x types and semantics; **the build passes** |
| 3 | Implement current SurrealDB features | Priority 1 and 2 features are fully supported, with tests; GQL remains a separate backlog item |
| 4 | Verify and release | Full test suite passes, docs and examples match the API, ready to publish |

Testing is continuous from milestone 2 onward; milestone 4 is the final pass, not the first.

## Target structure

```text
Aparecium/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── README.md
├── crates/
│   ├── orm/          # public API: models, queries, schema, execution, migrations (feature)
│   ├── macros/       # derives and query macros
│   └── tests/        # integration tests and fixtures
└── docs/             # user guide and examples
```

| Existing code | New home |
| --- | --- |
| `orm`, `query-builder` | `crates/orm` |
| `derive`, `derive_helpers`, `proc-macros-helpers`, `query-builder-macros`, `migrator-macros` | `crates/macros` |
| `migrator` | `crates/orm` (`aparecium::migrations`, `migrations` feature) |
| `orm-tests`, `orm-compile-tests`, `derive_tests`, `migrator-tests`, `models` | `crates/tests` |
| `orm/docs`, `examples` | `docs` |

Examples in `docs` are compiled in CI (declared as `[[example]]` targets or a workspace member) so they never drift from the API.

The migrator persists its history with ORM models and runs statements through the query builder, so it sits above the ORM. A separate migrations crate re-exported by `aparecium` would form a cycle; migrations are therefore a module of `aparecium` behind the `migrations` feature. The CLI is part of that feature until its types are separated from the core migration types.

The public crate re-exports its macros, so the macro crate must not depend back on the ORM. Generated code references runtime types through the resolved `aparecium` path instead.

## Milestone 1: Upgrade and restructure

**Get onto the latest everything and the target shape before touching behavior.** The build is expected to break here; milestone 2 fixes it.

- Latest stable Rust toolchain (`rust-toolchain.toml`), edition 2024, resolver 3, workspace lints and dependency inheritance.
- Latest SurrealDB SDK (3.x) and latest versions of every other dependency. Drop unused dependencies; move test-only ones to dev-dependencies.
- Make SDK features intentional: engines, transports, TLS and scripting become opt-in features of the `orm` crate.
- Collapse the 8 production/support crates and 4 test crates into the structure above. Remove the duplicate `derive_tests` workspace entry.
- Commit `Cargo.lock`.
- Move inherited inline `#[cfg(test)]` tests into `crates/tests`, following `AGENTS.md`.
- Draft short consumer examples for connection reuse, model definition, CRUD/upsert, partial updates, relations and transactions so milestone 2 has a clear API direction.

**Done when:** workspace membership and dependency direction are valid, package names and paths are updated, and remaining compile failures are attributable to the planned code migration. Dependency resolution errors and crate cycles are not deferred to milestone 2.

## Milestone 2: Migrate the codebase

**Modern Rust and SurrealDB 3.x underneath the existing functionality. Ends with a passing build.**

### Modern Rust
- Native async traits instead of `async-trait` where suitable, keeping the future bounds and dynamic-dispatch behavior consumers need.
- Let chains, `LazyLock`, iterator-returning and borrowing APIs where they simplify code.
- Fix edition 2024 macro-fragment and lifetime-capture changes.

### SurrealDB 3.x
- Replace `surrealdb::sql` everywhere with `surrealdb::types` (`Value`, `RecordId`, `Geometry`, `Duration`, `Datetime`, …). Remove the `pub use surrealdb::sql` re-export.
- Replace uses of AST types without a suitable public equivalent with the smallest Aparecium representation needed for query construction. Do not recreate the entire server AST merely to preserve old types.
- Derives emit `SurrealValue` instead of relying on Serde. Handle renames, flattening, defaults, enums and partial updates explicitly; partial updates keep omitted, `NONE` and `NULL` distinct.
- Record IDs become `RecordId`-based, keeping typed table safety.
- Retain compile-time query validation through the current upstream parser, with `surrealdb-core` as the initial candidate. Verify its entry points, features and version compatibility before adopting it; isolate parser internals from Aparecium's public API. Syntax validation does not prove schema or permission correctness.
- Update emitted SurrealQL: `meta::*` → `record::*`, `string::is::*` and other renamed functions, removed operators, `<future>` → computed fields, current full-text index syntax, `DEFINE SCOPE`/`TOKEN` → `DEFINE ACCESS`.
- Migrator: parse current `INFO FOR` output; compare normalized definitions, not trimmed strings.

### Fix while migrating
- `save()` emits `UPDATE` but promises create-or-update: use `UPSERT`.
- Execution returns unchecked responses: check every statement's result before reporting success.
- Query rendering: deterministic parameter names and structural rendering instead of random names and repeated string replacement. Done here so milestone 3 builds on it.
- Snapshot filenames: rename to Windows-safe names (no colons).
- Macro expansion: replace hardcoded `surreal_orm` paths, support consumers that rename the `aparecium` dependency, and preserve useful field/attribute error spans.
- Binding composition: cover nested queries, reused fragments, explicit user parameters and literal text resembling parameter names. Keep execution parameterized; diagnostic SQL rendering is separate.
- Simplify ordinary type inference using upstream type information where possible; retain explicit ORM rules for table safety, relations and schema overrides.

Parser dependency choice needs care: the published `surrealdb-syn` reference identifies that crate as internal and not SemVer-stable, and directs consumers toward the SDK or core. Verify the current release rather than assuming a separately published parser is a stable interface. [Upstream parser reference](https://docs.rs/surrealdb-syn/latest/surrealdb_syn/).

**Done when:** the workspace builds, existing tests pass or are updated for intended semantic changes, and a basic consumer example (connect, define model, CRUD, relate) works.

## Milestone 3: Implement current SurrealDB features

**Build out SurrealDB's current feature set on the migrated code. Refactor or replace existing code wherever it gets in the way.**

Start with a feature inventory: list every SurrealDB capability at the ORM level, its current Aparecium support, and the work needed. Each gap becomes a GitHub issue. Raw query access doesn't count as support; a feature is supported when it has a typed, documented, tested API.

Record the official reference, server/SDK version, engine restrictions, implementation issue and verification evidence for each capability. The tables below organize the work, not limit the inventory. Priority 2 is part of this migration, not an implicit deferral. Track actual upstream limitations explicitly instead of marking an unavailable feature complete.

### Priority 1: base features
| Area | Work |
| --- | --- |
| Data operations | create, insert, select, update, upsert, delete, relate, return clauses |
| Query composition | filters, projections, ordering (incl. unprojected fields), grouping, pagination, subqueries, control flow, `SELECT … FOR UPDATE` |
| Schema | tables, fields, types, defaults, assertions, computed fields, events, `ALTER` statements |
| Graph | nodes, edges, typed traversals, record links, references, `LIGHTWEIGHT` relations, `INLINE` fields/edges |
| Access and permissions | `DEFINE ACCESS` (incl. `CONTEXT`, `AUDIENCE`), users, permissions, sign-in/sign-up |
| Functions and operators | current built-in namespaces, custom functions |
| Transactions | query transactions and SDK transaction handles |
| Migrations | schema inspection/diffing, generation, embedded and file-based execution, history, checksums, ordering and rollback |

Migration acceptance includes an empty diff when nothing changed, explicit handling of unknown resource kinds, failure propagation, and history that agrees with committed database changes. Define recovery after interrupted execution and how concurrent runners are handled. Connecting must never silently apply schema changes. Evaluate upstream migration tooling only against these requirements; replacing custom code must preserve required behavior.

### Priority 2: extended features
| Area | Work |
| --- | --- |
| Indexes and search | full-text (incl. `SEGMENT` tokenizer), HNSW and DiskANN vector indexes, vector functions |
| Streaming and live queries | row-by-row result streams, live subscriptions, cancellation |
| Files | `DEFINE BUCKET` and file operations |
| Other definitions | sequences, `DEFINE API` |

### Backlog
- GQL support.

Transaction and stream APIs must define ownership, cancellation and cleanup. Surface commit failures and uncertain write outcomes without blindly retrying writes; distinguish a finite result stream from a live subscription.

**Done when:** every priority 1 and 2 item has a typed API with tests, and examples cover each area.

## Milestone 4: Verify and release

- Full test suite across supported feature combinations; check that a minimal consumer build works outside the workspace.
- Verify behavior as well as generated SQL: value round trips, omitted/`NONE`/`NULL` updates, ID safety, statement failures, transaction rollback, binding composition and migration recovery. Include macro success/failure cases and renamed-dependency consumers.
- Compile all documentation examples, including feature-gated ones in the relevant CI configurations.
- Final pass for redundant APIs, dead code and unneeded dependencies.
- CI aligned with the workspace and toolchain.
- README quick start, feature docs, examples, and a `surreal_orm` → `aparecium` migration guide.
- Separate consumer code migration from existing database-data migration. Document breaking changes, historical migration compatibility and any required backup/export/import steps without assuming a Cargo upgrade converts stored data.
- Recheck latest releases and reconcile the feature inventory before declaring completion. Record evidence for every priority 1 and 2 capability and identify GQL as backlog in release documentation.
- Verify package contents, macro dependencies and publication order.
- Release notes and publish.

**Done when:** required build, lint, formatting, test, example and package checks pass; the feature inventory has evidence; and the documented public API works outside the workspace.

## Tracking

One umbrella issue in `Yucked/Aparecium`, four GitHub milestones matching the above, and focused issues under each. Milestone 3 issues come from the feature inventory.

Each issue records its outcome, affected code, dependencies, tests and completion checks. Working rules live in `AGENTS.md`.
