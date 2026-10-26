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

Memory figures are process max RSS from GNU time (`%M` KiB); they are not cumulative disk, VRAM, host peak or allocator-only memory. GNU time's `elapsed_s` (`%e`) has only hundredth-second resolution and is unsuitable for very short startup comparisons. `wall_ms` uses Linux `date +%s%N` around the complete measured command and records milliseconds with three fractional digits. It includes process spawning and timer overhead, is wall-clock rather than monotonic, and must not be interpreted as isolated MCP handler latency. Synthetic setup and single local caller cannot predict multi-user or live Google networking.

### CI baseline 40336644502 (synthetic only)

The [passing core CI run 40336644502](https://github.com/sguzman/synmetra/actions/runs/40336644502), using source `b278dfd967725d3ca930f0ae6b1e585f0d9bc939` with default debug features on Linux `7.0.0-1017-azure x86_64`, produced a [machine-readable 11-measurement record](../spec/benchmarks/offline-40336644502.json). Median values over five fresh-process rounds were:

| Profile | Median full-command wall time | Median peak RSS | Samples |
| --- | ---: | ---: | ---: |
| Offline `--help` | **6.803 ms** | **9,204 KiB** | 5 |
| Initialize + list 20 tools + one synthetic MCP call | **12.593 ms** | **17,808 KiB** | 5 |
| One session, five synthetic calls | **14.896 ms total** | **17,692 KiB** | 1 |

The first help process measured **14.537 ms**, illustrating loader/scheduler/cache sensitivity. The benchmark is repeatable but these particular results represent **one CI runner, one source SHA, one debug build, warmed runner caches**, and a wall-clock timer including shell/process overhead. They do **not** establish per-tool request latency, a cold-cache run, release-build throughput or a speed improvement over either upstream server. T189's cold/warm comparison gate remains open.

## Runner-isolated page-cache experiment (acceptance pending)

The source-controlled `ci/benchmark-native.sh` now supports `SYNMETRA_BENCH_CACHE_MODE=page-cache-cleared` **only** when `GITHUB_ACTIONS=true`, `RUNNER_ENVIRONMENT=github-hosted` and `SYNMETRA_ALLOW_GLOBAL_CACHE_FLUSH=ci-only`. It must never reset cache on a user's workstation or on a self-hosted runner. The CI step runs three cache-cleared and three warm fresh-process rounds for both help and synthetic MCP round-trip profiles, plus one repeated-call process per mode.

For each cleared-cache measurement, the script runs `sync` and uses noninteractive CI-only `sudo` to write `3` to Linux `/proc/sys/vm/drop_caches`. If access is denied, the benchmark **fails closed** rather than silently labeling warm data as cold. This resets Linux page, dentry and inode caches; it does not clear CPU caches, control GitHub VM load, isolate shell startup overhead or claim a new hardware boot. The same compiled debug binary and single CI VM are used in both conditions. The warm samples run after the cleared-cache phase. Neither is an independent cross-machine production-performance baseline.

No performance comparison or T189 second-gate credit is claimed until the CI cache-reset step actually passes, the resulting values are checked, and a pinned machine-readable source-run record is committed. The earlier unqualified warmed-runner baseline remains separate.
