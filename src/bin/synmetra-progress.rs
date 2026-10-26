//! Read-only evidence-ledger auditor; not a Google Analytics MCP server.

use serde_json::Value;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn array<'a>(v: &'a Value, key: &str) -> &'a [Value] {
    v[key].as_array().map(Vec::as_slice).unwrap_or(&[])
}

fn field<'a>(v: &'a Value, key: &str) -> &'a str {
    v[key].as_str().unwrap_or("")
}

fn round_two(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}

fn bar(value: f64) -> String {
    let filled = (value / 10.0).round().clamp(0.0, 10.0) as usize;
    format!(
        "{}{} {:.1}%",
        "█".repeat(filled),
        "░".repeat(10 - filled),
        value
    )
}

/// Refresh only the bar on a uniquely matched Markdown line.
fn update_bar(markdown: &mut String, marker: &str, progress: f64) -> Result<(), String> {
    let mut count = 0;
    let mut result = String::with_capacity(markdown.len());
    for line in markdown.split_inclusive('\n') {
        if !line.contains(marker) {
            result.push_str(line);
            continue;
        }
        count += 1;
        let start = line
            .find('`')
            .ok_or_else(|| format!("{marker}: missing opening bar"))?;
        let suffix = &line[start + 1..];
        let end = suffix
            .find('`')
            .ok_or_else(|| format!("{marker}: missing closing bar"))?;
        result.push_str(&line[..start + 1]);
        result.push_str(&bar(progress));
        result.push_str(&suffix[end..]);
    }
    if count != 1 {
        return Err(format!(
            "{marker}: expected one Markdown marker, found {count}"
        ));
    }
    *markdown = result;
    Ok(())
}

fn regenerate_bars(
    ledger: &Value,
    source_roadmap: &str,
    source_readme: &str,
) -> Result<(String, String), String> {
    let mut roadmap = source_roadmap.to_owned();
    let mut readme = source_readme.to_owned();
    let mut overall = Vec::new();
    for milestone in array(ledger, "milestones") {
        let mut milestones = Vec::new();
        for phase in array(milestone, "phases") {
            let mut phases = Vec::new();
            for task in array(phase, "tasks") {
                let checks = array(task, "checks");
                if checks.len() != 2 {
                    return Err(format!("{}: expected two checks", field(task, "id")));
                }
                let mut accepted = 0;
                for c in checks {
                    let passed = c["passed"] == true;
                    let evidenced = !field(c, "evidence").trim().is_empty();
                    if passed != evidenced {
                        return Err(format!("{}: evidence mismatch", field(task, "id")));
                    }
                    if passed {
                        accepted += 1;
                    }
                }
                let value = accepted as f64 * 50.0;
                if (task["progress"].as_f64().unwrap_or(-1.0) - value).abs() > 0.005 {
                    return Err(format!("{}: stored progress drift", field(task, "id")));
                }
                update_bar(
                    &mut roadmap,
                    &format!("**{} — {}** —", field(task, "id"), field(task, "name")),
                    value,
                )?;
                phases.push(value);
                milestones.push(value);
                overall.push(value);
            }
            update_bar(
                &mut roadmap,
                &format!("{} — {} —", field(phase, "id"), field(phase, "name")),
                round_two(phases.iter().sum::<f64>() / phases.len().max(1) as f64),
            )?;
        }
        let value = round_two(milestones.iter().sum::<f64>() / milestones.len().max(1) as f64);
        update_bar(
            &mut roadmap,
            &format!(
                "{} — {} —",
                field(milestone, "id"),
                field(milestone, "name")
            ),
            value,
        )?;
        update_bar(
            &mut readme,
            &format!("{} {} |", field(milestone, "id"), field(milestone, "name")),
            value,
        )?;
    }
    let value = round_two(overall.iter().sum::<f64>() / overall.len().max(1) as f64);
    update_bar(&mut roadmap, "**Overall:**", value)?;
    update_bar(&mut readme, "**Overall:**", value)?;
    Ok((roadmap, readme))
}

