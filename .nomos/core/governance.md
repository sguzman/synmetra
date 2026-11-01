# Nomos / core/governance.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0006 — Represent Tarium ontology for humans and machines

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Each explicitly enrolled Tarium project must publish mutually consistent machine-readable ontology records and human-readable ontology documentation.

**Predicate:** {"action":"provide","object":"ontology_representations","target":"tarium_member_project"}

**Normative conditions:** ["Only applies to explicitly enrolled Tarium projects."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":[],"project_types":["tarium_project"]}

**Authority and overrides:** {"source":"Published in Nomos by explicit batch authorization; general guidance, never automatically applied","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["governance","ontology"],"topics":["ontology_representation","representation"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0006.json

---

## NOM-0007 — Maintain a per-project ontology and public specimen showcase

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Each explicitly enrolled Tarium project must maintain its own independently usable ontology and public website showcasing representative ontology specimens.

**Predicate:** {"action":"maintain","object":"ontology_and_specimen_showcase","target":"tarium_member_project"}

**Normative conditions:** ["Tarium membership must be explicitly approved."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["website"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":[],"project_types":["tarium_project"]}

**Authority and overrides:** {"source":"Published in Nomos by explicit batch authorization; general guidance, never automatically applied","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["governance","ontology","publication"],"topics":["ontology_gallery","specimen_showcase"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0007.json

---

## NOM-0239 — upstream notice continuity

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When distributing copied upstream or third-party source or compiled artifacts in a fork, verify required licenses, attribution and notices remain attached to the material actually redistributed.

**Predicate:** {"action":"verify","object":"fork_upstream_notice_continuity","target":"rust_fork_maintenance"}

**Normative conditions:** ["An owner-authorized Rust fork maintenance task has an accepted upstream baseline, and this concern falls within the maintained fork's supported contracts."]

**Normative exceptions:** ["Unpublished or unsupported upstream behavior is not automatically incorporated; approved scope and documented exceptions still govern."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each authorized applicable project acceptance","cue":"where the specified context is applicable","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An owner-authorized Rust fork maintenance task has an accepted upstream baseline, and this concern falls within the maintained fork's supported contracts."],"exclusions":["Unpublished or unsupported upstream behavior is not automatically incorporated; approved scope and documented exceptions still govern."],"project_types":["rust_fork_maintenance"]}

**Authority and overrides:** {"source":"Owner-authorized Nomos fork coverage remediation; opt-in only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0019"]}

**Categories:** {"domains":["maintenance","publication","governance"],"topics":["license_compliance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0239.json

---

## NOM-0240 — asset redistribution rights

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

Before publishing imported data or media in a Tarium project, verify source-specific redistribution rights and attribution obligations for each materially distinct dataset or resource, preserving evidence and recording excluded assets.

**Predicate:** {"action":"verify","object":"tarium_asset_redistribution_rights","target":"tarium_project"}

**Normative conditions:** ["An owner-authorized independent public Tarium project uses or publishes the relevant dataset, ontology, artifact or workflow."]

**Normative exceptions:** ["No new data collection or external publication is authorized merely by this obligation; excluded resources need no invented substitute."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each authorized applicable project acceptance","cue":"where the specified context is applicable","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An owner-authorized independent public Tarium project uses or publishes the relevant dataset, ontology, artifact or workflow."],"exclusions":["No new data collection or external publication is authorized merely by this obligation; excluded resources need no invented substitute."],"project_types":["tarium_project"]}

**Authority and overrides:** {"source":"Owner-authorized Nomos tarium coverage remediation; opt-in only","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0049"]}

**Categories:** {"domains":["publication","data_integrity","governance"],"topics":["source_provenance"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0240.json
