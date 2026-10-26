# 🧪 Differential payload projection audit: pinned Contract V1

**Scope:** the pinned SednaLabs Rust reference `039fd4620064fc4b76dc6bca641945aba52c14bc`, compared with Synmetra's local synthetic tabular projection. This is a *source-derived fixture comparison*; the upstream executable has not been launched and no real Google Analytics account was queried. See roadmap **T171**, whose independently generated upstream executable golden-fixture gate remains open.

## Pinned evidence

The upstream source [`tests/contract_safety_conformance.rs`](https://github.com/sednalabs/ga4-mcp/blob/039fd4620064fc4b76dc6bca641945aba52c14bc/tests/contract_safety_conformance.rs) asserts `contract_success_tabular_emits_required_v1_fields` using compact output mode, `summary_only=false`, total 100, returned 25, `truncated=true`, cursor `v1:abcd:25`, hash `abcd` and numeric `elapsed_ms=8`. Those seven shared metadata values have been transcribed without extrapolating unsupported semantics to the machine-readable [expectation fixture](../spec/fixtures/sednalabs-tabular-contract-expected.v1.json).

Synmetra's `src/tabular.rs` test `pinned_sednalabs_contract_fields_match_native_bounded_projection` generates 25 synthetic `page_view` rows and projects the same compact metadata fields. The test compares each expected field to the native result and ensures `elapsed_ms=0` is accompanied by `elapsed_ms_measured=false`. The native input is 25 rows instead of the upstream test's single synthetic data row because the upstream test sets `row_count_returned=25` independently of that one-row data payload. This prevents a false data-count equivalence assertion.

## Comparison and unresolved differences

| Contract aspect | Pinned upstream assertion | Native Synmetra | Status |
| --- | --- | --- | --- |
| Success `ok` | `true` | `true` in the outer MCP envelope | Supported by separate envelope tests |
| `output_mode`, `summary_only` | `compact`, `false` | Same | Compared in the pinned-field test |
| Row count total / returned | `100` / `25` | Same on equivalent 25-row synthetic input | Compared |
| Truncation | `true` | `true` | Compared |
| Next cursor / query hash | `v1:abcd:25` / `abcd` | Same | Compared |
| Elapsed time | `8` (supplied to reference contract) | `0` with `elapsed_ms_measured=false` | Intentional difference; actual latency not measured |
| Nested `data` shape | Reference fixture passes a one-row JSON array even while metadata says 25 returned | Native compact projection contains columns, tuples and reported row count | Not equivalent; further baseline execution/normalization needed |
| Policy-denied error | Upstream `POLICY_DENIED`, `reason=policy_denied`, `category=policy`, redacted `detail` and `hint` | Native typed errors use a different generic envelope and message mapping | Not accepted as error-contract parity |
| Unknown response fields | Upstream baseline policy varies by tool | Native GA report results preserve selected provider quota/metadata under `meta.ga4` | Tool-by-tool baseline execution still required |

The same pinned upstream test module covers SQL-policy rejection and cursor-query mismatch error naming. Those cases are identified as **future differential fixtures**, not already equivalent responses. This audit makes no claim of 36-tool response parity, live role validation, Google API alpha eligibility or real Codex/Inspector interoperability.

## Next acceptance work

A fully accepted T171 still requires running the **actual pinned SednaLabs baseline** against synthetic/mocked inputs, capturing its complete structured responses without credentials, normalizing volatile timestamps and elapsed durations, and comparing complete payloads and error categories against Synmetra's equivalent native calls. Only then can second-gate golden fixtures be credited. The source expectation fixture is useful, reviewed evidence for the first gate only.
