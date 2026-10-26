# 🔬 Pinned upstream response differential — first golden set (T171)

**Acceptance state:** source-derived synthetic differential fixtures, *not* execution of SednaLabs' binary, exhaustive normalized output equivalence, or authorized Google Analytics live QA. The frozen source is `sednalabs/ga4-mcp@039fd4620064fc4b76dc6bca641945aba52c14bc`. The machine-readable cases live in [sednalabs-response-differential.v1.json](../spec/fixtures/sednalabs-response-differential.v1.json); `src/tabular.rs::pinned_upstream_response_goldens_expose_shared_fields_and_real_divergences` compares the generated native projections.

## Source-grounded positive comparisons

| Case | Pinned upstream source | Shared normalized projection | Native difference deliberately **not** ignored |
| --- | --- | --- | --- |
| Historical compact, total 100 and returned 25 | `tests/contract_safety_conformance.rs::contract_success_tabular_emits_required_v1_fields` (lines 41–63) | `output_mode=compact`, `summary_only=false`, count/truncation, query hash and `v1:abcd:25` next cursor | Pinned test supplies a one-item source data array but independently declares 25 returned; native compact builds a column/tuple object from 25 synthetic provider rows. Full `data` values are **not identical** |
| Summary-only rows, total 10 and returned 5 | `src/contract.rs::tests::success_tabular_respects_summary_only` (lines 270–285) | `summary_only=true`, null data, counts and next cursor `v1:abc:5` | Pinned builder receives one supplied row but separately reports five returned. Native derives the count from five synthetic input rows |

The fixture preserves upstream test names, literal asserted shared values, and the native analog inputs. It does **not** assume the upstream service performed a GA4 request: SednaLabs' `contract::success_tabular` wraps user-supplied data and `TabularMeta`, while Synmetra's `tabular::project` transforms a bounded provider-shaped fixture. Shared metadata is compared value-for-value; the distinct input/output mechanisms are written down rather than silently normalized into false parity.

## Negative comparisons and unresolved contract differences

The same pinned source asserts `POLICY_DENIED`/`policy_denied`/`policy` with the detail `profile=read_only;tool=scratchpad_query`; it also asserts `CURSOR_QUERY_MISMATCH`/`invalid_cursor`/`cursor`. Synmetra now distinguishes a structurally valid but cross-property/cross-query SHA-256 cursor as `CURSOR_QUERY_MISMATCH`/`invalid_cursor`/`cursor`, matching those **three fields** from the pinned upstream source. Malformed cursors (bad version, malformed hash or offset) still map to generic `INVALID_REQUEST`/`validation`, so the broader cursor and validation taxonomy remains unaccepted. The policy-denial path likewise requires end-to-end profile-equivalence testing; the fixture explicitly marks that comparison unaccepted.

Upstream `elapsed_ms` is supplied as a measured elapsed duration in source tests (8–9 ms in those synthetic calls); native code deliberately marks `elapsed_ms=0` and `elapsed_ms_measured=false` until actual per-tool measurement is implemented. The differential therefore excludes elapsed-time equality **and records the exclusion**. Pinned upstream column metadata has an optional `logical_type` and `nullable`; the native GA4 projection adds inferred header type/nullable metadata. The combined fields are not proven fully equivalent.

## Progress criteria

The first source-backed fixture established the differential harness. The second gate, **normalized golden fixtures generated from the pinned baseline**, is now accepted from the completed executed upstream CI output and the passing native golden comparison (see the source-executed section below). Neither acceptance establishes all 36 tool outputs or live Google API behavioral equivalence. The broader T156–T159 response and cursor parity gates, T161 client conformance, and T178–T186 live Google gates remain separately unaccepted.

All examples use synthetic `event_name=page_view` and fixed counters; no account identifiers, OAuth credentials, private analytics, consent evidence or release approval are embedded.

## Source-executed baseline capture (additional gate)

The independent [pinned upstream CI run](https://github.com/sguzman/synmetra/actions/runs/40420006635) checked out exactly `sednalabs/ga4-mcp@039fd4620064fc4b76dc6bca641945aba52c14bc`, compiled its own Rust `ga4_mcp::contract` module with `cargo +1.99.1 test --locked`, and executed the four **synthetic** Contract V1 output builders defined in `ci/upstream-contract-golden.rs`. The resulting full [machine-readable executed upstream output](../spec/fixtures/sednalabs-executed-contract-v1.json) is now captured from that successful run. `src/tabular.rs::executed_pinned_upstream_goldens_match_only_shared_contract_fields` passed native core CI in [run 40422223509](https://github.com/sguzman/synmetra/actions/runs/40422223509), satisfying T171's fixture-comparison gate without claiming complete tool parity.

One important correction to the earlier source-derived fixture: **summary-only upstream `query_hash` was `null`**, even though `next_cursor` was `v1:abc:5`. The pinned source test sets the cursor but never sets `TabularMeta.query_hash`. Native Synmetra produces its own `query_hash="abc"` in the analogous projection. Both still match the shared `output_mode`, `summary_only`, total/returned counts, truncation, and next cursor. The source-derived summary case was corrected to **exclude** `query_hash` from the falsely asserted shared fields and to record each side's distinct actual value. Do **not** normalize the mismatch away.

Other outstanding differences also remain: upstream compact `data` is the caller-supplied one-row array while native compact produces a column-and-tuples structure; upstream `elapsed_ms` values are caller-measured synthetic durations (8/9 ms), whereas native `elapsed_ms=0` is explicitly unmeasured. The pinned policy-denied and cursor-query-mismatch examples are *captured and asserted*, but that alone does not prove full native profile-denial or malformed-cursor taxonomy equivalence. The comparison does not involve a real Google property, complete tool outputs or live authentication.
