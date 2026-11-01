# Nomos / quality/testing.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0024 — Rust formatting

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Check Rust source formatting with cargo fmt or equivalent toolchain-supported formatting.

**Predicate:** {"action":"practice","object":"rust_formatting","target":"authorized_project_work"}

**Normative conditions:** ["When Rust source changes","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When Rust source changes"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_quality","software_development"],"topics":["rust_formatting"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0024.json

---

## NOM-0025 — Rust Clippy linting

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Run Clippy on relevant targets and address actionable diagnostics; select warning strictness deliberately.

**Predicate:** {"action":"practice","object":"rust_clippy_linting","target":"authorized_project_work"}

**Normative conditions:** ["When Rust code is changed and tooling is available","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When Rust code is changed and tooling is available"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_quality","software_development"],"topics":["rust_linting"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0025.json

---

## NOM-0026 — Rust compilation checks

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Use cargo check or an appropriate build to verify compilation under the project's intended toolchain.

**Predicate:** {"action":"practice","object":"rust_compilation_checks","target":"authorized_project_work"}

**Normative conditions:** ["When Rust implementation changes","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When Rust implementation changes"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_quality","software_development"],"topics":["rust_compilation"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0026.json

---

## NOM-0027 — Unit tests

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Write and maintain focused automated tests for important functions, modules and invariants.

**Predicate:** {"action":"practice","object":"unit_tests","target":"authorized_project_work"}

**Normative conditions:** ["When implementation behavior is added or changed","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When implementation behavior is added or changed"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_quality","software_development","software_testing"],"topics":["unit_testing"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0027.json

---

## NOM-0028 — Integration tests

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Test interactions across components and public interfaces where integration boundaries warrant it.

**Predicate:** {"action":"practice","object":"integration_tests","target":"authorized_project_work"}

**Normative conditions:** ["When behavior spans components or interfaces","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When behavior spans components or interfaces"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_quality","software_development","software_testing"],"topics":["integration_testing"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0028.json

---

## NOM-0029 — Regression tests

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Reproduce reported defects with automated tests and retain them to detect recurrence.

**Predicate:** {"action":"practice","object":"regression_tests","target":"authorized_project_work"}

**Normative conditions:** ["When repairing reproducible bugs","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When repairing reproducible bugs"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_quality","software_development","software_testing"],"topics":["regression_testing"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0029.json

---

## NOM-0030 — Edge-case coverage

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Cover invalid states, malformed inputs, missing resources and failure paths relevant to the feature.

**Predicate:** {"action":"practice","object":"edge_case_coverage","target":"authorized_project_work"}

**Normative conditions:** ["When developing or validating behavior","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When developing or validating behavior"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_quality","software_development","software_testing"],"topics":["edge_case_testing"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0030.json

---

## NOM-0031 — Synthetic fixtures

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Prefer minimal synthetic, correctly licensed fixtures over personal, secret or commercial inputs.

**Predicate:** {"action":"practice","object":"synthetic_fixtures","target":"authorized_project_work"}

**Normative conditions:** ["When tests require sample data","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When tests require sample data"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_quality","software_development","software_testing"],"topics":["synthetic_test_fixtures"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0031.json

---

## NOM-0032 — Deterministic testing

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Make automated tests reproducible and independent of unnecessary network or timing variability.

**Predicate:** {"action":"practice","object":"deterministic_testing","target":"authorized_project_work"}

**Normative conditions:** ["When authoring automated tests","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When authoring automated tests"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_quality","software_development","software_testing"],"topics":["deterministic_testing"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0032.json

---

## NOM-0033 — Feature and target combinations

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Test relevant feature flags, supported compilation targets and build configurations without inventing platform support.

**Predicate:** {"action":"practice","object":"feature_and_target_combinations","target":"authorized_project_work"}

**Normative conditions:** ["When a project supports multiple build variants","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When a project supports multiple build variants"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_quality","software_development","software_testing"],"topics":["target_feature_testing"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0033.json

---

## NOM-0060 — Artifact integrity hashes

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Produce integrity checksums for distributable artifacts when useful and preserve their correspondence to outputs.

**Predicate:** {"action":"practice","object":"artifact_integrity_hashes","target":"authorized_project_work"}

**Normative conditions:** ["When packaging distributables","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When packaging distributables"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["data_integrity","releases","software_development"],"topics":["artifact_hashing"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0060.json

---

## NOM-0062 — Automated smoke checks

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Verify startup and essential functionality with repeatable smoke tests where the environment supports it.

**Predicate:** {"action":"practice","object":"automated_smoke_checks","target":"authorized_project_work"}

**Normative conditions:** ["When testing a runnable application","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When testing a runnable application"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["qa_verification","software_development","software_testing"],"topics":["smoke_testing"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0062.json

---

## NOM-0064 — Acceptance criteria review

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Compare delivered functionality with the original authorized requirements and record deviations.

**Predicate:** {"action":"practice","object":"acceptance_criteria_review","target":"authorized_project_work"}

**Normative conditions:** ["Before closing a work item","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Before closing a work item"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["qa_verification","software_development"],"topics":["acceptance_criteria"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0064.json

---

## NOM-0066 — Verifiable completion reports

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Report concrete commit IDs, test results, deployment evidence and unresolved work.

**Predicate:** {"action":"practice","object":"verifiable_completion_reports","target":"authorized_project_work"}

**Normative conditions:** ["At the end of authorized work","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["At the end of authorized work"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["qa_verification","software_development"],"topics":["completion_evidence"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0066.json

---

## NOM-0223 — inception contract

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Before major implementation, record the new Rust project's intended users, essential behavior, first runnable deliverable, supported environment, and explicit non-goals; keep this contract consistent with later MVP acceptance.

**Predicate:** {"action":"verify","object":"new_rust_inception_contract","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch, and the specified behavior or decision is relevant to its approved purpose."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch, and the specified behavior or decision is relevant to its approved purpose."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0088"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0223.json

---

## NOM-0224 — architecture boundaries

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For an authorized new Rust project, choose and document executable/library boundaries, ownership of persistent state and public interfaces before those decisions become expensive to change; avoid architecture unsupported by its real requirements.

**Predicate:** {"action":"verify","object":"new_rust_architecture_boundaries","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch, and the specified behavior or decision is relevant to its approved purpose."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch, and the specified behavior or decision is relevant to its approved purpose."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0044"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0224.json

---

## NOM-0225 — rust toolchain

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Record and verify a reproducible Rust toolchain and applicable Cargo package/target metadata, including the chosen edition, required compiler compatibility and supported targets; do not imply support for untested targets.

**Predicate:** {"action":"verify","object":"new_rust_rust_toolchain","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch, and the specified behavior or decision is relevant to its approved purpose."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch, and the specified behavior or decision is relevant to its approved purpose."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0012"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0225.json

---

## NOM-0226 — dependency risk

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Before introducing a new external crate, review its necessity, license compatibility, maintenance/security posture and operational cost; record exceptions relevant to the project's authorized scope.

**Predicate:** {"action":"verify","object":"new_rust_dependency_risk","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch, and the specified behavior or decision is relevant to its approved purpose."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch, and the specified behavior or decision is relevant to its approved purpose."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0019"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0226.json

---

## NOM-0227 — recoverable errors

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Where failures can be handled or reported, propagate recoverable Rust errors through stable result boundaries with actionable user-facing diagnostics; reserve panics for genuine invariant violations rather than routine external failures.

**Predicate:** {"action":"verify","object":"new_rust_recoverable_errors","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch, and the specified behavior or decision is relevant to its approved purpose."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch, and the specified behavior or decision is relevant to its approved purpose."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0030"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0227.json

---

## NOM-0228 — configuration semantics

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When the project accepts runtime configuration, verify each exposed key's precedence, defaults, validation, rejected values and resulting failure behavior against the documented contract.

**Predicate:** {"action":"verify","object":"new_rust_configuration_semantics","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0045"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0228.json

---

## NOM-0229 — durable writes

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When the application persists valuable mutable state, verify its write/replace operations cannot silently corrupt prior valid state on interruption and that recovery of interrupted writes is supported or limitations disclosed.

**Predicate:** {"action":"verify","object":"new_rust_durable_writes","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0053"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0229.json

---

## NOM-0230 — untrusted input

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Where input crosses an external trust boundary, enforce documented parsing, size/resource limits and authorization checks before side effects, with representative invalid and adversarial cases.

**Predicate:** {"action":"verify","object":"new_rust_untrusted_input","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0030"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0230.json

---

## NOM-0231 — shutdown lifecycle

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For long-running processes or stateful CLI/GUI applications, verify required termination, cancellation, resource release and preservation of pending valuable work on normal exit and relevant interruption paths.

**Predicate:** {"action":"verify","object":"new_rust_shutdown_lifecycle","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0062"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0231.json

---

## NOM-0232 — performance acceptance

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When startup, latency, memory use, throughput or power is part of the project's accepted usefulness contract, define representative measurement conditions and check those budgets before claiming completion.

**Predicate:** {"action":"verify","object":"new_rust_performance_acceptance","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0088"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0232.json

---

## NOM-0233 — concurrency lifecycle

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When the application uses background tasks or concurrent workers, verify task ownership, cancellation, error propagation and shutdown behavior under representative races; do not require a scheduler order the contract does not promise.

**Predicate:** {"action":"verify","object":"new_rust_concurrency_lifecycle","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0032"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0233.json

---

## NOM-0234 — gui keyboard access

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For a project exposing a GUI, verify its agreed primary operations remain operable by keyboard, with visible focus, readable labels and accessible error/feedback paths on supported platforms.

**Predicate:** {"action":"verify","object":"new_rust_gui_keyboard_access","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0063"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0234.json

---

## NOM-0235 — privacy safe diagnostics

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For applications producing logs, traces or crash reports, verify useful failure context is captured without inadvertently recording credentials or private user data; document opt-in collection where applicable.

**Predicate:** {"action":"verify","object":"new_rust_privacy_safe_diagnostics","target":"rust_project_from_scratch"}

**Normative conditions:** ["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."]

**Normative exceptions:** ["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized project development and acceptance cycle","cue":"Where this contract applies","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An expressly authorized Rust project is being created from scratch and this feature, interface or operating constraint is included in its agreed requirements."],"exclusions":["Excluded or inapplicable capabilities do not require invented implementation tasks; verified deviations and untested paths must remain disclosed."],"project_types":["rust_project_from_scratch"]}

**Authority and overrides:** {"source":"Owner-authorized new Rust project coverage remediation; selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0051"]}

**Categories:** {"domains":["rust_development","software_development","software_testing"],"topics":["new_rust_project"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0235.json
