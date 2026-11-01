# Nomos / ports/python-to-rust.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0139 — For Python source ports, identify interpreter version/build and relevant…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For Python source ports, identify interpreter version/build and relevant implementation features in the accepted source baseline.

**Predicate:** {"action":"verify","object":"execution_port_01_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0139.json

---

## NOM-0147 — Verify agreed identity and alias-visible mutation behavior for Python objects…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify agreed identity and alias-visible mutation behavior for Python objects crossing supported interfaces.

**Predicate:** {"action":"verify","object":"data_model_py_01_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0147.json

---

## NOM-0148 — Verify externally consumed Python attribute, reflection and dynamic type behavior

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify externally consumed Python attribute, reflection and dynamic type behavior.

**Predicate:** {"action":"verify","object":"data_model_py_02_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0148.json

---

## NOM-0149 — Verify callable signature and invocation/dispatch compatibility at supported…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify callable signature and invocation/dispatch compatibility at supported Python-facing boundaries.

**Predicate:** {"action":"verify","object":"data_model_py_02_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0149.json

---

## NOM-0150 — Verify Python truth-testing outcomes at in-scope decision boundaries

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify Python truth-testing outcomes at in-scope decision boundaries.

**Predicate:** {"action":"verify","object":"data_model_py_03_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0150.json

---

## NOM-0151 — Verify Python implicit and explicit conversion outcomes at in-scope interfaces

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify Python implicit and explicit conversion outcomes at in-scope interfaces.

**Predicate:** {"action":"verify","object":"data_model_py_03_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0151.json

---

## NOM-0152 — Verify integer range, division and overflow-equivalent behavior where source…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify integer range, division and overflow-equivalent behavior where source integer semantics are exposed.

**Predicate:** {"action":"verify","object":"numbers_py_04_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0152.json

---

## NOM-0153 — Verify floating-point special values, comparison and rounding where externally…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify floating-point special values, comparison and rounding where externally consequential.

**Predicate:** {"action":"verify","object":"numbers_py_04_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0153.json

---

## NOM-0154 — Verify Decimal precision, context and rounding where the upstream uses decimal…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify Decimal precision, context and rounding where the upstream uses decimal arithmetic.

**Predicate:** {"action":"verify","object":"numbers_py_04_2","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0154.json

---

## NOM-0155 — Verify in-scope Unicode string operations and code-point semantics

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify in-scope Unicode string operations and code-point semantics.

**Predicate:** {"action":"verify","object":"text_py_05_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0155.json

---

## NOM-0156 — Verify byte/text encoding, decoding and error-policy behavior at exposed interfaces

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify byte/text encoding, decoding and error-policy behavior at exposed interfaces.

**Predicate:** {"action":"verify","object":"text_py_05_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0156.json

---

## NOM-0157 — Verify in-scope equality, hashing and lookup-key behavior

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify in-scope equality, hashing and lookup-key behavior.

**Predicate:** {"action":"verify","object":"data_model_py_06_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0157.json

---

## NOM-0158 — Verify exposed collection iteration and ordering guarantees supported by the…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify exposed collection iteration and ordering guarantees supported by the agreed upstream baseline.

**Predicate:** {"action":"verify","object":"data_model_py_06_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0158.json

---

## NOM-0159 — Verify externally observable exception categories, propagation, failure returns…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify externally observable exception categories, propagation, failure returns and contractually exposed messages.

**Predicate:** {"action":"verify","object":"errors_py_07_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0159.json

---

## NOM-0160 — Verify in-scope iterator/generator lazy yield, termination and exception…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify in-scope iterator/generator lazy yield, termination and exception propagation behavior.

**Predicate:** {"action":"verify","object":"protocols_py_08_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0160.json

---

## NOM-0161 — Verify in-scope Python context-management resource lifecycle and exceptional-exit…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify in-scope Python context-management resource lifecycle and exceptional-exit behavior.

**Predicate:** {"action":"verify","object":"protocols_py_09_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0161.json

---

## NOM-0162 — Verify import/module name resolution and failure behavior where consumers rely on…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify import/module name resolution and failure behavior where consumers rely on Python imports.

**Predicate:** {"action":"verify","object":"ecosystem_py_10_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0162.json

---

## NOM-0163 — Verify distribution entry points, script commands and invocation contracts

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify distribution entry points, script commands and invocation contracts.

**Predicate:** {"action":"verify","object":"ecosystem_py_11_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0163.json

---

## NOM-0164 — Verify declared plugin registration, discovery and loading contracts where supported

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify declared plugin registration, discovery and loading contracts where supported.

**Predicate:** {"action":"verify","object":"ecosystem_py_11_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0164.json

---

## NOM-0165 — Verify in-scope asynchronous cancellation, timeout and failure-cleanup behavior

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify in-scope asynchronous cancellation, timeout and failure-cleanup behavior.

**Predicate:** {"action":"verify","object":"concurrency_py_12_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0165.json

---

## NOM-0166 — Verify promised task ordering, concurrency-limit and backpressure outcomes under…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify promised task ordering, concurrency-limit and backpressure outcomes under representative load.

**Predicate:** {"action":"verify","object":"concurrency_py_12_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0166.json

---

## NOM-0167 — Verify in-scope shared-state synchronization, visibility and race-sensitive behavior

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify in-scope shared-state synchronization, visibility and race-sensitive behavior.

**Predicate:** {"action":"verify","object":"concurrency_py_13_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0167.json

---

## NOM-0168 — Verify interprocess communication, isolation and process lifecycle where the…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify interprocess communication, isolation and process lifecycle where the Python baseline uses processes.

**Predicate:** {"action":"verify","object":"concurrency_py_13_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0168.json

---

## NOM-0169 — Verify supported Python-callable interface behavior using an independent consumer

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify supported Python-callable interface behavior using an independent consumer.

**Predicate:** {"action":"verify","object":"interop_py_14_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0169.json

---

## NOM-0170 — Verify binary extension/embedding ABI and lifetime invariants only where compiled…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify binary extension/embedding ABI and lifetime invariants only where compiled interoperation is part of the accepted baseline.

**Predicate:** {"action":"verify","object":"interop_py_14_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0170.json

---

## NOM-0171 — Verify externally consumed serialized representations against the source contract,…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify externally consumed serialized representations against the source contract, including exact bytes only when those bytes are contractual.

**Predicate:** {"action":"verify","object":"data_py_15_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0171.json

---

## NOM-0172 — Verify in-scope filesystem, path, permission and environment-variable contracts

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify in-scope filesystem, path, permission and environment-variable contracts.

**Predicate:** {"action":"verify","object":"os_py_16_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0172.json

---

## NOM-0173 — Verify subprocess arguments, streams, status codes and lifecycle contracts where…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify subprocess arguments, streams, status codes and lifecycle contracts where invoked.

**Predicate:** {"action":"verify","object":"os_py_16_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0173.json

---

## NOM-0174 — Verify accepted historical Python-era persisted files/configurations are read…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify accepted historical Python-era persisted files/configurations are read compatibly.

**Predicate:** {"action":"verify","object":"data_py_18_0","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0174.json

---

## NOM-0175 — migration occurs, verify transformation and available recovery/rollback against…

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Where migration occurs, verify transformation and available recovery/rollback against representative retained state.

**Predicate:** {"action":"verify","object":"data_py_18_1","target":"python_to_rust_port"}

**Normative conditions:** ["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."]

**Normative exceptions:** ["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable in-scope port verification","cue":"For each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An explicitly authorized Python-to-Rust strict port includes this behavior in its agreed source baseline or promised public contract."],"exclusions":["Unexposed or explicitly excluded upstream behavior does not require separate emulation; accepted departures must be recorded and must not be represented as verified parity."],"project_types":["python_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized strict-port coverage remediation; expressly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0175.json