fn check(ok: bool, message: String, errors: &mut Vec<String>) {
    if !ok {
        errors.push(message);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR"))?;
    let ledger: Value = serde_json::from_str(&fs::read_to_string("docs/progress.json")?)?;
    let catalog: Value = serde_json::from_str(&fs::read_to_string("docs/tool-inventory.json")?)?;
    let sedna_schema: Value = serde_json::from_str(&fs::read_to_string(
        "spec/upstream/sednalabs-tool-schema.v1.json",
    )?)?;
    let mut roadmap = fs::read_to_string("docs/ROADMAP.md")?;
    let mut readme = fs::read_to_string("README.md")?;
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if !args.is_empty() {
        if args.as_slice() != ["--write-bars"] {
            return Err("Usage: synmetra-progress [--write-bars]".into());
        }
        let (new_roadmap, new_readme) = regenerate_bars(&ledger, &roadmap, &readme)?;
        fs::write("docs/ROADMAP.md", &new_roadmap)?;
        fs::write("README.md", &new_readme)?;
        roadmap = new_roadmap;
        readme = new_readme;
    }
    let issue_index = fs::read_to_string("docs/ISSUES.md")?;
    let tool_provenance = fs::read_to_string("docs/TOOL_PROVENANCE.md")?;
    let mut errors = Vec::<String>::new();
    let mut ids = HashSet::<String>::new();
    let mut names = HashSet::<String>::new();
    let mut implementation_evidence = Vec::<(String, String, f64)>::new();
    let mut all = Vec::<f64>::new();
    let mut nphase = 0;
    let mut accepted = 0;

    for (midx, milestone) in array(&ledger, "milestones").iter().enumerate() {
        let mid = field(milestone, "id");
        let mut mile_values = Vec::<f64>::new();
        for phase in array(milestone, "phases") {
            nphase += 1;
            let pid = field(phase, "id");
            let mut phase_values = Vec::<f64>::new();
            for task in array(phase, "tasks") {
                let id = field(task, "id");
                let name = field(task, "name");
                names.insert(name.to_owned());
                check(
                    ids.insert(id.to_owned()),
                    format!("Duplicate task {id}"),
                    &mut errors,
                );
                let checks = array(task, "checks");
                check(
                    checks.len() == 2,
                    format!("Task {id} must have two checks"),
                    &mut errors,
                );
                let mut passed = 0;
                for c in checks {
                    let done = c["passed"].as_bool() == Some(true);
                    let evidence = field(c, "evidence");
                    check(
                        !done || !evidence.trim().is_empty(),
                        format!("Task {id} accepted without evidence"),
                        &mut errors,
                    );
                    if done && !evidence.trim().is_empty() {
                        passed += 1;
                    }
                    check(
                        done || evidence.is_empty(),
                        format!("Task {id} has unaccepted evidence"),
                        &mut errors,
                    );
                }
                let actual = round_two(passed as f64 * 100.0 / checks.len().max(1) as f64);
                if let Some(tool) = name.strip_prefix("Implement ") {
                    implementation_evidence.push((tool.to_owned(), id.to_owned(), actual));
                }
                let stored = task["progress"].as_f64().unwrap_or(-1.0);
                check(
                    (stored - actual).abs() < 0.005,
                    format!("Task {id}: stored {stored} differs from {actual}"),
                    &mut errors,
                );
                check(
                    roadmap.contains(&format!("{id} — {name}")) && roadmap.contains(&bar(actual)),
                    format!("Missing roadmap task/bar for {id}"),
                    &mut errors,
                );
                if actual == 100.0 {
                    accepted += 1;
                }
                phase_values.push(actual);
                mile_values.push(actual);
                all.push(actual);
            }
            let pct =
                round_two(phase_values.iter().sum::<f64>() / phase_values.len().max(1) as f64);
            check(
                roadmap.contains(&format!("{pid} — {}", field(phase, "name")))
                    && roadmap.contains(&bar(pct)),
                format!("Phase {pid} roadmap drift"),
                &mut errors,
            );
        }
        let pct = round_two(mile_values.iter().sum::<f64>() / mile_values.len().max(1) as f64);
        check(
            readme.contains(&format!("{mid} {}", field(milestone, "name")))
                && readme.contains(&bar(pct)),
            format!("Milestone {mid} README drift"),
            &mut errors,
        );
        check(
            issue_index.contains(&format!("github.com/sguzman/synmetra/issues/{}", midx + 1)),
            format!("Missing issue for {mid}"),
            &mut errors,
        );
    }

    let official: HashSet<String> = array(&catalog["official"], "names")
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect();
    let sedna: HashSet<String> = array(&catalog["rust"], "names")
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect();
    let snapshot: HashSet<String> = array(&sedna_schema, "tools")
        .iter()
        .map(|tool| field(tool, "name").to_owned())
        .collect();
    check(
        field(&sedna_schema, "schema") == "mcp_tool_schema_snapshot",
        "Pinned SednaLabs schema fixture format drift".into(),
        &mut errors,
    );
    check(
        snapshot == sedna && array(&sedna_schema, "tools").len() == 35,
        "Pinned SednaLabs input schemas do not match the audited 35-tool union".into(),
        &mut errors,
    );
    let wanted: HashSet<String> = official.union(&sedna).cloned().collect();
    let listed: HashSet<String> = array(&catalog["union"], "tools")
        .iter()
        .map(|tool| field(tool, "name").to_owned())
        .collect();
    check(
        official.len() == 10,
        "Official tool count changed".into(),
        &mut errors,
    );
    check(
        sedna.len() == 35,
        "Rust tool count changed".into(),
        &mut errors,
    );
    check(
        wanted.len() == 36 && wanted == listed,
        "36-name feature union changed".into(),
        &mut errors,
    );
    check(
        array(&catalog["union"], "tools").len() == 36,
        "Duplicate feature entries".into(),
        &mut errors,
    );
    for tool in wanted {
        check(
            names.contains(&format!("Implement {tool}")),
            format!("Missing atomic task for {tool}"),
            &mut errors,
        );
        let marker = format!("| {tool} |");
        let references = tool_provenance
            .lines()
            .filter(|line| line.starts_with(&marker))
            .collect::<Vec<_>>();
        check(
            references.len() == 1,
            format!("Expected one provenance row for {tool}"),
            &mut errors,
        );
        if let (Some(row), Some((_, id, pct))) = (
            references.first(),
            implementation_evidence
                .iter()
                .find(|(name, _, _)| name == &tool),
        ) {
            check(
                row.contains(&format!("| {id} |"))
                    && row.contains(&format!(", {}% (", *pct as usize))
                    && row.contains("github.com/sguzman/synmetra/blob/"),
                format!("Tool provenance task, acceptance or Rust source drift: {tool}"),
                &mut errors,
            );
        }
    }
    check(
        ids.len() == 227 && nphase == 30 && array(&ledger, "milestones").len() == 11,
        "Roadmap task/phase/milestone inventory unexpectedly changed".into(),
        &mut errors,
    );
    let overall = round_two(all.iter().sum::<f64>() / all.len().max(1) as f64);
    check(
        readme.contains(&bar(overall)) && roadmap.contains(&bar(overall)),
        "Overall percentage drift".into(),
        &mut errors,
    );
    println!("📊 Synmetra overall: {}", bar(overall));
    println!(
        "✅ {accepted}/{} tasks accepted, {nphase} phases, 36 planned MCP tools",
        ids.len()
    );
    if errors.is_empty() {
        Ok(())
    } else {
        for error in &errors {
            eprintln!("❌ {error}");
        }
        Err(format!("{} integrity violations", errors.len()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounded_bars() {
        assert_eq!(bar(0.0), "░░░░░░░░░░ 0.0%");
        assert_eq!(bar(35.29), "████░░░░░░ 35.3%");
        assert_eq!(bar(100.0), "██████████ 100.0%");
    }

    #[test]
    fn bar_regeneration_is_idempotent_and_preserves_editorial_text() {
        let fixture = serde_json::json!({
            "milestones": [{
                "id": "M0",
                "name": "Baseline",
                "phases": [{
                    "id": "P00",
                    "name": "Fixtures",
                    "tasks": [{
                        "id": "T001",
                        "name": "Demo",
                        "progress": 50,
                        "checks": [
                            {"passed": true, "evidence": "fixture evidence"},
                            {"passed": false}
                        ]
                    }]
                }]
            }]
        });
        let roadmap = "**Overall:** `░░░░░░░░░░ 0.0%` · keep editorial text\n## M0 — Baseline — `░░░░░░░░░░ 0.0%`\n### P00 — Fixtures — `░░░░░░░░░░ 0.0%`\n- **T001 — Demo** — `░░░░░░░░░░ 0.0%`\n";
        let readme = "**Overall:** `░░░░░░░░░░ 0.0%`\n| M0 Baseline | `░░░░░░░░░░ 0.0%` | MVP |\n";
        let (r, d) = regenerate_bars(&fixture, roadmap, readme).unwrap();
        assert!(r.contains("**Overall:** `█████░░░░░ 50.0%` · keep editorial text"));
        assert!(d.contains("| M0 Baseline | `█████░░░░░ 50.0%` |"));
        assert_eq!(regenerate_bars(&fixture, &r, &d).unwrap(), (r, d));
        assert!(regenerate_bars(&fixture, "missing markers", readme).is_err());
        let mut invalid = fixture.clone();
        invalid["milestones"][0]["phases"][0]["tasks"][0]["checks"][0]["evidence"] =
            serde_json::Value::Null;
        assert!(regenerate_bars(&invalid, roadmap, readme).is_err());
    }

    #[test]
    fn numeric_rounding() {
        assert_eq!(round_two(100.0 / 3.0), 33.33);
    }
}
