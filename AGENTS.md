# Working on Aparecium

Rules for any agent working in this repository. They override default behavior.

Aparecium is a pure Rust ORM for the latest SurrealDB, rebuilt from `surreal_orm`. `docs/migration-plan.md` is the roadmap.

## Repository

- `crates/orm/` — the public `aparecium` crate: models, queries, schema, execution; migrations and their CLI behind the `migrations` feature.
- `crates/macros/` — derives and query macros.
- `crates/tests/` — all tests.
- `docs/` — user guide and examples.

## Toolchain

- Development tooling runs in WSL (Ubuntu). `cargo`, `rustfmt` and `gh` are not on the Windows PATH. Run them through WSL:
  `wsl.exe -e bash -lc "cd <path> && <command>"`
- **Git runs natively on Windows, never in WSL.** Commit identity exists only in Windows Git, and WSL Git reports false modifications from CRLF/LF differences.
- Use only the project's configured toolchain (`rust-toolchain.toml`). Do not substitute Python, jq, ad-hoc scripts, formatters, linters, analyzers or other tooling without approval.
- Edit files with editor tools, not scripts.

## Permission before acting

- **No downloads without explicit approval.** This includes crates, toolchains, `cargo install`, `cargo search` and other dependencies.
- A plan mentioning an upgrade or dependency is not approval to install it. Ask immediately before the operation.
- **No new persistent databases or database instances without approval.** State where they will be created before asking. In-memory engines used by tests do not need approval.
- **Never start, restart or kill long-running processes** such as a SurrealDB server. Ask how the user wants to run them.
- **No unrequested automation, polling, wakeups or background processes.**
- **Do not discover or guess the next GitHub issue.** Work only from an issue number supplied by the user.

## Issue workflow

Issues live in `Yucked/Aparecium` on GitHub.

1. Fetch the specified issue with `gh issue view <N> --comments` and inspect the relevant code.
2. Draft a plan covering design decisions, files affected, tests and verification.
3. Post the plan to the issue with `gh issue comment <N> --body-file <file>`. Post research separately when applicable. Never embed the body directly in the shell command.
4. Stop for review. The user may edit the comment; re-fetch comments and treat the edited version as the specification.
5. Implement the approved work, keeping the change log (below) as you go.
6. Run self-verification.
7. **Stop and report.** Summarize the round from the change log. Do not commit until explicitly told to commit. Every additional implementation or fix round needs its own go-ahead before committing.
8. When approved, build the commits from the change log, not from the diff alone:
   - Make one logical change per commit. Group files only when they serve the same single change, such as a new function and its only call site. Do not merge unrelated changes just because they are small or sit in the same area.
   - If one file carries changes that belong to different logical changes, do not split hunks with scripts or interactive staging. Commit that file on its own, and list every change it contains in the body, using the log entries.
   - A fix to work that has not been committed yet belongs in that work's commit. Do not make a separate "fix" commit for it.
   - Message format: an imperative subject line, a body with one bullet per log entry covered, and `Refs #<N>` as the trailer. Never use `Closes` or `Fixes`, and add no co-author trailer.
9. Before every commit, use native Windows Git to inspect status and diff. Cross-check them against the log:
   - Every changed file must have a log entry.
   - Every logged file must appear in the status or be explained, for example as reverted.
10. After committing, record each commit's hash against its log entries, post the hashes to the issue, and stop. Do not close the issue or begin another one.

## Change log

Keep a running log of every change made for the current issue in `.agent/changes/<N>.md`. The file is local only (`.agent/` is listed in `.git/info/exclude`); never commit it.

- Append an entry immediately after each change, not in a batch at the end.
- Write one entry per logical change. A file changed for three reasons gets three entries.
- Each entry records:
  - the round
  - the path
  - the kind: added, modified, deleted, renamed or reverted
  - what changed, naming the function, type or query
  - why it changed
- Log reverts and abandoned approaches too. When an entry supersedes an earlier one, say which.
- The approved issue plan remains the specification. If the work deviates from it, note the deviation in the log and in the report.
- When resuming work on an issue, read the existing log first and continue it.

Example:

    ## Round 1 — implementation
    - `crates/orm/src/query/render.rs` — modified: `render()` allocates parameter names once per composed query so reused fragments keep their bindings.
    - `crates/orm/src/model/save.rs` — modified: `save()` emits `UPSERT` instead of `UPDATE`, matching its create-or-update contract.

    ## Round 2 — fix (review feedback)
    - `crates/orm/src/query/render.rs` — modified: literal text resembling `$param` is no longer treated as a binding, which round 1 missed.
    - `crates/orm/src/query/render.rs` — committed in `abc1234`.

