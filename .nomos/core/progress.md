# Nomos / core/progress.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0010 — Create structured roadmaps with stable emoji identities

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

New projects must begin with a clear roadmap of phases, milestones, subtasks, and features; give each roadmap item its own stable identifying emoji separate from status.

**Predicate:** {"action":"define","object":"roadmap_identity","target":"new_project"}

**Normative conditions:** ["Applies when creating a project from scratch."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"unspecified","minimum":null,"maximum":null,"unit":"entities","window":null,"cue":"each roadmap item","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":[],"exclusions":[],"project_types":["project"]}

**Authority and overrides:** {"source":"Published in Nomos by explicit batch authorization; general guidance, never automatically applied","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["governance","project_management","software_development"],"topics":["project_roadmap","roadmap_structure"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0010.json

---

## NOM-0043 — Roadmap status accuracy

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Maintain milestones and task completion states according to evidence; never equate a future idea with an authorized task.

**Predicate:** {"action":"practice","object":"roadmap_status_accuracy","target":"authorized_project_work"}

**Normative conditions:** ["When project planning is part of work","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When project planning is part of work"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["documentation","project_management","software_development"],"topics":["roadmap_status"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0043.json

---

## NOM-0068 — Report progress for touched roadmap items

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Each development progress report should show progress for every phase, milestone, task and feature worked on during its reporting interval.

**Predicate:** {"action":"report","object":"touched_work_item_progress","target":"development_progress_report"}

**Normative conditions:** ["A project progress report covers work performed during the reporting interval."]

**Normative exceptions:** ["Items whose completed state was already announced and not reopened."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"unspecified","minimum":null,"maximum":null,"unit":"entities","window":null,"cue":"every phase, milestone, task, and feature","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A project progress report covers work performed during the reporting interval."],"exclusions":["Items whose completed state was already announced and not reopened."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos catalog guidance; opt-in and subject to task-level authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["project_management"],"topics":["progress_reporting"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0068.json

---

## NOM-0069 — Pair progress percentages with visual progress bars

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

For each reported work item, show a numeric completion percentage together with a visual progress bar conveying the same value.

**Predicate:** {"action":"show","object":"numeric_and_visual_progress","target":"work_item_progress_report"}

**Normative conditions:** ["A work item's completion is quantifiable from available evidence."]

**Normative exceptions:** ["Do not invent numerical precision when progress cannot be estimated."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"unspecified","minimum":null,"maximum":null,"unit":"entities","window":null,"cue":"For each reported work item","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["A work item's completion is quantifiable from available evidence."],"exclusions":["Do not invent numerical precision when progress cannot be estimated."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos catalog guidance; opt-in and subject to task-level authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["project_management","user_interfaces"],"topics":["progress_reporting"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0069.json

---

## NOM-0070 — Report overall project progress alongside individual items

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Include an overall project progress summary alongside the progress of relevant phases, milestones, tasks and features.

**Predicate:** {"action":"report","object":"overall_and_individual_progress","target":"project_progress_report"}

**Normative conditions:** ["Reporting on progress for an authorized project."]

**Normative exceptions:** ["Do not double-count nested milestones or infer unverified aggregate percentages."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["Reporting on progress for an authorized project."],"exclusions":["Do not double-count nested milestones or infer unverified aggregate percentages."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos catalog guidance; opt-in and subject to task-level authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["project_management"],"topics":["progress_reporting"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0070.json

---

## NOM-0137 — Show previous, current, and per-round change beside visual progress bars

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

For each actively reported project or unfinished roadmap item whose completion is measurable, display the previous verified completion percentage, the current verified completion percentage beside its visual progress bar, and the signed percentage-point change during this reporting round (current minus previous). Compare the same item, scope, and acceptance-criteria denominator; otherwise explain that the estimates are not directly comparable instead of inventing a change. Show +0 pp when no verified change occurred and a negative pp when a verified correction decreases the estimate. If the previous or current percentage cannot be supported, mark it unknown and omit any unjustified numeric change or visual fill. Base reported gains on verified deliverables, not activity or repeated status messages.

**Predicate:** {"action":"report_with","object":"verified_previous_current_and_round_percentage_point_delta","target":"visual_project_progress_report"}

**Normative conditions:** ["Reporting measurable completion for an active project or unfinished roadmap item, including overall progress when shown.","Numerical interval delta is reported only when current and previous estimates have a verified, comparable acceptance-criteria basis."]

**Normative exceptions:** ["If previous or current percentages are unavailable, explicitly mark them unknown and omit unsupported numerical change or filled bars.","If scope or completion criteria changed, disclose non-comparability and do not compute a misleading delta.","Continue honoring the existing suppression of already-announced completed items in routine reports."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"unspecified","minimum":null,"maximum":null,"unit":"entities","window":null,"cue":"For each actively reported measurable project or unfinished roadmap item","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["Reporting measurable completion for an active project or unfinished roadmap item, including overall progress when shown.","Numerical interval delta is reported only when current and previous estimates have a verified, comparable acceptance-criteria basis."],"exclusions":["If previous or current percentages are unavailable, explicitly mark them unknown and omit unsupported numerical change or filled bars.","If scope or completion criteria changed, disclose non-comparability and do not compute a misleading delta.","Continue honoring the existing suppression of already-announced completed items in routine reports."]}

**Authority and overrides:** {"source":"Explicitly reviewed public-safe specimen normalization; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0069","NOM-0070"],"depends_on":[]}

**Categories:** {"domains":["project_management","agent_instructions","user_interfaces"],"topics":["progress_reporting","visual_progress_before_after_interval_delta"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0137.json

---

## NOM-0246 — Calibrate project completion against full-lifecycle remaining effort

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When establishing or updating a percentage-based project roadmap, inventory the full authorized objective and its acceptance criteria, including foreseeable difficult integration, edge cases, regression and runtime verification, migration, documentation, packaging, deployment, and release work. Assign defensible weights or effort estimates that account for relative complexity, dependencies, uncertainty, and the real remaining delivery burden; do not equate numerous easy tasks with a nearly completed project. Reassess estimates at material milestones and before claiming high completion figures such as 80 or 90 percent: a 90 percent estimate must be supported by evidence that the still-required work represents approximately the remaining tenth of the tracked baseline, not merely that 90 percent of task labels have been closed. If necessary original-scope work was omitted or underestimated, expose it, record the evidence and changed weights, and transparently rebaseline progress; preserve completed work, clarify incomparable percentages, and never fabricate progress, suppress required work, or weaken acceptance criteria to reach 100 percent. A difficult final phase is permitted when genuinely justified; identify its scope, risks, blockers, and verification conditions rather than promising an artificially smooth progress curve or inventing extra work.

**Predicate:** {"action":"calibrate","object":"project_completion_percentages_against_full_lifecycle_remaining_effort","target":"authorized_project_roadmap"}

**Normative conditions:** ["An authorized project uses completion percentages or weighted milestone progress.","A roadmap is created, adopted, audited, or its progress estimate is materially updated."]

**Normative exceptions:** ["When reliable effort estimates are unavailable, disclose uncertainty or unknown completion rather than inventing precise percentages.","A genuinely difficult final stage can last disproportionately long; the rule does not require linear or smooth progress.","Legitimate discoveries may require a transparent rebaseline, but do not authorize unrelated scope expansion."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"project baseline and material progress reviews","cue":"When establishing or updating a percentage-based project roadmap","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["An authorized project has a measurable objective and reports percent completion or weighted progress."],"exclusions":["No obligation to invent unsupported numerical baselines.","No guarantee of constant progress velocity or equal-duration percentage increments.","Never manufacture tasks, expand scope, or suppress newly discovered required work."]}

**Authority and overrides:** {"source":"Owner-directed public-safe general project-progress calibration guidance; opt-in application","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0043","NOM-0137"],"depends_on":["NOM-0064"]}

**Categories:** {"domains":["project_management","agent_instructions","software_development"],"topics":["roadmap_status","progress_reporting","acceptance_criteria"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0246.json
