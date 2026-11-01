# Nomos / core/git.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0014 — Repository inspection

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Inspect the existing source tree, configuration, branch, and established conventions before modifying a repository.

**Predicate:** {"action":"practice","object":"repository_inspection","target":"authorized_project_work"}

**Normative conditions:** ["Before authorized repository work","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Before authorized repository work"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["git_repository","software_development"],"topics":["repository_inspection"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0014.json

---

## NOM-0015 — Main branch default

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Use the existing main branch by default; deviate only when specifically instructed for the repository or task.

**Predicate:** {"action":"practice","object":"main_branch_default","target":"authorized_project_work"}

**Normative conditions:** ["When choosing a Git work branch","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When choosing a Git work branch"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["git_repository","software_development"],"topics":["branch_policy"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0015.json

---

## NOM-0016 — Change scope containment

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Keep changes within the authorized task and repository; avoid unrelated edits or surprise features.

**Predicate:** {"action":"practice","object":"change_scope_containment","target":"authorized_project_work"}

**Normative conditions:** ["During any repository modification","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["During any repository modification"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["git_repository","software_development"],"topics":["change_scope"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0016.json

---

## NOM-0017 — Coherent descriptive commits

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Record authorized changes in logical commits with concise, informative commit messages.

**Predicate:** {"action":"practice","object":"coherent_descriptive_commits","target":"authorized_project_work"}

**Normative conditions:** ["When committing authorized work","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When committing authorized work"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["git_repository","software_development"],"topics":["commit_quality"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0017.json

---

## NOM-0018 — Maintain ignore rules

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Exclude generated outputs, temporary artifacts, machine-specific files and secrets through appropriate ignore rules.

**Predicate:** {"action":"practice","object":"maintain_ignore_rules","target":"authorized_project_work"}

**Normative conditions:** ["When maintaining repository tracking","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When maintaining repository tracking"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["git_repository","software_development"],"topics":["ignore_file_maintenance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0018.json

---

## NOM-0019 — Respect project licensing

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Preserve existing licenses; add a suitable license at an explicitly authorized bootstrap when needed.

**Predicate:** {"action":"practice","object":"respect_project_licensing","target":"authorized_project_work"}

**Normative conditions:** ["When bootstrapping or changing licensing","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When bootstrapping or changing licensing"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["git_repository","software_development"],"topics":["license_compliance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0019.json

---

## NOM-0020 — Dependency lockfile consistency

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Keep tracked dependency lockfiles synchronized with manifest changes, using the conventions for that application or library.

**Predicate:** {"action":"practice","object":"dependency_lockfile_consistency","target":"authorized_project_work"}

**Normative conditions:** ["When dependency configuration changes","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When dependency configuration changes"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["dependency_management","git_repository","software_development"],"topics":["lockfile_integrity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0020.json

---

## NOM-0021 — Remove obsolete scaffolding safely

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Remove obsolete templates, dead files and stale configuration only after checking for active dependencies and preserving unrelated work.

**Predicate:** {"action":"practice","object":"remove_obsolete_scaffolding_safely","target":"authorized_project_work"}

**Normative conditions:** ["When cleanup is in scope","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When cleanup is in scope"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["git_repository","maintenance","software_development"],"topics":["scaffold_cleanup"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0021.json

---

## NOM-0022 — Verify Git branch and history

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Verify branch identity, expected ancestry and the exact changes before updating repository references.

**Predicate:** {"action":"practice","object":"verify_git_branch_and_history","target":"authorized_project_work"}

**Normative conditions:** ["Before commits or reference updates","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Before commits or reference updates"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["git_repository","software_development"],"topics":["git_history_verification"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0022.json

---

## NOM-0023 — Verify committed state

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Confirm successful commits and the intended branch HEAD before reporting work as committed.

**Predicate:** {"action":"practice","object":"verify_committed_state","target":"authorized_project_work"}

**Normative conditions:** ["After repository mutations","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["After repository mutations"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["git_repository","software_development"],"topics":["committed_state_verification"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0023.json

---

## NOM-0084 — Pull latest changes before local repository YAML job work

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Before performing work on an existing local Git repository through a YAML-specified execution job, run git pull in that verified repository to retrieve the latest remote changes rather than operating on an assumed-current checkout.

**Predicate:** {"action":"run","object":"git_pull","target":"local_repository_preflight"}

**Normative conditions:** ["Job targets an existing explicitly authorized local repository."]

**Normative exceptions:** ["Does not authorize changing unrelated or substitute repositories."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Job targets an existing explicitly authorized local repository."],"exclusions":["Does not authorize changing unrelated or substitute repositories."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0014","NOM-0015"]}

**Categories:** {"domains":["agent_instructions","software_development","workflow_governance"],"topics":["repository_sync","yaml_job_workflow"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0084.json

---

## NOM-0129 — Record the authoritative upstream source and synchronization baseline

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For an authorized Rust fork maintenance task, identify the intended upstream repository and remote, the branch or release being followed, and the last verified upstream commit incorporated into the fork. Do not infer upstream identity from similarly named remotes or unrelated repositories.

**Predicate:** {"action":"record","object":"upstream_identity_and_base_revision","target":"rust_fork_maintenance_preflight"}

**Normative conditions:** ["During an authorized Rust fork baseline review."]

**Normative exceptions:** ["Recording upstream refs does not permit merges or other repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["rust_fork_maintenance"],"preconditions":["During an authorized Rust fork baseline review."],"exclusions":["Recording upstream refs does not permit merges or other repository operations."]}

**Authority and overrides:** {"source":"Explicitly reviewed public-safe specimen normalization; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0014","NOM-0022"]}

**Categories:** {"domains":["git_repository","maintenance"],"topics":["rust_fork_upstream_provenance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0129.json

---

## NOM-0130 — Inspect upstream divergence before selecting incoming changes

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Before an authorized synchronization of a Rust fork, fetch the intended upstream references and compare the fork's actual upstream ancestry and incoming commits or tags. Review the change scope and proposed integration strategy before modifying maintained branches.

**Predicate:** {"action":"inspect","object":"upstream_divergence_and_integration_plan","target":"rust_fork_sync_operation"}

**Normative conditions:** ["Before changes from a verified upstream are integrated."]

**Normative exceptions:** ["Fetch and compare first; merge/rebase/cherry-pick or releases require separately authorized scope."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["rust_fork_maintenance"],"preconditions":["Before changes from a verified upstream are integrated."],"exclusions":["Fetch and compare first; merge/rebase/cherry-pick or releases require separately authorized scope."]}

**Authority and overrides:** {"source":"Explicitly reviewed public-safe specimen normalization; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0129"]}

**Categories:** {"domains":["git_repository","maintenance"],"topics":["rust_fork_upstream_change_discovery"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0130.json

---

## NOM-0131 — Maintain provenance and disposition for fork-specific patches

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Maintain a reviewable ledger linking independently maintained fork patches to their originating change, purpose, upstream issue or commit when known, and current status such as pending, upstreamed, superseded or deliberately retained; update it when patches are changed, rebased or retired.

**Predicate:** {"action":"maintain","object":"fork_patch_lineage_and_dispositions","target":"rust_fork_patch_ledger"}

**Normative conditions:** ["When a Rust fork retains independently maintained patches."]

**Normative exceptions:** ["The reviewable ledger can live in files, commit notes or issue metadata; no single file format is mandatory."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["rust_fork_maintenance"],"preconditions":["When a Rust fork retains independently maintained patches."],"exclusions":["The reviewable ledger can live in files, commit notes or issue metadata; no single file format is mandatory."]}

**Authority and overrides:** {"source":"Explicitly reviewed public-safe specimen normalization; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0129"]}

**Categories:** {"domains":["git_repository","maintenance"],"topics":["rust_fork_patch_provenance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0131.json

---

## NOM-0132 — Resolve upstream integration conflicts without silently losing fork changes

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When an authorized fork integration produces competing edits, identify the affected fork-specific patches and upstream changes, choose an explicit resolution supported by the approved task, and verify that intended behavior is preserved; never treat an automatic merge or rebase result as authorization to discard a local customization.

**Predicate:** {"action":"reconcile","object":"upstream_and_fork_patch_conflicts","target":"authorized_rust_fork_integration"}

**Normative conditions:** ["When authorized upstream integration affects local behavior or patches."]

**Normative exceptions:** ["Behavioral changes and unresolved conflicts must be surfaced; do not invent authorization to discard customizations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["rust_fork_maintenance"],"preconditions":["When authorized upstream integration affects local behavior or patches."],"exclusions":["Behavioral changes and unresolved conflicts must be surfaced; do not invent authorization to discard customizations."]}

**Authority and overrides:** {"source":"Explicitly reviewed public-safe specimen normalization; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0130","NOM-0131"]}

**Categories:** {"domains":["git_repository","software_development"],"topics":["rust_fork_conflict_reconciliation"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0132.json

---

## NOM-0134 — Select upstream fixes and features within authorized fork maintenance scope

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Classify upstream changes as security or correctness fixes, compatibility changes, maintenance improvements or optional features, and incorporate only those permitted by the fork's approved scope and supported versions. Do not equate 'new upstream version available' with blanket authorization to import all upstream behavior or release it.

**Predicate:** {"action":"select","object":"authorized_upstream_change_set","target":"rust_fork_maintenance_scope"}

**Normative conditions:** ["When deciding which upstream changes to include in a maintained fork."]

**Normative exceptions:** ["The availability of an upstream release does not authorize importing everything; compatibility exceptions require approval."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["rust_fork_maintenance"],"preconditions":["When deciding which upstream changes to include in a maintained fork."],"exclusions":["The availability of an upstream release does not authorize importing everything; compatibility exceptions require approval."]}

**Authority and overrides:** {"source":"Explicitly reviewed public-safe specimen normalization; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0130"]}

**Categories:** {"domains":["maintenance","git_repository"],"topics":["rust_fork_selective_upstream_adoption"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0134.json
