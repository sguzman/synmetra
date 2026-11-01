# Nomos / delivery/documentation.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0042 — README upkeep

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Keep the project README aligned with its purpose, setup, usage and development procedures.

**Predicate:** {"action":"practice","object":"readme_upkeep","target":"authorized_project_work"}

**Normative conditions:** ["When user-facing project behavior or setup changes","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When user-facing project behavior or setup changes"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["documentation","software_development"],"topics":["readme_maintenance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0042.json

---

## NOM-0044 — Architecture documentation

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Document consequential design decisions, interfaces and data models in project-appropriate files.

**Predicate:** {"action":"practice","object":"architecture_documentation","target":"authorized_project_work"}

**Normative conditions:** ["When architecture changes merit durable explanation","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When architecture changes merit durable explanation"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["documentation","project_architecture","software_development"],"topics":["architecture_documentation"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0044.json

---

## NOM-0045 — Configuration reference

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Document configuration keys, defaults, examples and environment requirements.

**Predicate:** {"action":"practice","object":"configuration_reference","target":"authorized_project_work"}

**Normative conditions:** ["When configuration surfaces are added or changed","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When configuration surfaces are added or changed"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["documentation","software_development"],"topics":["configuration_documentation"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0045.json

---

## NOM-0046 — CLI help accuracy

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Synchronize command help, flags and examples with implemented command-line behavior.

**Predicate:** {"action":"practice","object":"cli_help_accuracy","target":"authorized_project_work"}

**Normative conditions:** ["When a CLI interface changes","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When a CLI interface changes"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["documentation","software_development"],"topics":["cli_help"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0046.json

---

## NOM-0047 — Changelog upkeep

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Record notable user-facing changes in the repository's established changelog or release notes.

**Predicate:** {"action":"practice","object":"changelog_upkeep","target":"authorized_project_work"}

**Normative conditions:** ["When the repository uses changelogs","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When the repository uses changelogs"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["documentation","maintenance","software_development"],"topics":["changelog"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0047.json

---

## NOM-0048 — Documentation consistency audit

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Identify and correct stale instructions, inaccurate claims and broken internal links.

**Predicate:** {"action":"practice","object":"documentation_consistency_audit","target":"authorized_project_work"}

**Normative conditions:** ["When documentation is updated or audited","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When documentation is updated or audited"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["documentation","software_development"],"topics":["documentation_audit"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0048.json

---

## NOM-0049 — Source provenance and attribution

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Preserve publication-safe references, licenses and hashes needed to explain imported data or resources.

**Predicate:** {"action":"practice","object":"source_provenance_and_attribution","target":"authorized_project_work"}

**Normative conditions:** ["When material from outside the project is incorporated","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When material from outside the project is incorporated"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["documentation","software_development"],"topics":["source_provenance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0049.json

---

## NOM-0136 — Keep repository READMEs approachable and separate from change histories

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Write a repository README as a professional, welcoming orientation for new visitors: briefly explain what the project is, its value, how to get started, and links to relevant resources. Do not use it as an accumulating changelog, internal progress log, or dump of implementation details. Put extensive explanations, designs, technical references, and long-form guides under docs/; where a chronological change history is useful, maintain a separate changelog file such as CHANGELOG.md and link to it. Brief release highlights in a README are permissible when helpful.

**Predicate:** {"action":"maintain","object":"concise_visitor_friendly_readme_with_linked_detailed_docs","target":"public_repository_readme"}

**Normative conditions:** ["When preparing a public-facing repository README."]

**Normative exceptions:** ["Short release highlights are permitted; a separate changelog is needed only when chronological history is maintained."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["public"],"artifact_kind":["repository_content"],"actor":["any"]},"project_types":["project"],"preconditions":["When preparing a public-facing repository README."],"exclusions":["Short release highlights are permitted; a separate changelog is needed only when chronological history is maintained."]}

**Authority and overrides:** {"source":"Explicitly reviewed public-safe specimen normalization; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0042","NOM-0047","NOM-0048"],"depends_on":[]}

**Categories:** {"domains":["documentation","publication"],"topics":["readme_public_orientation_and_documentation_separation"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0136.json
