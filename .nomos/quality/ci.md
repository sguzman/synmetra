# Nomos / quality/ci.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0034 — Continuous validation workflows

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Configure an appropriate GitHub Actions or comparable validation workflow for a repository.

**Predicate:** {"action":"practice","object":"continuous_validation_workflows","target":"authorized_project_work"}

**Normative conditions:** ["When automated CI is appropriate and within authorized project work","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When automated CI is appropriate and within authorized project work"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["ci_cd","software_development"],"topics":["ci_workflows"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0034.json

---

## NOM-0035 — CI event triggers

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Run configured checks for appropriate events, such as pushes, while respecting repository workflow policies.

**Predicate:** {"action":"practice","object":"ci_event_triggers","target":"authorized_project_work"}

**Normative conditions:** ["When defining CI triggers","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When defining CI triggers"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["ci_cd","software_development"],"topics":["ci_triggers"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0035.json

---

## NOM-0036 — CI quality gates

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Combine applicable formatting, compilation, linting and test checks into automated validation stages.

**Predicate:** {"action":"practice","object":"ci_quality_gates","target":"authorized_project_work"}

**Normative conditions:** ["When defining a project CI pipeline","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When defining a project CI pipeline"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["ci_cd","software_development","software_testing"],"topics":["ci_quality_gates"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0036.json

---

## NOM-0037 — Cross-platform CI checks

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Exercise supported operating systems in CI without claiming unsupported platform compatibility.

**Predicate:** {"action":"practice","object":"cross_platform_ci_checks","target":"authorized_project_work"}

**Normative conditions:** ["When a project advertises multiple target platforms","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When a project advertises multiple target platforms"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["ci_cd","software_development"],"topics":["cross_platform_ci"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0037.json

---

## NOM-0038 — Deployment workflows

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Automate approved website or artifact deployment through GitHub Pages or an appropriate existing target.

**Predicate:** {"action":"practice","object":"deployment_workflows","target":"authorized_project_work"}

**Normative conditions:** ["When a project has an authorized deployment target","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When a project has an authorized deployment target"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["ci_cd","software_development"],"topics":["deployment_workflows"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0038.json

---

## NOM-0039 — Repair failing CI

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Investigate failed workflow steps, fix underlying causes where authorized and recheck status.

**Predicate:** {"action":"practice","object":"repair_failing_ci","target":"authorized_project_work"}

**Normative conditions:** ["When an authorized change causes or encounters CI failure","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When an authorized change causes or encounters CI failure"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["ci_cd","software_development"],"topics":["ci_repair"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0039.json

---

## NOM-0040 — Workflow configuration maintenance

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Maintain workflow permissions, triggers, versions and paths to match actual repository needs.

**Predicate:** {"action":"practice","object":"workflow_configuration_maintenance","target":"authorized_project_work"}

**Normative conditions:** ["When workflow files or project structure change","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When workflow files or project structure change"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["ci_cd","software_development"],"topics":["ci_configuration"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0040.json

---

## NOM-0041 — Verify CI outcomes

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Report whether each relevant check was successful, failed, skipped or pending rather than presuming success.

**Predicate:** {"action":"practice","object":"verify_ci_outcomes","target":"authorized_project_work"}

**Normative conditions:** ["When reporting automated validation","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When reporting automated validation"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["ci_cd","software_development"],"topics":["ci_result_verification"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0041.json
