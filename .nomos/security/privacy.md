# Nomos / security/privacy.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0005 — Require system-aware appearance themes in graphical interfaces

**Version:** 1.1.1 · **Kind:** obligation · **Authority:** general_guidance

Graphical and web applications must support light, dark, and system appearance modes; start with system mode and provide an immediately effective on-screen dropdown for changing modes.

**Predicate:** {"action":"configure","object":"appearance_modes","target":"gui_or_website"}

**Normative conditions:** []

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":["Must be an interface project."],"project_types":["any"]}

**Authority and overrides:** {"source":"Published in Nomos by explicit batch authorization; general guidance, never automatically applied","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["software_development","user_interfaces"],"topics":["content_handling","gui_theme_support","theme_control"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0005.json
