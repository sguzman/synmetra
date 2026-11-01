# Nomos / security/safety.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0001 — Do not publish private material to public repositories

**Version:** 1.1.1 · **Kind:** prohibition · **Authority:** repository_policy

Do not include private material in public Git repository artifacts or their associated publication surfaces.

**Predicate:** {"action":"include","object":"private_material","target":"artifact"}

**Normative conditions:** []

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":null,"cue":"Do not include private material","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["git_repository"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":[],"project_types":["any"]}

**Authority and overrides:** {"source":"Nomos publication constitution","tier":"repository_policy","priority":100,"override_policy":"non_overridable_within_scope"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["privacy","publication","security"],"topics":["content_handling","information_disclosure","private_public_boundary"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0001.json

---

## NOM-0003 — Do not disclose nonpublic information without authorization

**Version:** 1.1.1 · **Kind:** prohibition · **Authority:** general_guidance

Do not include confidential or otherwise nonpublic information in public outputs without specific authorization to disclose that information.

**Predicate:** {"action":"include","object":"nonpublic_information","target":"artifact"}

**Normative conditions:** ["Specific authorization to disclose the information has not been given"]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":[],"project_types":["any"]}

**Authority and overrides:** {"source":"Published in Nomos by explicit batch authorization; general guidance, never automatically applied","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["privacy","publication","security"],"topics":["content_handling","information_disclosure"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0003.json

---

## NOM-0050 — Public repository privacy review

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Review public-facing files, fixtures, metadata and commits for unauthorized nonpublic material.

**Predicate:** {"action":"practice","object":"public_repository_privacy_review","target":"authorized_project_work"}

**Normative conditions:** ["Before publishing repository changes","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Before publishing repository changes"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["privacy","publication","safety_integrity","software_development"],"topics":["public_privacy_review"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0050.json

---

## NOM-0051 — Keep secrets out of version control

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Avoid committing credentials, access tokens, private keys and nonpublic configuration.

**Predicate:** {"action":"practice","object":"keep_secrets_out_of_version_control","target":"authorized_project_work"}

**Normative conditions:** ["Whenever creating or staging repository content","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Whenever creating or staging repository content"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["privacy","safety_integrity","security","software_development"],"topics":["secret_management"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0051.json

---

## NOM-0052 — Non-destructive operational defaults

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Prefer inspection, bounded changes and explicit authorization before destructive operations.

**Predicate:** {"action":"practice","object":"non_destructive_operational_defaults","target":"authorized_project_work"}

**Normative conditions:** ["When modifying data, files or external systems","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When modifying data, files or external systems"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["safety_integrity","software_development"],"topics":["safe_operations"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0052.json

---

## NOM-0053 — Data round-trip integrity

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Verify preservation of data through import, export, migration, hashing or serialization where applicable.

**Predicate:** {"action":"practice","object":"data_round_trip_integrity","target":"authorized_project_work"}

**Normative conditions:** ["When handling transformations of valuable data","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When handling transformations of valuable data"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["data_integrity","safety_integrity","software_development"],"topics":["data_roundtrip"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0053.json

---

## NOM-0054 — Backwards compatibility checks

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Guard established public APIs, file formats and command behavior against unintended breaking changes.

**Predicate:** {"action":"practice","object":"backwards_compatibility_checks","target":"authorized_project_work"}

**Normative conditions:** ["When changing externally consumed interfaces","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When changing externally consumed interfaces"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["maintenance","safety_integrity","software_development"],"topics":["backward_compatibility"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0054.json

---

## NOM-0112 — Contain and verify necessary unsafe Rust and FFI boundaries

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

For each necessary unsafe Rust operation or C FFI boundary in an authorized C-to-Rust port, prefer safe Rust where feasible, isolate the boundary, document required safety and ABI invariants, and test the exposed contract.

**Predicate:** {"action":"contain_and_verify","object":"unsafe_and_ffi_boundaries","target":"c_to_rust_port"}

**Normative conditions:** ["An authorized C-to-Rust port uses Rust unsafe operations or an FFI boundary."]

**Normative exceptions:** ["No unsafe or FFI boundary is required when a fully safe Rust implementation can express the behavior."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each in-scope port verification or implementation cycle","cue":"For each","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized C-to-Rust port uses Rust unsafe operations or an FFI boundary."],"exclusions":["No unsafe or FFI boundary is required when a fully safe Rust implementation can express the behavior."],"project_types":["c_to_rust_port"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0030"],"depends_on":["NOM-0074"]}

**Categories:** {"domains":["rust_development","security","software_development","software_porting"],"topics":["rust_port_unsafe_ffi","unsafe_ffi_containment"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0112.json

---

## NOM-0236 — advisory disposition

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For each security advisory affecting the fork's supported upstream baseline or retained dependencies, determine actual applicability, record the chosen fix or mitigation, and verify its outcome before claiming the affected version is secure.

**Predicate:** {"action":"verify","object":"fork_advisory_disposition","target":"rust_fork_maintenance"}

**Normative conditions:** ["An owner-authorized Rust fork maintenance task has an accepted upstream baseline, and this concern falls within the maintained fork's supported contracts."]

**Normative exceptions:** ["Unpublished or unsupported upstream behavior is not automatically incorporated; approved scope and documented exceptions still govern."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each authorized applicable project acceptance","cue":"where the specified context is applicable","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An owner-authorized Rust fork maintenance task has an accepted upstream baseline, and this concern falls within the maintained fork's supported contracts."],"exclusions":["Unpublished or unsupported upstream behavior is not automatically incorporated; approved scope and documented exceptions still govern."],"project_types":["rust_fork_maintenance"]}

**Authority and overrides:** {"source":"Owner-authorized Nomos fork coverage remediation; opt-in only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0134"]}

**Categories:** {"domains":["rust_development","maintenance","security"],"topics":["rust_fork_selective_upstream_adoption"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0236.json
