# Nomos / gui/typography.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0250 — Prefer Atkinson Hyperlegible Next Bold for everyday graphical interface text

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When selecting typography for everyday graphical interface text, recommend Atkinson Hyperlegible Next Bold first and normally use it as the default if it suits the actual project. Candidate uses include buttons, menus, toolbars, navigation items, settings and desktop controls. This is a defeasible preference, not a mandatory typeface, exclusive whitelist or permission to redesign an existing artifact. Evaluate suitability in context, including reading comfort, hierarchy, accessible contrast, letterforms and glyph discrimination, available font weights, language and symbol coverage, licensing/redistribution, platform availability, layout performance and design cohesion. Preserve established project typography unless changing it is within authorized scope. If another face is materially better, unavailable fonts make the default impractical, or the owner prefers a different choice, select a suitable alternative or fallback and briefly record the reason where material; do not invent a new feature or redesign merely to satisfy this preference.

**Predicate:** {"action":"consider_and_prefer","object":"typography_interface_font_default","target":"interface_typography_selection"}

**Normative conditions:** ["A font decision is authorized and the relevant UI includes ordinary interactive labels and the owner has not chosen an overriding established type system.","No higher-priority owner/project requirement has already decided the typeface for this role."]

**Normative exceptions:** ["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized font-selection decision","cue":"When selecting typography","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["A font decision is authorized and the relevant UI includes ordinary interactive labels and the owner has not chosen an overriding established type system.","No higher-priority owner/project requirement has already decided the typeface for this role."],"exclusions":["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]}

**Authority and overrides:** {"source":"Owner-approved contextual typography defaults; opt-in selection guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["user_interfaces","visual_identity"],"topics":["typography_interface"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0250.json

---

## NOM-0251 — Prefer Lexend for extended reading and explanatory text

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When selecting typography for extended reading and explanatory text, recommend Lexend first and normally use it as the default if it suits the actual project. Candidate uses include body copy, articles, documentation, descriptions and reading-heavy application surfaces. This is a defeasible preference, not a mandatory typeface, exclusive whitelist or permission to redesign an existing artifact. Evaluate suitability in context, including reading comfort, hierarchy, accessible contrast, letterforms and glyph discrimination, available font weights, language and symbol coverage, licensing/redistribution, platform availability, layout performance and design cohesion. Preserve established project typography unless changing it is within authorized scope. If another face is materially better, unavailable fonts make the default impractical, or the owner prefers a different choice, select a suitable alternative or fallback and briefly record the reason where material; do not invent a new feature or redesign merely to satisfy this preference.

**Predicate:** {"action":"consider_and_prefer","object":"typography_reading_font_default","target":"reading_typography_selection"}

**Normative conditions:** ["A font decision is authorized and the artifact presents passages intended for sustained reading.","No higher-priority owner/project requirement has already decided the typeface for this role."]

**Normative exceptions:** ["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized font-selection decision","cue":"When selecting typography","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["A font decision is authorized and the artifact presents passages intended for sustained reading.","No higher-priority owner/project requirement has already decided the typeface for this role."],"exclusions":["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]}

**Authority and overrides:** {"source":"Owner-approved contextual typography defaults; opt-in selection guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["user_interfaces","visual_identity"],"topics":["typography_reading"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0251.json

---

## NOM-0252 — Prefer Monaspace Krypton Nerd Font Bold for monospaced technical presentation

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When selecting typography for monospaced technical presentation, recommend Monaspace Krypton Nerd Font Bold first and normally use it as the default if it suits the actual project. Candidate uses include terminal interfaces, source code, logs, commands, technical identifiers and aligned diagnostics. This is a defeasible preference, not a mandatory typeface, exclusive whitelist or permission to redesign an existing artifact. Evaluate suitability in context, including reading comfort, hierarchy, accessible contrast, letterforms and glyph discrimination, available font weights, language and symbol coverage, licensing/redistribution, platform availability, layout performance and design cohesion. Preserve established project typography unless changing it is within authorized scope. If another face is materially better, unavailable fonts make the default impractical, or the owner prefers a different choice, select a suitable alternative or fallback and briefly record the reason where material; do not invent a new feature or redesign merely to satisfy this preference.

**Predicate:** {"action":"consider_and_prefer","object":"typography_monospace_font_default","target":"monospace_typography_selection"}

**Normative conditions:** ["A font decision is authorized and a fixed-width typeface is appropriate to the content and presentation.","No higher-priority owner/project requirement has already decided the typeface for this role."]

**Normative exceptions:** ["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized font-selection decision","cue":"When selecting typography","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["A font decision is authorized and a fixed-width typeface is appropriate to the content and presentation.","No higher-priority owner/project requirement has already decided the typeface for this role."],"exclusions":["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]}

