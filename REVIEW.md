# Review guide

What a reviewer, human or agent, checks on every PR to this repository.

1. **The API first.** Does the change keep what `spec/openapi.json` says? The envelope, pagination by `page`
   and `page_size`, the headers, the error codes.
2. **Generated code untouched.** `src/models.rs`, `src/resources/`, the generated tests and `spec/` change
   only through the lane in `sent-dm-sdk-pipeline`. A PR editing them by hand is wrong by construction.
3. **Both checks green** on the PR's head commit: `test` and `contract`.
4. **No secrets in logs or errors.** An API key never reaches a log line or an error message.
5. **Compatibility.** A field, an enum value or a resource Sent adds later must not break a customer on this
   version: enums keep their `Other` fallback, structs ignore unknown fields.
6. **Changelog line.** A `feat:` or `fix:` subject a customer can read.
