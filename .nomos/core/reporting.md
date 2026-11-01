# Nomos / core/reporting.md

These rule records are reference material. Apply only when their original scope and conditions are met. Keep exceptions and preferences intact.

## NOM-0065 — Evidence-based state labels

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Distinguish written, staged, committed, compiled, tested, deployed and visually verified states.

**Predicate:** {"action":"practice","object":"evidence_based_state_labels","target":"authorized_project_work"}

**Normative conditions:** ["When describing development status","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When describing development status"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["qa_verification","software_development"],"topics":["evidence_based_status"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0065.json

---

## NOM-0067 — Preserve unfinished work state

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

Keep blockers and unfinished acceptance criteria visible rather than marking incomplete work as complete.

**Predicate:** {"action":"practice","object":"preserve_unfinished_work_state","target":"authorized_project_work"}

**Normative conditions:** ["When the requested result is not fully verified","Applies only when expressly selected for a matching project or authorized development workflow.","This rule does not itself authorize changes to other repositories, publication or destructive work."]

**Normative exceptions:** []

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["When the requested result is not fully verified"],"exclusions":["Projects or operations outside authorized task scope"],"project_types":["project"]}

**Authority and overrides:** {"source":"Published opt-in hygiene rule; no project-wide activation or execution authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["maintenance","qa_verification","software_development"],"topics":["unfinished_work_state"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0067.json

---

## NOM-0071 — Announce first attainment of 100 percent exactly once

**Version:** 1.2.0 · **Kind:** obligation · **Authority:** general_guidance

When a project, phase, milestone, task or feature first reaches 100 percent, show its completion and percentage once in the reporting conversation.

**Predicate:** {"action":"announce","object":"first_completion_at_100_percent","target":"work_item_progress_report"}

**Normative conditions:** ["The project or roadmap item reaches and is first reported at 100 percent."]

**Normative exceptions:** ["Historical completion events should not be reannounced as new completions."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":1,"maximum":1,"unit":"occurrences","window":"first report after item completion","cue":"once","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["The project or roadmap item reaches and is first reported at 100 percent."],"exclusions":["Historical completion events should not be reannounced as new completions."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos catalog guidance; opt-in and subject to task-level authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["project_management"],"topics":["completion_reporting"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0071.json

---

## NOM-0072 — Suppress completed items from subsequent routine progress reports

**Version:** 1.2.0 · **Kind:** prohibition · **Authority:** general_guidance

After the 100 percent completion of a project, phase, milestone, task or feature has been reported once, omit it from later routine progress reports while preserving the underlying roadmap and completion history.

**Predicate:** {"action":"repeat","object":"already_announced_complete_item","target":"later_routine_progress_report"}

**Normative conditions:** ["An item's 100 percent completion was already announced in an earlier report."]

**Normative exceptions:** ["This does not erase archived completion data; explicitly reopened work may be reported as incomplete."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"never","minimum":0,"maximum":0,"unit":"occurrences","window":"later routine progress reports","cue":"omit it from later routine progress reports","basis":"reviewed_inference"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"preconditions":["An item's 100 percent completion was already announced in an earlier report."],"exclusions":["This does not erase archived completion data; explicitly reopened work may be reported as incomplete."],"project_types":["project"]}

**Authority and overrides:** {"source":"Reviewed Nomos catalog guidance; opt-in and subject to task-level authorization","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":["NOM-0071"]}

**Categories:** {"domains":["project_management"],"topics":["completion_reporting"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0072.json

---

## NOM-0135 — End completed-work reports with explicit user action requirements

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

When reporting a completed work session or development deliverable, end the summary with a clearly separated user-action section. State each approval, decision, choice or input genuinely needed to proceed, identifying what it unblocks. If no user action is necessary, explicitly state that nothing is required. Keep optional follow-up suggestions distinct from mandatory action, and do not imply that automatic work authorization follows from reporting a possible next step.

**Predicate:** {"action":"conclude_with","object":"explicit_required_user_action_or_none","target":"completed_work_summary"}

**Normative conditions:** ["When reporting a completed development work session or deliverable."]

**Normative exceptions:** ["Not required for casual answers; never invent mandatory approvals or conflate optional follow-ups with blockers."]

**Normative bounds:** {"quantifier":"unspecified","interpretation":"unspecified","frequency":"unspecified","minimum":null,"maximum":null,"unit":"unspecified","window":null,"cue":null,"basis":"unresolved"}

**Project and context scope:** {"axes":{"channel":["any"],"visibility":["any"],"artifact_kind":["any"],"actor":["any"]},"project_types":["project"],"preconditions":["When reporting a completed development work session or deliverable."],"exclusions":["Not required for casual answers; never invent mandatory approvals or conflate optional follow-ups with blockers."]}

**Authority and overrides:** {"source":"Explicitly reviewed public-safe specimen normalization; opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":["NOM-0066","NOM-0067"],"depends_on":[]}

**Categories:** {"domains":["project_management","agent_instructions"],"topics":["end_of_work_action_requirement_reporting"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0135.json

---

## NOM-0244 — Conserve conversational context during agent development

**Version:** 1.0.0 · **Kind:** obligation · **Authority:** general_guidance

During ongoing agent-assisted project development, treat conversational context and tool-result size as finite resources. Retrieve only the relevant files, lines, commit diffs and diagnostics needed for the current decision; expand scope when necessary. Filter or summarize excessive results before returning them to the conversation, and avoid redundant full-file, log, history, roadmap or instruction dumps. Maintain a concise, recoverable project checkpoint covering the objective, accepted decisions, verified progress, current commit, blockers and next actionable work, updating it when project state materially changes. Proactively preserve a handoff when context exhaustion becomes a credible risk where feasible. Do not claim to know an exact remaining context budget or assume automatic compaction. Never omit necessary evidence, testing, requirements or truthful reporting, and do not end useful authorized work merely to minimize tokens.

**Predicate:** {"action":"conserve","object":"conversation_context_through_targeted_retrieval_and_durable_checkpoints","target":"agent_development_workflow"}

**Normative conditions:** ["During owner-authorized agent-assisted project work with a finite conversation context.","Apply targeted retrieval and bounded output whenever it remains sufficient for sound engineering decisions."]

**Normative exceptions:** ["Expand retrieval and include necessary output when correctness, debugging, verification or recovery requires it.","Do not invent context measurements or prematurely stop legitimate work as a token-saving measure."]

**Normative bounds:** {"quantifier":"all","interpretation":"universal","frequency":"always","minimum":null,"maximum":null,"unit":"entities","window":"during each applicable agent development session","cue":"During ongoing agent-assisted project development","basis":"explicit_text"}

**Project and context scope:** {"axes":{"channel":["chat"],"visibility":["any"],"artifact_kind":["instructions"],"actor":["ai_agent"]},"project_types":["project"],"preconditions":["Owner-authorized ongoing agent-assisted development and finite conversation context."],"exclusions":["Necessary broad retrieval, diagnostic evidence, and QA remain allowed.","No arbitrary work-duration or task-count limit is imposed."]}

**Authority and overrides:** {"source":"Explicit owner-approved public-safe generalized instruction, opt-in guidance","tier":"general_guidance","priority":0,"override_policy":"explicit_resolution"}

**Relations:** {"conflicts_with":[],"exceptions_to":[],"refines":[],"depends_on":[]}

**Categories:** {"domains":["agent_instructions","software_development","project_management"],"topics":["development_hygiene","unfinished_work_state"],"audiences":["ai_agents","human_authors","automations"]}

**Canonical source:** https://github.com/sguzman/nomos/blob/main/rules/NOM-0244.json
