# Nomos / tarium/membership.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0241 — public checkout operability

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

From a clean public checkout or equivalent isolated public environment, exercise the project's essential documented workflow using only declared public dependencies; report inaccessible or unpublished inputs instead of silently relying on private workspaces.

**Predicate:** {"action":"verify","object":"tarium_public_checkout_operability","target":"tarium_project"}

**Normative conditions:** ["An owner-authorized independent public Tarium project uses or publishes the relevant dataset, ontology, artifact or workflow."]

**Normative exceptions:** ["No new data collection or external publication is authorized merely by this obligation; excluded resources need no invented substitute."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each authorized applicable project acceptance","cue":"where the specified context is applicable","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An owner-authorized independent public Tarium project uses or publishes the relevant dataset, ontology, artifact or workflow."],"exclusions":["No new data collection or external publication is authorized merely by this obligation; excluded resources need no invented substitute."],"project_types":["tarium_project"]}

**Authority and overrides:** {"source":"Owner-authorized Nomos tarium coverage remediation; opt-in only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0121"]}

**Categories:** {"domains":["qa_verification","publication","software_testing"],"topics":["tarium_autonomy"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0241.json
