# Nomos / delivery/releases.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0055 — Semantic version policy

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Use coherent major, minor and patch version semantics for projects adopting semantic versioning.

**Predicate:** {"action":"practice","object":"semantic_version_policy","target":"authorized_project_work"}

**Normative conditions:** ["When managing versioned releases","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When managing versioned releases"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["releases","software_development"],"topics":["semantic_versioning"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0055.json

---

## NOM-0056 — Synchronize version references

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Keep declared package versions, relevant manifests, release notes and tags mutually consistent.

**Predicate:** {"action":"practice","object":"synchronize_version_references","target":"authorized_project_work"}

**Normative conditions:** ["When preparing or publishing version changes","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When preparing or publishing version changes"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["releases","software_development"],"topics":["version_consistency"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0056.json

---

## NOM-0057 — Release readiness checks

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Confirm builds, tests, package contents and release notes before an authorized release.

**Predicate:** {"action":"practice","object":"release_readiness_checks","target":"authorized_project_work"}

**Normative conditions:** ["Before producing a release","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Before producing a release"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["releases","software_development"],"topics":["release_readiness"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0057.json

---

## NOM-0058 — Versioned release tags

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Create and verify version tags only when publishing a release is explicitly authorized.

**Predicate:** {"action":"practice","object":"versioned_release_tags","target":"authorized_project_work"}

**Normative conditions:** ["When an authorized release is made","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When an authorized release is made"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["releases","software_development"],"topics":["release_tagging"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0058.json

---

## NOM-0059 — Release artifact packaging

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Build and assemble platform-appropriate distributable binaries, libraries or archives for authorized releases.

**Predicate:** {"action":"practice","object":"release_artifact_packaging","target":"authorized_project_work"}

**Normative conditions:** ["When a release distributes artifacts","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When a release distributes artifacts"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["releases","software_development"],"topics":["artifact_packaging"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0059.json

---

## NOM-0061 — Release workflow automation

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Use relevant release tooling or workflows when automation adds value and publishing scope is authorized.

**Predicate:** {"action":"practice","object":"release_workflow_automation","target":"authorized_project_work"}

**Normative conditions:** ["When maintaining a release process","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When maintaining a release process"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["releases","software_development"],"topics":["release_automation"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0061.json
