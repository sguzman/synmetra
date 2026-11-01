# Nomos / core/workflow.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0004 — Do not expose private generation context in public outputs

**Version:** 1.1.1 · **Kind:** prohibition · **Authority:** general_guidance

Do not reproduce or reveal private conversations, private drafting language, or nonpublic internal instructions in public outputs merely because they informed the work; public release of the result does not by itself authorize release of its private inputs.

**Predicate:** {"action":"include","object":"private_generation_context","target":"artifact"}

**Normative conditions:** ["Separate authorization to disclose the private generation context has not been given"]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":[],"project_types":["any"]}

**Authority and overrides:** {"source":"Published in Nomos by explicit batch authorization; general guidance, never automatically applied","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["agent_instructions","privacy","publication"],"topics":["content_handling","information_disclosure","personal_expression","private_public_boundary"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0004.json

---

## NOM-0011 — Keep specimen intake, processing, derived task queue and execution distinct

**Version:** 1.1.1 · **Kind:** obligation · **Authority:** general_guidance

Keep specimen intake, authorized specimen analysis, any resulting downstream task queue, and execution of those tasks as independent, explicitly authorized workflows with distinct states and traceable transitions.

**Predicate:** {"action":"separate","object":"workflow_stages","target":"specimen_and_task_system"}

**Normative conditions:** ["Derived tasks do not exist automatically merely because analysis finishes."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":[],"project_types":["any"]}

**Authority and overrides:** {"source":"Published in Nomos by explicit batch authorization; general guidance, never automatically applied","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["governance","workflow_governance"],"topics":["specimen_task_workflow","workflow_stage"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0011.json

---

## NOM-0077 — Do not suggest names ending with the suffix arium

**Version:** 1.0.1 · **Kind:** prohibition · **Authority:** general_guidance

When generating or recommending new names for projects, products, applications, repositories, tools, or other user-named entities, do not propose candidate names whose final letters are 'arium', regardless of letter case.

**Predicate:** {"action":"suggest","object":"new_name_ending_in_arium_case_insensitive","target":"user_facing_name_proposal"}

**Normative conditions:** ["Proposing a new name; established names may still be mentioned."]

**Normative exceptions:** ["Does not prevent discussing historical or existing names."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":null,"cue":"Do not","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Proposing a new name; established names may still be mentioned."],"exclusions":["Does not prevent discussing historical or existing names."],"project_types":["any"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["agent_instructions","software_development"],"topics":["naming_restrictions"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0077.json

---

## NOM-0078 — Ask for the actual local repository path before issuing YAML execution jobs

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Before preparing or executing a YAML-specified job that operates on an existing local Git repository, request and use the user's actual local repository filesystem path instead of assuming its location.

**Predicate:** {"action":"request","object":"confirmed_local_repository_path","target":"local_repository_yaml_job"}

**Normative conditions:** ["Job targets an existing explicitly authorized local repository."]

**Normative exceptions:** ["Does not authorize changing unrelated or substitute repositories."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Job targets an existing explicitly authorized local repository."],"exclusions":["Does not authorize changing unrelated or substitute repositories."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0014"]}

**Categories:** {"domains":["agent_instructions","software_development","workflow_governance"],"topics":["local_job_path","yaml_job_workflow"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0078.json

---

## NOM-0079 — Do not invent local repository paths

**Version:** 1.1.0 · **Kind:** prohibition · **Authority:** general_guidance

Do not fabricate, infer, or substitute an unverified filesystem path for a project when writing or running a YAML-specified job against a local Git repository.

**Predicate:** {"action":"invent","object":"unverified_repository_path","target":"local_repository_yaml_job"}

**Normative conditions:** ["Job targets an existing explicitly authorized local repository."]

**Normative exceptions:** ["Does not authorize changing unrelated or substitute repositories."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":null,"cue":"Do not","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Job targets an existing explicitly authorized local repository."],"exclusions":["Does not authorize changing unrelated or substitute repositories."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0014"]}

**Categories:** {"domains":["agent_instructions","software_development","workflow_governance"],"topics":["repository_path_integrity","yaml_job_workflow"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0079.json

---

## NOM-0080 — Do not create substitute repositories or temporary project clones

**Version:** 1.1.0 · **Kind:** prohibition · **Authority:** general_guidance

When a YAML-specified job is intended to operate on the user's existing local repository, do not create a replacement project, repository checkout, or temporary clone as a substitute for that repository.

**Predicate:** {"action":"create","object":"substitute_repository_or_temporary_checkout","target":"local_repository_yaml_job"}

**Normative conditions:** ["Job targets an existing explicitly authorized local repository."]

**Normative exceptions:** ["Does not authorize changing unrelated or substitute repositories."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":null,"cue":"Do not","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Job targets an existing explicitly authorized local repository."],"exclusions":["Does not authorize changing unrelated or substitute repositories."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0014"]}

**Categories:** {"domains":["agent_instructions","software_development","workflow_governance"],"topics":["repository_clone_policy","yaml_job_workflow"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0080.json

---

## NOM-0081 — Do not embed large Python programs in YAML execution jobs

**Version:** 1.1.0 · **Kind:** prohibition · **Authority:** general_guidance

Do not put substantial multi-line Python program implementations directly inside YAML job commands; keep executable program logic in separate source files.

**Predicate:** {"action":"embed","object":"large_inline_python_program","target":"yaml_job_commands"}

**Normative conditions:** ["Job targets an existing explicitly authorized local repository."]

**Normative exceptions:** ["Does not authorize changing unrelated or substitute repositories."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":null,"cue":"Do not","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Job targets an existing explicitly authorized local repository."],"exclusions":["Does not authorize changing unrelated or substitute repositories."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0014"]}

**Categories:** {"domains":["agent_instructions","software_development","workflow_governance"],"topics":["yaml_job_workflow","yaml_script_size"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0081.json

---

## NOM-0082 — Place nontrivial job scripts in the repository scripts directory

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

When an automation job requires a substantial standalone script, create or maintain that script as a file under the verified local project's scripts/ directory rather than implementing it inside the job YAML.

**Predicate:** {"action":"store","object":"standalone_job_script","target":"repository_scripts_directory"}

**Normative conditions:** ["Job targets an existing explicitly authorized local repository."]

**Normative exceptions:** ["Does not authorize changing unrelated or substitute repositories."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Job targets an existing explicitly authorized local repository."],"exclusions":["Does not authorize changing unrelated or substitute repositories."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["agent_instructions","software_development","workflow_governance"],"topics":["job_script_placement","yaml_job_workflow"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0082.json

---

## NOM-0083 — Invoke external scripts from YAML jobs

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

When a YAML execution job uses a standalone script, keep the YAML focused on invoking the script file from its confirmed project path instead of embedding the script body in YAML commands.

**Predicate:** {"action":"invoke","object":"repository_standalone_script","target":"yaml_job_commands"}

**Normative conditions:** ["Job targets an existing explicitly authorized local repository."]

**Normative exceptions:** ["Does not authorize changing unrelated or substitute repositories."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Job targets an existing explicitly authorized local repository."],"exclusions":["Does not authorize changing unrelated or substitute repositories."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["agent_instructions","software_development","workflow_governance"],"topics":["job_script_invocation","yaml_job_workflow"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0083.json

---

## NOM-0085 — Increase Tampermonkey @version on every userscript change

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Every time a Tampermonkey userscript is changed, including during local iteration, increment its UserScript metadata @version to a strictly higher value. This keeps script revisions distinguishable for update detection; do not wait for a formal release or distribution.

**Predicate:** {"action":"increment","object":"tampermonkey_metadata_at_version","target":"modified_userscript"}

**Normative conditions:** ["Every authorized script or extension modification, including local changes."]

**Normative exceptions:** ["Metadata changes do not independently authorize distribution."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"occurrences","window":"each modification","cue":"Every time","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Every authorized script or extension modification, including local changes."],"exclusions":["Metadata changes do not independently authorize distribution."],"project_types":["browser_userscript"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0055"],"depends_on":["NOM-0056"]}

**Categories:** {"domains":["agent_instructions","software_development"],"topics":["browser_artifact_versioning","browser_userscript_version"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0085.json

---

## NOM-0086 — Increase WebExtension manifest version on every extension change

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Every time a browser extension is changed, including during local iteration, increase its manifest.json version to a strictly higher valid extension version. This applies to code, resources and metadata revisions; do not confuse the extension version with manifest_version.

**Predicate:** {"action":"increment","object":"webextension_manifest_version_field","target":"modified_extension"}

**Normative conditions:** ["Every authorized script or extension modification, including local changes."]

**Normative exceptions:** ["Metadata changes do not independently authorize distribution."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"occurrences","window":"each modification","cue":"Every time","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Every authorized script or extension modification, including local changes."],"exclusions":["Metadata changes do not independently authorize distribution."],"project_types":["browser_extension"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0055"],"depends_on":["NOM-0056"]}

**Categories:** {"domains":["agent_instructions","software_development"],"topics":["browser_artifact_versioning","browser_extension_version"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0086.json

---

## NOM-0087 — Define the MVP's essential purpose

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Define the project's essential real-user purpose and minimal useful externally observable outcome before deciding whether an MVP is ready.

**Predicate:** {"action":"define","object":"define_the_mvp_s_essential_purpose","target":"project_mvp_lifecycle"}

**Normative conditions:** ["Only during an authorized MVP readiness, graduation, release, or maintenance review."]

**Normative exceptions:** ["No automatic release, tag, deploy, or other project mutation."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Only during an authorized MVP readiness, graduation, release, or maintenance review."],"exclusions":["No automatic release, tag, deploy, or other project mutation."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0064"]}

**Categories:** {"domains":["project_management","releases","software_development"],"topics":["mvp_definition","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0087.json

---

## NOM-0088 — Set testable MVP acceptance criteria and scope

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Before MVP graduation, specify required capabilities, measurable acceptance criteria, target environment, and excluded future features.

**Predicate:** {"action":"set","object":"set_testable_mvp_acceptance_criteria_and_scope","target":"project_mvp_lifecycle"}

**Normative conditions:** ["Only during an authorized MVP readiness, graduation, release, or maintenance review."]

**Normative exceptions:** ["No automatic release, tag, deploy, or other project mutation."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Only during an authorized MVP readiness, graduation, release, or maintenance review."],"exclusions":["No automatic release, tag, deploy, or other project mutation."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0064","NOM-0075"]}

**Categories:** {"domains":["project_management","releases","software_development"],"topics":["mvp_acceptance","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0088.json

---

## NOM-0089 — Verify essential workflows end to end

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Exercise the MVP's essential workflows end to end in the actual running implementation before inaugurating it; compilation alone cannot establish readiness.

**Predicate:** {"action":"verify","object":"verify_essential_workflows_end_to_end","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0062","NOM-0064"]}

**Categories:** {"domains":["project_management","releases","software_development","software_testing"],"topics":["mvp_end_to_end_testing","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0089.json

---

## NOM-0090 — Pass applicable automated tests and CI gates

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Run and pass the relevant unit, integration, regression, formatting, lint, compilation, and configured CI checks before inauguration; disclose untested targets.

**Predicate:** {"action":"pass","object":"pass_applicable_automated_tests_and_ci_gates","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0027","NOM-0028","NOM-0029","NOM-0036","NOM-0041"]}

**Categories:** {"domains":["project_management","releases","software_development","software_testing"],"topics":["mvp_ci_gate","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0090.json

---

## NOM-0091 — Verify the MVP in the intended actual environment

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Test essential behavior on the intended desktop, browser, CLI, service, device or other real target environment, including necessary integration boundaries.

**Predicate:** {"action":"verify","object":"verify_the_mvp_in_the_intended_actual_environment","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0063"],"depends_on":["NOM-0064","NOM-0065"]}

**Categories:** {"domains":["project_management","releases","software_development","software_testing"],"topics":["mvp_environment_verification","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0091.json

---

## NOM-0092 — Verify the user installation or operating path

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Demonstrate the real documented path to obtain, build, launch, install or consume the product before MVP graduation, according to project type.

**Predicate:** {"action":"verify","object":"verify_the_user_installation_or_operating_path","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0057","NOM-0059","NOM-0062"]}

**Categories:** {"domains":["project_management","releases","software_development"],"topics":["mvp_inauguration","mvp_installation"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0092.json

---

## NOM-0093 — Test a public library API with an independent consumer

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

When a project's MVP includes a reusable library or agent-facing API, confirm it works with an independent consuming application or integration before inaugurating that interface.

**Predicate:** {"action":"test","object":"test_a_public_library_api_with_an_independent_consumer","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0028","NOM-0054","NOM-0064"]}

**Categories:** {"domains":["project_management","releases","software_development","software_testing"],"topics":["mvp_api_consumer","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0093.json

---

## NOM-0094 — Do not inaugurate despite purpose-defeating defects

**Version:** 1.1.0 · **Kind:** prohibition · **Authority:** general_guidance

Do not declare the MVP inaugurated while known defects, regressions or failed acceptance checks prevent its agreed essential purpose; accepted minor limitations must be disclosed.

**Predicate:** {"action":"avoid","object":"do_not_inaugurate_despite_purpose_defeating_defects","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":null,"cue":"Do not","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0064","NOM-0067"]}

**Categories:** {"domains":["project_management","releases","software_development","software_testing"],"topics":["mvp_defect_gate","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0094.json

---

## NOM-0095 — Get explicit project-owner acceptance for inauguration

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Before changing the MVP status or publishing its inaugural release, obtain explicit owner approval of the verified acceptance evidence and acknowledged limitations.

**Predicate:** {"action":"get","object":"get_explicit_project_owner_acceptance_for_inauguration","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0064","NOM-0065"]}

**Categories:** {"domains":["governance","project_management","releases","software_development"],"topics":["mvp_inauguration","mvp_owner_approval"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0095.json

---

## NOM-0096 — Do not declare MVP solely from proxy progress indicators

**Version:** 1.1.0 · **Kind:** prohibition · **Authority:** general_guidance

Do not call an MVP inaugurated solely because CI is green, compilation succeeds, tasks are 100 percent, or milestones are marked complete without operational acceptance evidence.

**Predicate:** {"action":"avoid","object":"do_not_declare_mvp_solely_from_proxy_progress_indicators","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":null,"cue":"Do not","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0065","NOM-0066","NOM-0067"]}

**Categories:** {"domains":["project_management","releases","software_development"],"topics":["mvp_evidence_standard","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0096.json

---

## NOM-0097 — Record the graduation into a maintained baseline

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

At MVP inauguration, record the transition from exploratory implementation to a maintained, versioned baseline for the product's core capabilities.

**Predicate:** {"action":"record","object":"record_the_graduation_into_a_maintained_baseline","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0054","NOM-0065"]}

**Categories:** {"domains":["maintenance","project_management","releases","software_development"],"topics":["mvp_baseline","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0097.json

---

## NOM-0098 — Record MVP compatibility promises and accepted limitations

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Document the inaugural supported features, public interfaces, data formats, environments, operational requirements and accepted limitations as a maintainable contract.

**Predicate:** {"action":"record","object":"record_mvp_compatibility_promises_and_accepted_limitations","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0044","NOM-0054","NOM-0064"]}

**Categories:** {"domains":["maintenance","project_management","releases","software_development"],"topics":["mvp_compatibility","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0098.json

---

## NOM-0099 — Keep future improvements outside MVP acceptance requirements

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Separate optional future enhancements from essential MVP gates; completing the minimum useful product need not exhaust the roadmap, but required features cannot be silently deferred.

**Predicate:** {"action":"keep","object":"keep_future_improvements_outside_mvp_acceptance_requirements","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0076","NOM-0067"]}

**Categories:** {"domains":["project_management","releases","software_development"],"topics":["mvp_inauguration","mvp_scope_boundary"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0099.json

---

## NOM-0100 — Reconcile README, roadmap and milestones at graduation

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Before the inaugural release, make README, roadmap and milestone reports accurately reflect verified MVP features and unresolved work.

**Predicate:** {"action":"reconcile","object":"reconcile_readme_roadmap_and_milestones_at_graduation","target":"project_mvp_lifecycle"}

**Normative conditions:** ["An authorized MVP readiness, graduation, release, or follow-up review is in progress."]

**Normative exceptions:** ["Does not independently authorize a tag, release or external deployment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An authorized MVP readiness, graduation, release, or follow-up review is in progress."],"exclusions":["Does not independently authorize a tag, release or external deployment."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; requires opt-in selection","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0042"],"depends_on":["NOM-0065","NOM-0066","NOM-0067"]}

**Categories:** {"domains":["documentation","project_management","releases","software_development"],"topics":["mvp_documentation","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0100.json

---

## NOM-0101 — Establish an approved stable inaugural version

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

For a SemVer-based MVP inauguration, choose the accepted initial maintained release version, normally v1.0.0 unless the project has an explicitly different established contract.

**Predicate:** {"action":"establish","object":"establish_an_approved_stable_inaugural_version","target":"project_mvp_lifecycle"}

**Normative conditions:** ["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."]

**Normative exceptions:** ["No implicit authorization to publish tags, releases or deployments."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."],"exclusions":["No implicit authorization to publish tags, releases or deployments."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0055","NOM-0056"]}

**Categories:** {"domains":["project_management","releases","software_development"],"topics":["inaugural_version","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0101.json

---

## NOM-0102 — Tag and release the exact verified and accepted commit

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

After explicit release authorization, create and verify the inaugural version tag and GitHub release for the exact tested and accepted commit.

**Predicate:** {"action":"tag","object":"tag_and_release_the_exact_verified_and_accepted_commit","target":"project_mvp_lifecycle"}

**Normative conditions:** ["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."]

**Normative exceptions:** ["No implicit authorization to publish tags, releases or deployments."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."],"exclusions":["No implicit authorization to publish tags, releases or deployments."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0057","NOM-0058"]}

**Categories:** {"domains":["project_management","releases","software_development"],"topics":["inaugural_release","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0102.json

---

## NOM-0103 — Provide relevant inaugural downloads through free channels

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

For distributable applications or libraries, provide appropriate downloadable release assets through authorized zero-cost hosting where feasible; do not require binaries for source-only products.

**Predicate:** {"action":"provide","object":"provide_relevant_inaugural_downloads_through_free_channels","target":"project_mvp_lifecycle"}

**Normative conditions:** ["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."]

**Normative exceptions:** ["No implicit authorization to publish tags, releases or deployments."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."],"exclusions":["No implicit authorization to publish tags, releases or deployments."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0059","NOM-0061"]}

**Categories:** {"domains":["project_management","releases","software_development"],"topics":["inaugural_distribution","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0103.json

---

## NOM-0104 — Verify the published inaugural release

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Confirm the published tag, version, CI result, release metadata, assets and documented usage path actually correspond to the accepted candidate.

**Predicate:** {"action":"verify","object":"verify_the_published_inaugural_release","target":"project_mvp_lifecycle"}

**Normative conditions:** ["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."]

**Normative exceptions:** ["No implicit authorization to publish tags, releases or deployments."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."],"exclusions":["No implicit authorization to publish tags, releases or deployments."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0041","NOM-0057","NOM-0066"]}

**Categories:** {"domains":["project_management","qa_verification","releases","software_development"],"topics":["inaugural_release_verification","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0104.json

---

## NOM-0105 — Record durable evidence for MVP acceptance

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

Preserve the acceptance criteria, test results, exact commit, limitations, owner approval and release references without exposing private discussions.

**Predicate:** {"action":"record","object":"record_durable_evidence_for_mvp_acceptance","target":"project_mvp_lifecycle"}

**Normative conditions:** ["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."]

**Normative exceptions:** ["No implicit authorization to publish tags, releases or deployments."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."],"exclusions":["No implicit authorization to publish tags, releases or deployments."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0065","NOM-0066"]}

**Categories:** {"domains":["project_management","qa_verification","releases","software_development"],"topics":["mvp_evidence_record","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0105.json

---

## NOM-0106 — Use standardized post-MVP commits and releases

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

For projects adopting maintained post-MVP governance, use agreed standardized commits, normally Conventional Commits, and consistent changelog/release discipline without rewriting history.

**Predicate:** {"action":"use","object":"use_standardized_post_mvp_commits_and_releases","target":"project_mvp_lifecycle"}

**Normative conditions:** ["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."]

**Normative exceptions:** ["No implicit authorization to publish tags, releases or deployments."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."],"exclusions":["No implicit authorization to publish tags, releases or deployments."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0017"],"depends_on":["NOM-0047","NOM-0055"]}

**Categories:** {"domains":["maintenance","project_management","releases","software_development"],"topics":["mvp_inauguration","post_mvp_releases"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0106.json

---

## NOM-0107 — Manage changes after MVP with the compatibility contract

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

After inauguration, classify bug fixes, additive changes and incompatible changes against supported behavior and version them under the project's approved SemVer policy.

**Predicate:** {"action":"manage","object":"manage_changes_after_mvp_with_the_compatibility_contract","target":"project_mvp_lifecycle"}

**Normative conditions:** ["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."]

**Normative exceptions:** ["No implicit authorization to publish tags, releases or deployments."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."],"exclusions":["No implicit authorization to publish tags, releases or deployments."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0054","NOM-0055","NOM-0056"]}

**Categories:** {"domains":["maintenance","project_management","releases","software_development"],"topics":["mvp_inauguration","post_mvp_compatibility"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0107.json

---

## NOM-0108 — Do not authorize unrelated operational work through inauguration

**Version:** 1.1.0 · **Kind:** prohibition · **Authority:** general_guidance

Do not interpret MVP inauguration as permission to alter unrelated repositories, system services, startup behavior or deployments outside the approved scope.

**Predicate:** {"action":"avoid","object":"do_not_authorize_unrelated_operational_work_through_inauguration","target":"project_mvp_lifecycle"}

**Normative conditions:** ["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."]

**Normative exceptions:** ["No implicit authorization to publish tags, releases or deployments."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":null,"cue":"Do not","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."],"exclusions":["No implicit authorization to publish tags, releases or deployments."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0052"]}

**Categories:** {"domains":["governance","project_management","releases","software_development"],"topics":["mvp_authorization_boundary","mvp_inauguration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0108.json

---

## NOM-0109 — Reevaluate inauguration if core acceptance was actually invalid

**Version:** 1.1.0 · **Kind:** obligation · **Authority:** general_guidance

If foundational failures invalidate a prior MVP declaration, explicitly correct or reopen the readiness claim, verify repairs, and preserve release history rather than silently claiming success.

**Predicate:** {"action":"reevaluate","object":"reevaluate_inauguration_if_core_acceptance_was_actually_invalid","target":"project_mvp_lifecycle"}

**Normative conditions:** ["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."]

**Normative exceptions:** ["No implicit authorization to publish tags, releases or deployments."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A user-authorized MVP review, inauguration or post-MVP maintenance operation is taking place."],"exclusions":["No implicit authorization to publish tags, releases or deployments."],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0065","NOM-0067"]}

**Categories:** {"domains":["project_management","qa_verification","releases","software_development"],"topics":["mvp_inauguration","mvp_invalid_acceptance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0109.json

---

## NOM-0110 — Do not add explicit exit 1 guards to AND-linked YAML execution jobs

**Version:** 1.1.0 · **Kind:** prohibition · **Authority:** general_guidance

When authoring YAML-specification execution jobs whose runner already AND-links commands and propagates failures, do not insert redundant explicit `exit 1` statements or shell error-exit guards in the YAML commands. Rely on the job's existing command-failure behavior instead.

**Predicate:** {"action":"insert","object":"redundant_explicit_exit_1_guard","target":"and_linked_yaml_job"}

**Normative conditions:** ["The YAML runner already AND-links commands and propagates errors."]

**Normative exceptions:** ["Does not permit inventing substitute repositories or unrelated changes."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":null,"cue":"Do not","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["The YAML runner already AND-links commands and propagates errors."],"exclusions":["Does not permit inventing substitute repositories or unrelated changes."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos guidance; explicitly selected use only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["agent_instructions","software_development","workflow_governance"],"topics":["yaml_error_propagation","yaml_job_workflow"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0110.json

---

## NOM-0114 — Do not initiate or repeat unwarranted formulaic suicide-risk questions

**Version:** 1.0.1 · **Kind:** prohibition · **Authority:** general_guidance

In chat, do not initiate or repeat formulaic suicide-risk questions solely because someone expresses ordinary stress, frustration, figurative distress, or past difficulties, particularly after an explicit request to stop such questioning. When credible new information indicates a material current risk of self-harm, address that risk proportionately rather than mechanically repeating a stock question.

**Predicate:** {"action":"initiate_or_repeat","object":"unwarranted_formulaic_suicide_risk_question","target":"chat_response"}

**Normative conditions:** ["A conversational response concerns ordinary frustration, stress, figurative distress, or historical difficulty without credible new information indicating a material current self-harm risk.","A request not to repeat a generic risk-screening question reinforces the restriction unless materially new risk indications emerge."]

**Normative exceptions:** ["Credible, materially new signs of current self-harm risk warrant a proportionate safety response, including direct clarification when genuinely necessary; this rule does not prohibit warranted safety assessment."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":"each applicable chat response","cue":"Do not","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["chat"],"visibility":["any"],"artifact_kind":["any"],"actor":["ai_agent"]},"preconditions":["A conversational response concerns ordinary frustration, stress, figurative distress, or historical difficulty without credible new information indicating a material current self-harm risk.","A request not to repeat a generic risk-screening question reinforces the restriction unless materially new risk indications emerge."],"exclusions":["Credible, materially new signs of current self-harm risk warrant a proportionate safety response, including direct clarification when genuinely necessary; this rule does not prohibit warranted safety assessment."],"project_types":["any"]}

**Authority and overrides:** {"source":"Reviewed Nomos rule guidance; opt-in and subject to context-specific safety requirements","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["agent_instructions","safety_integrity"],"topics":["unwarranted_safety_checkin_repetition"],"audiences":["ai_agents","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0114.json

---

## NOM-0115 — Independent repositories for Tarium projects

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Maintain each Tarium candidate or enrolled project's implementation, ontology, gallery, datasets and research resources in its own repository; the central Tarium hub hosts only its authorized registry, governance, validation tooling and reusable templates.

**Predicate:** {"action":"house","object":"member_project_resources","target":"independent_project_repository"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0016"],"depends_on":[]}

**Categories:** {"domains":["governance","agent_instructions"],"topics":["tarium_autonomy"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0115.json

---

## NOM-0116 — No Tarium enrollment without project-specific owner authorization

**Version:** 1.0.0 · **Kind:** prohibition · **Authority:** general_guidance

Do not enroll, remove or present a repository as an official Tarium member unless the owner explicitly authorized that specific membership operation on the named public repository. Project creation, naming, or public visibility does not grant enrollment.

**Predicate:** {"action":"change_without_authorization","object":"tarium_membership","target":"central_tarium_registry"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":"each applicable Tarium project operation","cue":"Do not","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["governance","agent_instructions"],"topics":["tarium_membership"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0116.json

---

## NOM-0117 — Never expose private Taria resources through Tarium projects

**Version:** 1.0.0 · **Kind:** prohibition · **Authority:** general_guidance

Do not publish, link, import or require private Taria resources, conversations, histories, unpublished research, internal paths or nonpublic dependencies in a public Tarium project without separately authorized publication of appropriate material.

**Predicate:** {"action":"expose","object":"private_taria_resources","target":"public_tarium_project"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["Explicitly reviewed and separately authorized publication of specific public-safe replacements remains permitted; never assume authorization from a project task."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":"each applicable Tarium project operation","cue":"Do not","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0001","NOM-0003","NOM-0004"],"depends_on":[]}

**Categories:** {"domains":["privacy","agent_instructions"],"topics":["tarium_public_boundary"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0117.json

---

## NOM-0118 — Maintain an independent public Tarium project constitution

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Provide a project-local constitution that preserves Tarium's public-information restrictions, explicit membership authority, independent repository boundary and public-dependency requirements without weakening these shared guarantees.

**Predicate:** {"action":"maintain","object":"project_constitution","target":"tarium_project_repository"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["governance","agent_instructions"],"topics":["tarium_governance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0118.json

---

## NOM-0119 — Define a complete public Tarium domain ontology

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Maintain a project-owned, self-contained ontology documenting domain entities, attributes, identifiers, relationships, semantics, sources and versioning, with mutually consistent human-readable and machine-readable descriptions.

**Predicate:** {"action":"maintain","object":"complete_domain_ontology","target":"tarium_project_repository"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0006"],"depends_on":[]}

**Categories:** {"domains":["ontology","agent_instructions"],"topics":["tarium_ontology"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0119.json

---

## NOM-0120 — Maintain human and machine gallery representations

**Version:** 1.0.1 · **Kind:** obligation · **Authority:** general_guidance

Maintain an independently navigable, human-facing gallery or showcase of the project's public specimens or results and a corresponding machine-readable index. Represent an empty gallery accurately without inventing exhibits.

**Predicate:** {"action":"maintain","object":"dual_representation_gallery","target":"tarium_project_repository"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["ontology","agent_instructions"],"topics":["tarium_gallery"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0120.json

---

## NOM-0121 — Keep Tarium projects independently usable from public inputs

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Ensure a Tarium project's core operation and documented setup can use its own public assets or expressly documented publicly accessible dependencies, without requiring private Taria workspaces, secret services or unpublished resources.

**Predicate:** {"action":"ensure","object":"public_reproducibility","target":"tarium_project_repository"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["governance","agent_instructions"],"topics":["tarium_reproducibility"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0121.json

---

## NOM-0122 — Restrict Tarium cross-project integrations to public resources

**Version:** 1.0.0 · **Kind:** prohibition · **Authority:** general_guidance

Reference other projects only through appropriate public interfaces, resources and published dependencies; do not infer official membership for a referenced project or create hidden reliance on private repositories or data.

**Predicate:** {"action":"create","object":"unauthorized_private_or_implied_cross_project_dependency","target":"tarium_project_integration"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":"each applicable Tarium project operation","cue":"Do not","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["governance","agent_instructions"],"topics":["tarium_interoperation"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0122.json

---

## NOM-0123 — Externalize private source work through a clean reviewed public subset

**Version:** 1.0.1 · **Kind:** obligation · **Authority:** general_guidance

When separately authorized to externalize a private-origin project, classify candidate inputs privately, approve releasable items individually, rewrite or exclude unsafe material, and create a clean public repository without importing nonpublic Git history, logs or artifacts.

**Predicate:** {"action":"externalize","object":"approved_public_subset","target":"tarium_project_repository"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0004"],"depends_on":[]}

**Categories:** {"domains":["privacy","agent_instructions"],"topics":["tarium_externalization"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0123.json

---

## NOM-0124 — Review every Tarium publication surface for private-content exposure

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Before publicly publishing Tarium project material, review source files, fixtures, metadata, documentation, images, generated output, changes, commit text and release artifacts for private-origin disclosures; do not treat an automated signature check as proof of safety.

**Predicate:** {"action":"review","object":"all_publication_surfaces","target":"tarium_project_repository"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0050"],"depends_on":[]}

**Categories:** {"domains":["privacy","agent_instructions"],"topics":["tarium_publication_review"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0124.json

---

## NOM-0125 — Verify independently authorized Tarium enrollment preconditions

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Only after explicit authorization to enroll a named public repository, verify its independence, public constitution, documented ontology, human gallery, machine gallery index, licensing, public dependencies and accurate registry metadata before changing its Tarium membership entry.

**Predicate:** {"action":"verify","object":"membership_preflight","target":"central_tarium_registry"}

**Normative conditions:** ["The project owner explicitly authorized enrollment or registry modification of the named public repository."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["governance","agent_instructions"],"topics":["tarium_admission_preflight"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0125.json

---

## NOM-0126 — Version and validate evolution of a Tarium ontology

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When changing public ontology concepts, identifiers, relationships or semantics, document version changes, check reference integrity, preserve compatible identifiers where applicable and explicitly record migration or incompatibility requirements.

**Predicate:** {"action":"verify","object":"ontology_evolution","target":"tarium_project_repository"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0006"],"depends_on":[]}

**Categories:** {"domains":["ontology","agent_instructions"],"topics":["tarium_ontology_evolution"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0126.json

---

## NOM-0127 — Verify gallery and ontology cross-reference integrity

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Validate that published gallery-index entries resolve to existing public resources, that their ontology references use defined concepts and identifiers, and that the human-facing gallery agrees with its machine-readable index.

**Predicate:** {"action":"validate","object":"gallery_ontology_links","target":"tarium_project_repository"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0007"],"depends_on":[]}

**Categories:** {"domains":["ontology","agent_instructions"],"topics":["tarium_gallery_integrity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0127.json

---

## NOM-0128 — Keep Tarium project manifests and public membership claims accurate

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Keep local Tarium manifests and publicly advertised ontology and gallery entry points consistent with published project resources and actual registry state; never claim a project is enrolled merely because it has a template or is compatible with Tarium.

**Predicate:** {"action":"maintain","object":"manifest_and_membership_accuracy","target":"tarium_project_repository"}

**Normative conditions:** ["The task concerns an independently authorized public Tarium candidate or member project."]

**Normative exceptions:** ["This rule does not authorize Tarium membership changes, private-material disclosure, or unrelated repository operations."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["any"],"actor":["any"]},"project_types":["tarium_project"],"preconditions":["The user has separately authorized the relevant project task; Tarium membership remains a separately approved operation."],"exclusions":["No automatic cross-repository action, enrollment, or publication of nonpublic source data."]}

**Authority and overrides:** {"source":"Tarium public governance, selected as optional project guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["governance","agent_instructions"],"topics":["tarium_manifest_consistency"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0128.json

---

## NOM-0245 — Version only independently affected artifacts

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When revising a collection of independently versioned rules, prompts, profiles, or other artifacts, create a successor version only for each member whose own content, normative meaning, behavior, applicability, or explicit version contract changes. Preserve the stable identity, published version, and unchanged content of every unaffected member; do not increment neighboring or dependent artifact versions merely because they share a registry, keyset, export, manifest, or workflow. Update affected shared indexes, selected-version records, references, exports, documentation, and tests as necessary and when authorized, without misrepresenting those consistency updates as new versions of unchanged members. Preserve historical versions and record which artifacts changed. Honor an artifact's explicit mandatory version-bump rules, including any-change versioning policies, and separately authorized coordinated release policies. Do not silently select, activate, or publish a new version without the required authorization.

**Predicate:** {"action":"restrict","object":"successor_version_creation_to_independently_affected_artifacts","target":"versioned_artifact_collection"}

**Normative conditions:** ["A change affects one or more members of a collection of independently versioned artifacts.","Scope of an artifact-level version change must be established from the artifact's own changed content or contract."]

**Normative exceptions:** ["Artifact-specific policies may require a version increment for any change to that artifact.","An explicitly authorized coordinated release may require broader coordinated version changes.","Shared manifest, export, reference, test, or index edits required for consistency are permitted, but do not by themselves require new versions of unaffected members."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each authorized versioned-artifact modification","cue":"When revising a collection of independently versioned rules, prompts, profiles, or other artifacts","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["The affected resources have independent version identities and the proposed change is authorized."],"exclusions":["Special mandatory version-increment rules for a modified artifact remain effective.","Coordinated version updates remain permitted when explicitly authorized.","This rule never automatically selects or activates a prompt or profile."]}

**Authority and overrides:** {"source":"Explicitly owner-approved public-safe generalized versioning instruction; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0016"],"depends_on":[]}

**Categories:** {"domains":["agent_instructions","governance","software_development"],"topics":["version_consistency","change_scope"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0245.json

---

## NOM-0247 — Prefer MIT for original projects while honoring chosen dependency licenses

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When selecting the license for newly owned original project code at an authorized bootstrap, prefer MIT unless the owner chooses otherwise or relevant third-party terms require another arrangement. Preserve an existing project's license unless a change is justified and authorized. Judge useful libraries and source code on their merits as well as their actual licensing obligations; do not automatically reject or reimplement worthwhile code just to retain MIT. Concessions to GPL, AGPL, LGPL or other licenses are acceptable when the owner chooses those dependencies and the intended integration, use and distribution can honor their terms. Verify the actual license, copyright and notice obligations, distribution and network-use conditions, and whether incorporation, copying or linking imposes reciprocal obligations on the resulting combined work. Neither assume all copyleft dependencies force unrelated code to be relicensed nor claim MIT compatibility without checking the specific circumstances. Preserve required notices and provenance, record significant trade-offs, and seek owner authorization before changing an existing repository's overall license. Where legally material terms remain uncertain, disclose the uncertainty and obtain appropriate review rather than inventing legal conclusions.

**Predicate:** {"action":"prefer_and_evaluate","object":"MIT_default_with_informed_dependency_license_concessions","target":"project_license_decision"}

**Normative conditions:** ["A new project or original work is being licensed, or code dependencies present a licensing decision.","The project owner has authorized the relevant project or dependency decision."]

**Normative exceptions:** ["The owner expressly chooses a different license or must comply with third-party license terms.","Existing project licenses cannot be changed without applicable owner approval.","MIT preference never overrides real license obligations or licenses that require a different arrangement."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each owner-authorized licensing decision","cue":"When licensing original project code or evaluating dependencies","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["An authorized project bootstrap, source incorporation, or project license decision."],"exclusions":["Do not silently relicense existing projects.","Do not presume that every copyleft dependency imposes the same obligations on all uses.","Not a substitute for legal review where material license implications are uncertain."]}

**Authority and overrides:** {"source":"Explicit owner-approved generalized MIT-first preference with informed dependency concessions; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0019"],"depends_on":["NOM-0049"]}

**Categories:** {"domains":["software_development","governance","agent_instructions"],"topics":["license_compliance","source_provenance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0247.json

---

## NOM-0248 — Avoid npm for project tooling and prefer native established alternatives

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Do not use or instruct the project owner to use npm for installation, development, building, testing or release workflows. Prefer Rust/Cargo for application implementations, Lua for scripting, and existing native tools or maintained preinstalled dependencies. For Arch-based operating-system package management prefer the owner's established yay workflow rather than imposing another manager. Respect existing project toolchains and external dependencies where feasible, but if a required upstream workflow specifically depends on npm, report the concrete need and constraints and request explicit authorization for an alternative strategy; do not silently run npm, rewrite unrelated build systems, or pretend npm-dependent work was verified.

**Predicate:** {"action":"enforce","object":"no_npm_prefer_cargo_lua_existing_native_tools","target":"project_workflow"}

**Normative conditions:** ["Work involves selecting, invoking, documenting or requesting developer-facing package/build tools."]

**Normative exceptions:** ["Project-specific constraints override generic tool preferences only when explicitly authorized; never invoke npm without the owner's change of instruction.","Do not unnecessarily replace or disrupt existing project build systems outside authorized scope."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable project decision","cue":"When starting, maintaining or executing work in the applicable project","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["Work involves selecting, invoking, documenting or requesting developer-facing package/build tools."],"exclusions":["Project-specific constraints override generic tool preferences only when explicitly authorized; never invoke npm without the owner's change of instruction.","Do not unnecessarily replace or disrupt existing project build systems outside authorized scope."]}

**Authority and overrides:** {"source":"Explicit owner-approved generalized MIT-first preference with informed dependency concessions; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0008","NOM-0016"],"depends_on":[]}

**Categories:** {"domains":["software_development","governance","agent_instructions"],"topics":["license_compliance","source_provenance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0248.json

---

## NOM-0249 — Assign OKLab identity colors when establishing new project branding

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When initializing a new project whose owner has not already assigned an identity color, generate a stable, distinctive color using OKLab or its polar form OKLCH; assess perceptual distinction, readable day/night contrast and non-color identifiers. Record the project's chosen color and rationale in its README or established branding records and apply it consistently to project graphics and optional UI tokens. Preserve existing assigned colors during project adoption and normal maintenance unless the owner authorizes rebranding. For user interfaces, independently comply with light, dark and system-default appearance requirements; a project identity color does not replace the theme selector or accessible contrast checks.

**Predicate:** {"action":"enforce","object":"oklab_new_project_identity_color","target":"project_workflow"}

**Normative conditions:** ["A new project is being established or an explicitly authorized project branding decision is required."]

**Normative exceptions:** ["Do not impose a new identity color on an existing project with established colors without rebranding authorization.","Do not falsely claim mathematically validated color separation without checking the comparison set."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable project decision","cue":"When starting, maintaining or executing work in the applicable project","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["A new project is being established or an explicitly authorized project branding decision is required."],"exclusions":["Do not impose a new identity color on an existing project with established colors without rebranding authorization.","Do not falsely claim mathematically validated color separation without checking the comparison set."]}

**Authority and overrides:** {"source":"Explicit owner-approved generalized MIT-first preference with informed dependency concessions; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0009","NOM-0073","NOM-0005"],"depends_on":[]}

**Categories:** {"domains":["software_development","governance","agent_instructions"],"topics":["license_compliance","source_provenance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0249.json
