# 🧪 Reproducible hostile-input mutation corpus

**Scope:** offline, deterministic boundary tests for existing T188. The corpus is part of the default Rust test suite and does not require Google OAuth, a local GA4 account, a server listener, Python, npm, Node, fuzz servers, or private data. No production fuzz-safety certification is implied.

## Seeded test families

| Source test | Fixed seeds (hexadecimal) | Cases per seed | Safety assertions |
| --- | --- | ---: | --- |
| `src/tabular.rs::seeded_resource_and_cursor_mutations_fail_closed_without_panics` | `7B13C451F239A02D`, `13C400D5AABB9F17`, `EADBEEF045678913` | 256 | Invalid resource IDs and modified query-bound pagination cursors are denied; attempted report body remains unmodified |
| `src/google.rs::seeded_json_and_filter_mutations_are_bounded_and_panic_free` | `C0DE42F019A588E1`, `000000000001CAFE`, `9A3D75F1B2C40E87` | 128 | Trailing-NUL malformed JSON, oversized text filters and report preflight are rejected locally |

Both use a fixed `xorshift64` mutation generator scoped to the test, with a known ASCII punctuation alphabet and explicit maximum lengths. A panicking parser is a failing test, not merely an error return. Seeds and the failing iteration are included in the test assertion, making a failure reproducible without writing the potentially sensitive input into CI logs.

The corpus supplements the existing exact hostile SQL and filesystem tests in `src/scratchpad.rs`, fixed adversarial Google resource/filter fixtures in `src/google.rs`, and deterministic cross-resource cursor fixtures in `src/tabular.rs`. It does not use random system entropy, unrestricted path access or live provider requests. Fixed seeds and bounded mutation ranges keep the suite suitable for native CI.

## Coverage and limitations

These are **seeded mutation fixtures**, not exhaustive coverage-guided fuzzing, formal verification, proof of zero crashes, a memory sanitizer, or validation of arbitrary untrusted bytes under every Rust/Google/DuckDB release. The corpus intentionally concentrates on **known invalid input classes**; it does not establish that every valid Google filter or report produces identical upstream behavior.

To rerun the two named tests in an isolated developer workspace with Rust 1.99.1:

```fish
cargo +1.99.1 test --locked seeded_resource_and_cursor_mutations_fail_closed_without_panics
cargo +1.99.1 test --locked seeded_json_and_filter_mutations_are_bounded_and_panic_free
```

Standard CI runs both through `cargo test --locked`. Security gates for credential isolation, native DuckDB sandboxing, authorized live OAuth and complete third-party MCP interoperability remain independent in [progress.json](progress.json) and [ROADMAP.md](ROADMAP.md).