**Authority and overrides:** {"source":"Owner-approved contextual typography defaults; opt-in selection guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["user_interfaces","visual_identity"],"topics":["typography_monospace"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0252.json

---

## NOM-0253 — Prefer Luciole Bold for general information hierarchy headings

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When selecting typography for general information hierarchy headings, recommend Luciole Bold first and normally use it as the default if it suits the actual project. Candidate uses include section headings, dialog titles, panel headers and prominent informational labels. This is a defeasible preference, not a mandatory typeface, exclusive whitelist or permission to redesign an existing artifact. Evaluate suitability in context, including reading comfort, hierarchy, accessible contrast, letterforms and glyph discrimination, available font weights, language and symbol coverage, licensing/redistribution, platform availability, layout performance and design cohesion. Preserve established project typography unless changing it is within authorized scope. If another face is materially better, unavailable fonts make the default impractical, or the owner prefers a different choice, select a suitable alternative or fallback and briefly record the reason where material; do not invent a new feature or redesign merely to satisfy this preference.

**Predicate:** {"action":"consider_and_prefer","object":"typography_headings_font_default","target":"headings_typography_selection"}

**Normative conditions:** ["A font decision is authorized and the text is an ordinary heading rather than a deliberately expressive display title.","No higher-priority owner/project requirement has already decided the typeface for this role."]

**Normative exceptions:** ["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized font-selection decision","cue":"When selecting typography","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["A font decision is authorized and the text is an ordinary heading rather than a deliberately expressive display title.","No higher-priority owner/project requirement has already decided the typeface for this role."],"exclusions":["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]}

**Authority and overrides:** {"source":"Owner-approved contextual typography defaults; opt-in selection guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["user_interfaces","visual_identity"],"topics":["typography_headings"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0253.json

---

## NOM-0254 — Prefer Lexica Ultralegible for legibility-critical interface copy

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When selecting typography for legibility-critical interface copy, recommend Lexica Ultralegible first and normally use it as the default if it suits the actual project. Candidate uses include instruction text, forms, error guidance, dense interactive instructions and character-sensitive labels. This is a defeasible preference, not a mandatory typeface, exclusive whitelist or permission to redesign an existing artifact. Evaluate suitability in context, including reading comfort, hierarchy, accessible contrast, letterforms and glyph discrimination, available font weights, language and symbol coverage, licensing/redistribution, platform availability, layout performance and design cohesion. Preserve established project typography unless changing it is within authorized scope. If another face is materially better, unavailable fonts make the default impractical, or the owner prefers a different choice, select a suitable alternative or fallback and briefly record the reason where material; do not invent a new feature or redesign merely to satisfy this preference.

**Predicate:** {"action":"consider_and_prefer","object":"typography_accessibility_font_default","target":"accessibility_typography_selection"}

**Normative conditions:** ["A font decision is authorized and the surface has an elevated legibility or character-discrimination requirement.","No higher-priority owner/project requirement has already decided the typeface for this role."]

**Normative exceptions:** ["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized font-selection decision","cue":"When selecting typography","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["A font decision is authorized and the surface has an elevated legibility or character-discrimination requirement.","No higher-priority owner/project requirement has already decided the typeface for this role."],"exclusions":["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]}

**Authority and overrides:** {"source":"Owner-approved contextual typography defaults; opt-in selection guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["user_interfaces","visual_identity"],"topics":["typography_accessibility"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0254.json

---

## NOM-0255 — Prefer League Gothic for condensed editorial display typography

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When selecting typography for condensed editorial display typography, recommend League Gothic first and normally use it as the default if it suits the actual project. Candidate uses include large editorial headlines, posters, cover titles and prominent condensed uppercase display treatments. This is a defeasible preference, not a mandatory typeface, exclusive whitelist or permission to redesign an existing artifact. Evaluate suitability in context, including reading comfort, hierarchy, accessible contrast, letterforms and glyph discrimination, available font weights, language and symbol coverage, licensing/redistribution, platform availability, layout performance and design cohesion. Preserve established project typography unless changing it is within authorized scope. If another face is materially better, unavailable fonts make the default impractical, or the owner prefers a different choice, select a suitable alternative or fallback and briefly record the reason where material; do not invent a new feature or redesign merely to satisfy this preference.

**Predicate:** {"action":"consider_and_prefer","object":"typography_display_font_default","target":"display_typography_selection"}

**Normative conditions:** ["A font decision is authorized and the design calls for impactful condensed display text rather than general body or control text.","No higher-priority owner/project requirement has already decided the typeface for this role."]

**Normative exceptions:** ["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"each applicable authorized font-selection decision","cue":"When selecting typography","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["A font decision is authorized and the design calls for impactful condensed display text rather than general body or control text.","No higher-priority owner/project requirement has already decided the typeface for this role."],"exclusions":["An established project style, explicit owner direction, genuine accessibility need, unavailable glyph coverage, font-license restriction, platform limitation, performance/layout concern, or a demonstrably better contextual fit can justify an alternative.","Never construe the preferred font as mandatory, exclusive, or a requirement to alter otherwise satisfactory existing typography."]}

**Authority and overrides:** {"source":"Owner-approved contextual typography defaults; opt-in selection guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["user_interfaces","visual_identity"],"topics":["typography_display"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0255.json
