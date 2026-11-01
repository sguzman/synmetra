# Nomos / ports/c-to-rust.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0111 — Differentially verify C and Rust program behavior

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

For each agreed reproducible behavior of an authorized C-to-Rust port, run the original C baseline and Rust implementation with equivalent inputs and relevant environment, compare observable outputs, exit codes and side effects, and document any intentionally accepted differences.

**Predicate:** {"action":"compare","object":"upstream_and_rust_observable_results","target":"c_to_rust_port"}

**Normative conditions:** ["An authorized C-to-Rust port has an agreed upstream baseline and equivalent test cases are runnable or reference observations are available."]

**Normative exceptions:** ["Documented accepted deviations and justified nondeterministic differences; missing baseline executability must be reported as a verification gap, not silently treated as a passing test."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each in-scope port verification or implementation cycle","cue":"For each","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized C-to-Rust port has an agreed upstream baseline and equivalent test cases are runnable or reference observations are available."],"exclusions":["Documented accepted deviations and justified nondeterministic differences; missing baseline executability must be reported as a verification gap, not silently treated as a passing test."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0074","NOM-0029"],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_development","software_porting","software_testing"],"topics":["differential_testing","rust_port_differential"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0111.json

---

## NOM-0113 — Inventory upstream capabilities and verified Rust port coverage

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

For each in-scope upstream C capability in an authorized C-to-Rust port, record its Rust implementation or replacement mapping, track implementation and verification status, and explicitly identify missing, incompatible or unverified behavior.

**Predicate:** {"action":"track","object":"upstream_capability_coverage","target":"c_to_rust_port"}

**Normative conditions:** ["An upstream baseline and the port's in-scope feature set have been identified."]

