# Nomos / ports/common.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0074 — Maintain behavioral fidelity during strict software ports

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

During a strict software port, prioritize the agreed upstream application's observable behavior over redesign and unrelated enhancements.

**Predicate:** {"action":"preserve","object":"upstream_observable_behavior","target":"strict_software_port"}

**Normative conditions:** ["A strict implementation port has been authorized with a defined upstream baseline."]

**Normative exceptions:** ["Explicitly authorized deviations from the upstream baseline."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A strict implementation port has been authorized with a defined upstream baseline."],"exclusions":["Explicitly authorized deviations from the upstream baseline."],"project_types":["software_port"]}

**Authority and overrides:** {"source":"Reviewed Nomos catalog guidance; opt-in and subject to task-level authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["software_development","software_porting"],"topics":["port_fidelity","strict_port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0074.json

---

## NOM-0075 — Define porting MVP through verified replacement capability

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

For a strict software port, judge MVP against the agreed capability to replace the upstream application's baseline functionality rather than successful compilation alone.

**Predicate:** {"action":"verify","object":"replacement_capability","target":"strict_port_mvp"}

**Normative conditions:** ["Strict port MVP criteria and an upstream baseline have been agreed."]

**Normative exceptions:** ["Compilation alone is not proof of replacement capability."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Strict port MVP criteria and an upstream baseline have been agreed."],"exclusions":["Compilation alone is not proof of replacement capability."],"project_types":["software_port"]}

**Authority and overrides:** {"source":"Reviewed Nomos catalog guidance; opt-in and subject to task-level authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["software_development","software_porting"],"topics":["port_mvp"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0075.json

---

## NOM-0076 — Defer nonessential enhancements until strict-port MVP

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Keep redesigns and optional post-MVP features outside the strict porting phase until the agreed MVP is reached, unless a separately authorized exception changes scope.

**Predicate:** {"action":"defer","object":"optional_post_mvp_enhancements","target":"strict_port_work"}

**Normative conditions:** ["A strict port is not yet at its agreed replacement-capable MVP."]

**Normative exceptions:** ["Independently authorized pre-MVP additions are outside this default deferral."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A strict port is not yet at its agreed replacement-capable MVP."],"exclusions":["Independently authorized pre-MVP additions are outside this default deferral."],"project_types":["software_port"]}

**Authority and overrides:** {"source":"Reviewed Nomos catalog guidance; opt-in and subject to task-level authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0075"]}

**Categories:** {"domains":["software_development","software_porting"],"topics":["post_mvp_scope"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0076.json

---

## NOM-0138 — Record the authoritative upstream revision and supported comparison environment…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Record the authoritative upstream revision and supported comparison environment for a strict port.

**Predicate:** {"action":"verify","object":"execution_port_01_0","target":"software_port"}

**Normative conditions:** ["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["software_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0138.json

---

## NOM-0140 — in-scope upstream capability, maintain its Rust replacement mapping and…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For each in-scope upstream capability, maintain its Rust replacement mapping and independently verified status, explicitly accounting for gaps.

**Predicate:** {"action":"verify","object":"baseline_port_02_0","target":"software_port"}

**Normative conditions:** ["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["software_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0140.json

---

## NOM-0141 — Compare equivalent upstream and Rust observations for each reproducible in-scope…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Compare equivalent upstream and Rust observations for each reproducible in-scope behavior and record unresolved or unavailable comparisons.

**Predicate:** {"action":"verify","object":"testing_port_03_0","target":"software_port"}

**Normative conditions:** ["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["software_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0141.json

---

## NOM-0142 — Link each intentional divergence to an explicit approval and affected consumer…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Link each intentional divergence to an explicit approval and affected consumer contract.

**Predicate:** {"action":"verify","object":"governance_port_04_0","target":"software_port"}

**Normative conditions:** ["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["software_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0142.json

---

## NOM-0143 — Classify unresolved divergences as unverified or incompatible rather than verified…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Classify unresolved divergences as unverified or incompatible rather than verified parity.

**Predicate:** {"action":"verify","object":"governance_port_04_1","target":"software_port"}

**Normative conditions:** ["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["software_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0143.json

---

## NOM-0144 — necessary unsafe or FFI boundary, establish and verify its safety, lifetime and…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For each necessary unsafe or FFI boundary, establish and verify its safety, lifetime and ABI contract, isolating the boundary where feasible.

**Predicate:** {"action":"verify","object":"interop_port_06_0","target":"software_port"}

**Normative conditions:** ["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["software_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0144.json

---

## NOM-0145 — Verify supported installation, runtime dependency and launch/invocation behavior…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify supported installation, runtime dependency and launch/invocation behavior when replacing a deployed upstream distribution.

**Predicate:** {"action":"verify","object":"deployment_port_07_0","target":"software_port"}

**Normative conditions:** ["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["software_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0145.json

---

## NOM-0146 — deployment replaces existing state, verify authorized upgrade, recovery and…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When deployment replaces existing state, verify authorized upgrade, recovery and rollback procedures against representative state.

**Predicate:** {"action":"verify","object":"deployment_port_07_1","target":"software_port"}

**Normative conditions:** ["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized strict software port has an agreed upstream source baseline and this concern falls within its accepted replacement behavior."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["software_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0146.json
