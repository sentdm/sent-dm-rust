# Working in sent-dm-rust

The Sent SDK for Rust, `sent` on crates.io: the REST API, SMS, WhatsApp, RCS and voice. Rendered by
`sent-dm-sdk-pipeline` (`rest/`) from the published OpenAPI document; a spec sync there regenerates the
generated parts.

## What is generated and what is written

- **Generated, do not edit**: `src/models.rs`, everything under `src/resources/`, `tests/contract.rs`,
  `tests/examples.rs`, and `spec/openapi.json`. A change there is a change to the API or to the lane's
  emitter in `sent-dm-sdk-pipeline`.
- **Rendered from the lane's template**: `src/lib.rs`, `src/client.rs`, `src/error.rs`, `src/pagination.rs`,
  the workflows and these files. A fix here is welcome; the lane carries it back into the template.
- **release-please-owned**: `CHANGELOG.md`, `.release-please-manifest.json`, the `version` in `Cargo.toml`
  and `Cargo.lock`.

## Rules

- Two checks gate every PR and every release: `test` (fmt, clippy, build, the spec's examples decoding into
  the models, the crate packaging) and `contract` (every operation against a mock of `spec/openapi.json`).
- The runtime follows the API: `x-api-key` on every request, `x-profile-id` when a profile is set, an
  `Idempotency-Key` on every non-GET request that is kept across retries, retries on 429 and 5xx with
  backoff, the envelope's `data` returned and its `error` raised.
- Never log an API key.
- `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` pass on every PR, generated files
  included: the emitter formats what it writes.
- Branches named `release-please--branches--main` belong to release-please. Don't push to them or merge into
  them. Merging the release PR tags the release and publishes to crates.io by the trusted publisher; no token
  is stored.

## Commits and pull requests

- **Conventional Commits drive release-please**: `feat:`, `fix:`, `chore:`, `docs:`. The subject becomes the
  changelog line customers read, so write it for them: what changed, and what to do about it.
- **`ci:` and `test:` are hidden from the changelog.** `chore:`, `docs:`, `feat:` and `fix:` are not, and any of
  them cuts a release when the release PR is merged.
- **PR titles carry no type prefix**: `rust: <what changed>`. PRs land with a merge commit, and release-please
  reads a conventional PR title as one more commit, so a `feat:` title lists the change twice.
- **One logical change per PR.**
- **Never name a carrier or a channel provider** in code, comments, docs, changelogs or release notes.
