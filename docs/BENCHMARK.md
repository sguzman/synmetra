# ⏱️ Synthetic Linux performance measurements

**Scope:** an evidence-producing local benchmark harness for the experimental default Rust CLI, addressing the first (implementation) gate of T189. It does **not** certify upstream parity, production latency service levels, real Google API throughput or a truly cold operating-system page cache.

Run the executable from the repository after a pinned `cargo +1.99.1 build --locked --bin synmetra`:

```fish
./ci/benchmark-native.sh target/debug/synmetra
```

The Linux-only script uses Bash internally, GNU `/usr/bin/time`, `jq`, `git` and ordinary coreutils. Fish users can execute it directly; they do not need Python, npm or Node. It runs entirely against synthetic offline local MCP calls, with `XDG_CONFIG_HOME` isolated in a temporary directory and Google/MCP bearer environment overrides removed. It deletes its own temporary files on exit.

## Profiles and metrics

| Profile | What is measured | Interpretation |
| --- | --- | --- |
| `fresh_help` | New process executing `synmetra --help`, timed across whole lifecycle | CLI startup/teardown wall time and peak resident KiB, including loader and locale overhead |
| `fresh_mcp_roundtrip` | New stdio process handling initialize, tools/list and one `ga4_get_started` call | Full synthetic lifecycle wall time and peak resident KiB; **not** per-tool runtime |
| `same_process_repeated_calls` | One initialized stdio process handling multiple `ga4_get_started` calls | Combined full session time and peak resident KiB, not divided into individual call latencies |

The default harness executes five independent process rounds (configurable via `SYNMETRA_BENCH_RUNS` from 1–20). It verifies actual JSON-RPC response counts, 20 default read-only tool registrations and successful structured call responses before emitting results. It prints one JSON record containing the tested source commit, kernel, binary path, limitations, and all raw measurements. CI should capture this output, not invent hard-coded times or claim improvements without before/after measurements on comparable runners.

Because Linux file/dynamic loader caches persist across runs, a *fresh process* is **not** a true `drop_caches` cold run. This harness deliberately does not flush global kernel caches, require root, make live Google requests, or report private tokens. The second T189 acceptance check (repeatable verified cold/no-cache and warm-cache comparisons) stays open until that distinct methodology is independently demonstrated.

Memory figures are process max RSS from GNU time (`%M` KiB); they are not cumulative disk, VRAM, host peak or allocator-only memory. Elapsed wall time (`%e`) includes scheduling jitter. Synthetic setup and single local caller cannot predict multi-user or live Google networking.