## Code rules

- **Latest everything:** Write code for the latest Rust edition, compiler and SurrealDB release, using current language, SDK and crate conventions. Use the newest available features when they materially improve correctness, clarity, safety, performance or simplicity. Existing code and version pins do not define the preferred style. If this needs an upgrade, identify it and obtain approval before changing or downloading anything.
- **Old code isn't the spec:** `surreal_orm` behavior is a starting point. Keep what works; rewrite what doesn't. Do not reproduce outdated patterns or SurrealDB 1.x semantics merely because they already exist.
- **Documentation-first:** For SurrealDB, the Rust SDK and every external crate, verify APIs, syntax, defaults, feature flags and deprecations against the latest official documentation before writing or modifying code. Do not rely on model memory when current official sources are available.
- **Official sources:** Prefer the SurrealDB docs and release notes, docs.rs, the Rust reference and edition guide, and upstream repositories. Do not treat blogs, forums or third-party examples as authoritative unless official material is insufficient.
- **Verify dependencies individually:** Do not infer behavior from similar crates, previous versions or memory. Verify the specific crate and APIs being used, including breaking changes between versions.
- **Pure ORM:** Aparecium covers models, queries, schema, graph, transactions, streaming and migrations. Server-side and operational features are out of scope. Raw SurrealQL access does not count as supporting a feature; supported means a typed, documented, tested API.
- **Code quality:** Code must be lean, clear, fast and production-quality. Prefer straightforward control flow, cohesive well-named units and the minimum abstraction necessary.
- **No duplication:** Do not duplicate rendering, binding, parsing, validation, conversions, queries, constants or substantial logic. Shared behavior has a single implementation. Do not introduce pointless abstractions merely to remove trivial repetition.
- **Performance:** Avoid unnecessary allocations, clones, conversions, intermediate collections, string re-processing and repeated work, especially in the query-building path.
- **Crate boundaries:** `macros` must not depend on `orm`; generate paths to runtime types instead, resolving the consumer's crate name. Optional engines, transports and migrations stay behind features.
- **Comments:** Keep comments brief and use them to explain why something exists, not restate what the code does.
- **No accidental hardcoding:** Namespaces, databases, endpoints, credentials and tunable values belong in configuration, not literals. Genuine protocol or SurrealQL invariants may remain constants. If the distinction is unclear, ask.
- **Synthetic data only:** Tests, fixtures, examples and docs use placeholder data such as Alice/Bob, Acme Corp and `*@example.com`.

## Tests

- Tests for new or changed behavior are part of the issue plan. Do not add tests outside the approved plan.
- Existing tests must be updated when behavior they cover changes. Review snapshot changes against intended behavior; never accept them wholesale.
- **All tests live in `crates/tests`**, one file per source module named `<crate>_<module>.rs`. No `#[cfg(test)]`, `#[test]` or `#[tokio::test]` or test-only helpers in production `src/`. Inline tests inherited from `surreal_orm` are moved during the restructure; until then, do not add new ones.
- Private functionality required by tests is exposed through the crate's `test-support` feature. Never make an item generally `pub` solely for testing.
- Tests needing a running SurrealDB server or persistent storage are `#[ignore]`d. In-memory engine tests run normally.

## Tooling and formatting

- Use the configured formatter, linter and compiler. They define mechanical formatting and lint rules; do not invent competing style rules.
- Fix issues produced by the tooling rather than suppressing them, unless the suppression is technically justified and commented.
- Existing code is not authoritative when it conflicts with current conventions or configured tooling.

## Self-verification

Run in this order:

1. `cargo build --workspace --all-targets`
2. `cargo clippy --workspace --all-targets`
   - No new warnings. Known pre-existing warnings are listed here once a baseline exists after milestone 2: (none recorded yet)
3. `cargo fmt` on changed files (`rustfmt <changed files>`)
4. `cargo test --workspace`
   - Check failures and confirm the ignored-test count is expected.
5. Native Windows `git status --porcelain=v1`
   - Every modification must be intentional and match the change log. Revert unrelated changes.
6. Verify that changed production `src/` files contain no `cfg(test)`, `#[test]` or `#[tokio::test]`.

During milestone 1 the workspace is expected not to build. Report the remaining errors instead of steps 1–4, and confirm they are attributable to the planned code migration.

- Do not run checks in a way known to produce false CRLF/LF failures.
