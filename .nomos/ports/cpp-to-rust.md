# Nomos / ports/cpp-to-rust.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0176 — baseline

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Pin the accepted C++ language dialect, compiler and standard library version, build flags, and ABI environment needed to reproduce in-scope upstream behavior.

**Predicate:** {"action":"verify","object":"cpp_baseline","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0176.json

---

## NOM-0177 — ownership

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify transfer, sharing, and non-owning alias guarantees at supported C++ object interfaces before substituting Rust ownership.

**Predicate:** {"action":"verify","object":"cpp_ownership","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0177.json

---

## NOM-0178 — lifetime

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify reference and pointer validity, temporary lifetime extension, and observable invalidation at supported C++ object boundaries.

**Predicate:** {"action":"verify","object":"cpp_lifetime","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0178.json

---

## NOM-0179 — raii

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify required RAII resource release, destructor side effects, and observable destruction order during normal exits.

**Predicate:** {"action":"verify","object":"cpp_raii","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0179.json

---

## NOM-0180 — copy

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify observable C++ copy-construction and copy-assignment semantics, including ownership and independence of copied state.

**Predicate:** {"action":"verify","object":"cpp_copy","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0180.json

---

## NOM-0181 — move

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify required move-construction and move-assignment results, including documented moved-from guarantees, without assuming every moved-from value is empty.

**Predicate:** {"action":"verify","object":"cpp_move","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0181.json

---

## NOM-0182 — exceptions

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify externally observable C++ exception types, propagation, and the approved translation to Rust errors at compatible interfaces.

**Predicate:** {"action":"verify","object":"cpp_exceptions","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0182.json

---

## NOM-0183 — unwind

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify required cleanup and object destruction during exception unwinding and equivalent Rust failure paths.

**Predicate:** {"action":"verify","object":"cpp_unwind","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0183.json

---

## NOM-0184 — noexcept

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Where noexcept or termination behavior is externally contractual, verify no-throw boundaries and required termination outcomes without allowing exceptions across unsupported FFI.

**Predicate:** {"action":"verify","object":"cpp_noexcept","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0184.json

---

## NOM-0185 — templates

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify consumed C++ template instantiation and constraint acceptance/rejection behavior for supported source/API combinations.

**Predicate:** {"action":"verify","object":"cpp_templates","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0185.json

---

## NOM-0186 — overloads

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify accepted overload selection and implicit conversion effects at externally consumed C++ APIs.

**Predicate:** {"action":"verify","object":"cpp_overloads","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0186.json

---

## NOM-0187 — value categories

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify relevant C++ value-category and forwarding behavior when it changes supported public API results or ownership transitions.

**Predicate:** {"action":"verify","object":"cpp_value_categories","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0187.json

---

## NOM-0188 — virtual dispatch

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify observable virtual override dispatch and inheritance behavior of supported polymorphic interfaces.

**Predicate:** {"action":"verify","object":"cpp_virtual_dispatch","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0188.json

---

## NOM-0189 — rtti

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify public RTTI and checked downcast behavior where consumers depend on dynamic_cast, typeid, or equivalent type discovery.

**Predicate:** {"action":"verify","object":"cpp_rtti","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0189.json

---

## NOM-0190 — object layout

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Where native object representations cross a binary boundary, verify relevant field layout, size, alignment and permitted representation assumptions against the pinned C++ ABI.

**Predicate:** {"action":"verify","object":"cpp_object_layout","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0190.json

---

## NOM-0191 — binary abi

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify required calling conventions, exposed binary symbols, and toolchain-specific name-mangling compatibility across retained C++/Rust binary interfaces.

**Predicate:** {"action":"verify","object":"cpp_binary_abi","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0191.json

---

## NOM-0192 — cross language allocation

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify allocation and deallocation ownership, exception barriers and object lifetime across necessary C++/Rust interop boundaries.

**Predicate:** {"action":"verify","object":"cpp_cross_language_allocation","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0192.json

---

## NOM-0193 — container invalidation

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Where consumers rely on C++ container/iterator contracts, verify invalidation, iteration and reference-stability behavior at exposed boundaries.

**Predicate:** {"action":"verify","object":"cpp_container_invalidation","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0193.json

---

## NOM-0194 — atomics

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify required atomic modification ordering, acquire/release visibility, and synchronization guarantees for in-scope concurrent behavior; do not compare unpromised scheduling order.

**Predicate:** {"action":"verify","object":"cpp_atomics","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0194.json

---

## NOM-0195 — thread sync

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify accepted mutex, lock, condition-variable, and wakeup/predicate behavior for supported multithreaded interactions.

**Predicate:** {"action":"verify","object":"cpp_thread_sync","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0195.json

---

## NOM-0196 — static lifecycle

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify observable initialization and teardown ordering for supported static and thread-local objects when it affects externally visible behavior.

**Predicate:** {"action":"verify","object":"cpp_static_lifecycle","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0196.json

---

## NOM-0197 — coroutines

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Where C++20 coroutines are part of the accepted upstream interface, verify resume, suspend, cancellation and destruction contracts in the Rust replacement.

**Predicate:** {"action":"verify","object":"cpp_coroutines","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0197.json

---

## NOM-0198 — defined behavior

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Distinguish specified C++ behavior from undefined, implementation-defined and unspecified behavior when setting compatibility acceptance; document environment-specific decisions rather than asserting impossible universal parity.

**Predicate:** {"action":"verify","object":"cpp_defined_behavior","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0198.json

---

## NOM-0199 — numeric

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Verify supported C++ integer promotions, conversion widths, signedness, floating-point and rounding outcomes wherever consumers observe them.

**Predicate:** {"action":"verify","object":"cpp_numeric","target":"cpp_to_rust_port"}

**Normative conditions:** ["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."]

**Normative exceptions:** ["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each relevant C++ port verification","cue":"each applicable contract","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized C++-to-Rust strict port includes this C++ behavior in its agreed upstream baseline, exposed interface, or accepted compatibility contract."],"exclusions":["Excluded or unobservable behavior need not be emulated; permitted deviations must be disclosed; no parity promise is inferred for undefined behavior."],"project_types":["cpp_to_rust_port"]}

**Authority and overrides:** {"source":"Owner-authorized C++ strict-port coverage remediation; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","software_porting","software_testing"],"topics":["port_fidelity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0199.json