**Normative exceptions:** ["Out-of-scope features and accepted deviations remain explicitly recorded and are not silently reported as implemented."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each in-scope port verification or implementation cycle","cue":"For each","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An upstream baseline and the port's in-scope feature set have been identified."],"exclusions":["Out-of-scope features and accepted deviations remain explicitly recorded and are not silently reported as implemented."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0075","NOM-0064"],"depends_on":["NOM-0074","NOM-0075"]}

**Categories:** {"domains":["rust_development","software_development","software_porting"],"topics":["port_coverage_inventory","rust_port_coverage"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0113.json

---

## NOM-0200 — preprocessing

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify externally consumed C macro expansion, preprocessor conditionals and feature-test selections for each supported source build configuration.

**Predicate:** {"action":"verify","object":"c_preprocessing","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0200.json

---

## NOM-0201 — integer

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify C integer promotions, usual arithmetic conversions, signedness, widths, truncation and division or overflow behavior wherever the accepted baseline makes results observable.

**Predicate:** {"action":"verify","object":"c_integer","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0201.json

---

## NOM-0202 — floating

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify source-contractual C floating-point precision, exceptional values, environment flags and rounding behavior on each supported target.

**Predicate:** {"action":"verify","object":"c_floating","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0202.json

---

## NOM-0203 — characters

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify C character signedness, multibyte-to-byte conversions and exposed null-terminated string/byte semantics against the pinned source environment.

**Predicate:** {"action":"verify","object":"c_characters","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0203.json

---

## NOM-0204 — pointer arithmetic

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify C pointer arithmetic, one-past bounds, null treatment and reference validity at supported data and API boundaries.

**Predicate:** {"action":"verify","object":"c_pointer_arithmetic","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0204.json

---

## NOM-0205 — effective type

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify C effective-type and permitted aliasing guarantees for in-scope typed object accesses; avoid treating undefined aliasing as a parity target.

**Predicate:** {"action":"verify","object":"c_effective_type","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0205.json

---

## NOM-0206 — restrict

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify consumed C restrict-qualified pointer contracts and equivalent nonaliasing assumptions without introducing unpromised aliasing guarantees.

**Predicate:** {"action":"verify","object":"c_restrict","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0206.json

---

## NOM-0207 — storage lifetime

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify C automatic, static, thread-local and compound-literal storage lifetimes where callers observe object validity or cleanup timing.

**Predicate:** {"action":"verify","object":"c_storage_lifetime","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0207.json

---

## NOM-0208 — allocation

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify required malloc/calloc/realloc/free allocation, reallocation, ownership and failure semantics wherever existing callers or retained C APIs depend on them.

**Predicate:** {"action":"verify","object":"c_allocation","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0208.json

---

## NOM-0209 — aggregate layout

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify C structure, union and bit-field layout, alignment and representation assumptions where consumed across a binary boundary.

**Predicate:** {"action":"verify","object":"c_aggregate_layout","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0209.json

---

## NOM-0210 — flexible arrays

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify flexible-array-member sizes, alignment and valid access contracts, including variable-length array behavior where source interfaces require it.

**Predicate:** {"action":"verify","object":"c_flexible_arrays","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0210.json

---

## NOM-0211 — variadics

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify C variadic function calling conventions, default argument promotions and va_list handling for agreed source or retained ABI interfaces.

**Predicate:** {"action":"verify","object":"c_variadics","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0211.json

---

## NOM-0212 — callbacks

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify C function-pointer and callback signatures, calling conventions, context lifetimes and permitted invocation behavior at the Rust boundary.

**Predicate:** {"action":"verify","object":"c_callbacks","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0212.json

---

## NOM-0213 — linkage

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify externally consumed C header declarations, linkage, exported symbol visibility and public calling-interface compatibility.

**Predicate:** {"action":"verify","object":"c_linkage","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0213.json

---

## NOM-0214 — errno

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify documented C/POSIX error-status and errno propagation when part of the accepted consumer contract; do not assume errno is universal.

**Predicate:** {"action":"verify","object":"c_errno","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0214.json

---

## NOM-0215 — locale io

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify exposed C formatted input/output and locale-sensitive conversion results only where the original program promises those behaviors.

**Predicate:** {"action":"verify","object":"c_locale_io","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0215.json

---

## NOM-0216 — atomics

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify applicable C11-or-later atomic modification ordering, visibility and synchronization guarantees without assuming unspecified execution order.

**Predicate:** {"action":"verify","object":"c_atomics","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0216.json

---

## NOM-0217 — volatile io

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Where volatile-qualified or memory-mapped I/O is required, verify observable access ordering and side effects while preserving the distinction between volatile and synchronization.

**Predicate:** {"action":"verify","object":"c_volatile_io","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0217.json

---

## NOM-0218 — nonlocal jumps

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify accepted setjmp/longjmp nonlocal control-flow and resource lifetime requirements where they are used; exclude undefined jumps from parity claims.

**Predicate:** {"action":"verify","object":"c_nonlocal_jumps","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0218.json

---

## NOM-0219 — signals

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify signal-handler constraints and observable signal-related behavior where source and target integrations rely on defined signal handling.

**Predicate:** {"action":"verify","object":"c_signals","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0219.json

---

## NOM-0220 — threads

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify in-scope C thread synchronization, mutex/condition behavior and thread-local state visibility for supported concurrent consumers.

**Predicate:** {"action":"verify","object":"c_threads","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0220.json

---

## NOM-0221 — defined behavior

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Classify undefined, unspecified and implementation-defined C behavior before setting the Rust compatibility target; pin platform-specific accepted outcomes and do not claim universal parity.

**Predicate:** {"action":"verify","object":"c_defined_behavior","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0221.json

---

## NOM-0222 — binary bytes

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify contractual C object-byte representation, endianness and padding assumptions for externally consumed binary files, packets and buffers.

**Predicate:** {"action":"verify","object":"c_binary_bytes","target":"c_to_rust_port"}

**Normative conditions:** ["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."]

**Normative exceptions:** ["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant authorized C port comparison","cue":"For each applicable accepted contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized C-to-Rust strict port includes this behavior in its pinned C upstream baseline or externally consumed contract."],"exclusions":["Behavior not present or not promised in the agreed source baseline requires no separate emulation; unapproved or undefined behavior is never silently recorded as verified parity."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C strict-port coverage remediation; opt-in rules only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0222.json
