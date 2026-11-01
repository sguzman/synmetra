# Nomos / development/rust.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0008 — Prefer Rust for application code and justify narrow exceptions

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Write application implementation code in Rust; permit non-Rust website code only with a recorded justification; use Lua when scripting is required, and prefer TOML, JSON, and Markdown for their respective configuration and documentation roles.

**Predicate:** {"action":"choose","object":"implementation_language","target":"software_project"}

**Normative conditions:** ["Scripting uses Lua; declarative configuration and documentation use their appropriate formats."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":[],"project_types":["software_project"]}

**Authority and overrides:** {"source":"Published in Nomos by explicit batch authorization; general guidance, never automatically applied","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_development","software_development"],"topics":["implementation_language","rust_preference"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0008.json

---

## NOM-0012 — Choose the latest stable supported Rust edition

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

When selecting a Cargo package edition, use the newest stable Rust edition supported by the project's toolchain and compatibility requirements.

**Predicate:** {"action":"set","object":"rust_edition","target":"cargo_manifest"}

**Normative conditions:** ["Choose newest stable edition supported by the targeted toolchain and project compatibility requirements, at edition-selection time."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":[],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published in Nomos by explicit batch authorization; general guidance, never automatically applied","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_development","software_development"],"topics":["edition_selection"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0012.json

---

## NOM-0013 — Choose Rust edition 2024 for Cargo packages

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

When selecting a Cargo package edition, set the [package].edition value to 2024.

**Predicate:** {"action":"set","object":"rust_edition","target":"cargo_manifest"}

**Normative conditions:** ["Fixed Cargo edition value, even if a newer edition later becomes available."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":[],"project_types":["rust_project"]}

**Authority and overrides:** {"source":"Published in Nomos by explicit batch authorization; general guidance, never automatically applied","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["rust_development","software_development"],"topics":["edition_selection"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0013.json
