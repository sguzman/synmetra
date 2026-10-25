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

fn check(ok: bool, message: String, errors: &mut Vec<String>) {
    if !ok {
        errors.push(message);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_current_dir(env!("CARGO_MANIFEST_DIR"))?;
    let ledger: Value = serde_json::from_str(&fs::read_to_string("docs/progress.json")?)?;
    let catalog: Value = serde_json::from_str(&fs::read_to_string("docs/tool-inventory.json")?)?;
    let roadmap = fs::read_to_string("docs/ROADMAP.md")?;
    let readme = fs::read_to_string("README.md")?;
    let issue_index = fs::read_to_string("docs/ISSUES.md")?;
    let mut errors = Vec::<String>::new();
    let mut ids = HashSet::<String>::new();
    let mut names = HashSet::<String>::new();
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
                check(ids.insert(id.to_owned()), format!("Duplicate task {id}"), &mut errors);
                let checks = array(task, "checks");
                check(checks.len() == 2, format!("Task {id} must have two checks"), &mut errors);
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
                let stored = task["progress"].as_f64().unwrap_or(-1.0);
                check(
                    (stored - actual).abs() < 0.005,
                    format!("Task {id}: stored {stored} differs from {actual}"),
                    &mut errors,
                );
                check(
                    roadmap.contains(&format!("{id} — {name}"))
                        && roadmap.contains(&bar(actual)),
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
            let pct = round_two(phase_values.iter().sum::<f64>() / phase_values.len().max(1) as f64);
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
    let wanted: HashSet<String> = official.union(&sedna).cloned().collect();
    let listed: HashSet<String> = array(&catalog["union"], "tools")
        .iter()
        .map(|tool| field(tool, "name").to_owned())
        .collect();
    check(official.len() == 10, "Official tool count changed".into(), &mut errors);
    check(sedna.len() == 35, "Rust tool count changed".into(), &mut errors);
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
    println!("✅ {accepted}/{} tasks accepted, {nphase} phases, 36 planned MCP tools", ids.len());
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
    fn numeric_rounding() {
        assert_eq!(round_two(100.0 / 3.0), 33.33);
    }
}
