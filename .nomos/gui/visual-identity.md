# Nomos / gui/visual-identity.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0009 — Derive stable, distinguishable visual identities from strings

**Version:** 1.1.1 · **Kind:** obligation · **Authority:** general_guidance

Assign stable, deterministic identity colors from string IDs using a perceptually aware color method that prioritizes human distinction and preserves readable non-color identifiers.

**Predicate:** {"action":"derive","object":"identity_color","target":"visual_identity"}

**Normative conditions:** ["Finite color spaces cannot guarantee globally unique colors."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["website"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":[],"project_types":["any"]}

**Authority and overrides:** {"source":"Published in Nomos by explicit batch authorization; general guidance, never automatically applied","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["visual_identity"],"topics":["identity_color","identity_colors"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0009.json

---

## NOM-0073 — Display project identity colors in README documentation

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Embed a project's assigned stable, perceptually distinguishable identity color in its README using a portable rendering such as SVG or CSS when suitable.

**Predicate:** {"action":"show","object":"stable_project_color","target":"project_readme"}

**Normative conditions:** ["The project has an assigned color and a public-facing README."]

**Normative exceptions:** ["Do not expose private project artifacts; preserve README rendering compatibility."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["The project has an assigned color and a public-facing README."],"exclusions":["Do not expose private project artifacts; preserve README rendering compatibility."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos catalog guidance; opt-in and subject to task-level authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0009"],"depends_on":["NOM-0042"]}

**Categories:** {"domains":["documentation","visual_identity"],"topics":["readme_identity"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0073.json
