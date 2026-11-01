# Nomos / tarium/standards.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0242 — derivation reproducibility

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For a Tarium project's published derived dataset, ontology specimen or gallery artifact, record the public input identities and relevant transformation versions, parameters or steps sufficient to reproduce the published result where reproduction is promised.

**Predicate:** {"action":"verify","object":"tarium_derivation_reproducibility","target":"tarium_project"}

**Normative conditions:** ["An owner-authorized independent public Tarium project uses or publishes the relevant dataset, ontology, artifact or workflow."]

**Normative exceptions:** ["No new data collection or external publication is authorized merely by this obligation; excluded resources need no invented substitute."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each authorized applicable project acceptance","cue":"where the specified context is applicable","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An owner-authorized independent public Tarium project uses or publishes the relevant dataset, ontology, artifact or workflow."],"exclusions":["No new data collection or external publication is authorized merely by this obligation; excluded resources need no invented substitute."],"project_types":["tarium_project"]}

**Authority and overrides:** {"source":"Owner-authorized Nomos tarium coverage remediation; opt-in only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0049"]}

**Categories:** {"domains":["data_integrity","documentation","ontology"],"topics":["tarium_reproducibility"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0242.json

---

## NOM-0243 — schema conformance validation

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For Tarium machine-readable ontology and gallery schemas, validate required fields, stable IDs, type constraints and cross-references automatically against representative valid and invalid public fixtures when such structured schemas are declared.

**Predicate:** {"action":"verify","object":"tarium_schema_conformance_validation","target":"tarium_project"}

**Normative conditions:** ["An owner-authorized independent public Tarium project uses or publishes the relevant dataset, ontology, artifact or workflow."]

**Normative exceptions:** ["No new data collection or external publication is authorized merely by this obligation; excluded resources need no invented substitute."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each authorized applicable project acceptance","cue":"where the specified context is applicable","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An owner-authorized independent public Tarium project uses or publishes the relevant dataset, ontology, artifact or workflow."],"exclusions":["No new data collection or external publication is authorized merely by this obligation; excluded resources need no invented substitute."],"project_types":["tarium_project"]}

**Authority and overrides:** {"source":"Owner-authorized Nomos tarium coverage remediation; opt-in only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0127"]}

**Categories:** {"domains":["ontology","qa_verification","data_integrity"],"topics":["tarium_gallery_integrity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0243.json
