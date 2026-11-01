# Nomos / forks/upstream.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0133 — Verify affected fork behavior after upstream integration

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

After integrating authorized upstream changes into a Rust fork, run appropriate tests and focused compatibility or regression checks against the fork's promised interfaces, supported platforms and preserved customizations, and record any unresolved failures before describing the integration as complete.

**Predicate:** {"action":"verify","object":"post_integration_fork_behavior","target":"rust_fork_sync_acceptance"}

**Normative conditions:** ["After authorized upstream integration into an existing Rust fork."]

**Normative exceptions:** ["Use affected-behavior checks with the fork's established compatibility promises; untestable paths remain disclosed."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["rust_fork_maintenance"],"preconditions":["After authorized upstream integration into an existing Rust fork."],"exclusions":["Use affected-behavior checks with the fork's established compatibility promises; untestable paths remain disclosed."]}

**Authority and overrides:** {"source":"Explicitly reviewed public-safe specimen normalization; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0132"]}

**Categories:** {"domains":["qa_verification","software_testing"],"topics":["rust_fork_post_sync_verification"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0133.json

---

## NOM-0237 — reversible upstream sync

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Before an authorized upstream sync that changes installed state or persisted data, preserve a tested recoverable baseline and verify the relevant rollback or migration route; do not promise rollback when the state change is irreversible.

**Predicate:** {"action":"verify","object":"fork_reversible_upstream_sync","target":"rust_fork_maintenance"}

**Normative conditions:** ["An owner-authorized Rust fork maintenance task has an accepted upstream baseline, and this concern falls within the maintained fork's supported contracts."]

**Normative exceptions:** ["Unpublished or unsupported upstream behavior is not automatically incorporated; approved scope and documented exceptions still govern."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each authorized applicable project acceptance","cue":"where the specified context is applicable","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An owner-authorized Rust fork maintenance task has an accepted upstream baseline, and this concern falls within the maintained fork's supported contracts."],"exclusions":["Unpublished or unsupported upstream behavior is not automatically incorporated; approved scope and documented exceptions still govern."],"project_types":["rust_fork_maintenance"]}

**Authority and overrides:** {"source":"Owner-authorized Nomos fork coverage remediation; opt-in only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0132"]}

**Categories:** {"domains":["maintenance","data_integrity","qa_verification"],"topics":["rust_fork_conflict_reconciliation"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0237.json

---

## NOM-0238 — release source traceability

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For a distributed fork release, identify the fork revision, followed upstream revision, retained deviations and materially relevant imported fixes in public release metadata so downstream users can determine exactly what was shipped.

**Predicate:** {"action":"verify","object":"fork_release_source_traceability","target":"rust_fork_maintenance"}

**Normative conditions:** ["An owner-authorized Rust fork maintenance task has an accepted upstream baseline, and this concern falls within the maintained fork's supported contracts."]

**Normative exceptions:** ["Unpublished or unsupported upstream behavior is not automatically incorporated; approved scope and documented exceptions still govern."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each authorized applicable project acceptance","cue":"where the specified context is applicable","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An owner-authorized Rust fork maintenance task has an accepted upstream baseline, and this concern falls within the maintained fork's supported contracts."],"exclusions":["Unpublished or unsupported upstream behavior is not automatically incorporated; approved scope and documented exceptions still govern."],"project_types":["rust_fork_maintenance"]}

**Authority and overrides:** {"source":"Owner-authorized Nomos fork coverage remediation; opt-in only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0129"]}

**Categories:** {"domains":["maintenance","releases","documentation"],"topics":["rust_fork_upstream_provenance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0238.json
