//! Explicit Contract V1 tabular projection for GA4 reports.
//! A tool's default response stays provider-shaped unless the caller opts in.

use crate::google::ApiError;
use rmcp::schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{json, Map, Value};

const DEFAULT_ROWS: usize = 100;
const MAX_ROWS: usize = 1000;
const MAX_CELL_CHARS: usize = 2048;

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    Rows,
    Tuples,
    Scalar,
    Compact,
}

impl OutputMode {
    fn label(self) -> &'static str {
        match self {
            Self::Rows => "rows",
            Self::Tuples => "tuples",
            Self::Scalar => "scalar",
            Self::Compact => "compact",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Controls {
    pub output_mode: Option<OutputMode>,
    pub summary_only: bool,
    pub max_rows: Option<usize>,
    pub max_cell_chars: Option<usize>,
    pub cursor_supplied: bool,
}

impl Controls {
    pub fn active(self) -> bool {
        self.output_mode.is_some()
            || self.summary_only
            || self.max_rows.is_some()
            || self.max_cell_chars.is_some()
    }

    pub fn validate(self) -> Result<(), ApiError> {
        if self.cursor_supplied
            || self.max_rows.is_some_and(|n| n == 0 || n > MAX_ROWS)
            || self.max_cell_chars.is_some_and(|n| n == 0 || n > MAX_CELL_CHARS)
        {
            return Err(ApiError::InvalidRequest);
        }
        Ok(())
    }

    pub fn request_limit(self, existing: Option<u32>) -> Result<Option<u32>, ApiError> {
        self.validate()?;
        if !self.active() {
            return Ok(existing);
        }
        let local = self.max_rows.unwrap_or(DEFAULT_ROWS) as u32;
        Ok(Some(existing.map_or(local, |n| n.min(local))))
    }
}

pub struct Projection {
    pub data: Value,
    pub meta: Value,
}

fn columns(response: &Value, header: &str, fallback: &str) -> Result<Vec<Value>, ApiError> {
    let headers = match response.get(header) {
        Some(value) => value.as_array().ok_or(ApiError::InvalidResponse)?,
        None => return Ok(Vec::new()),
    };
    headers
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let name = item
                .get("name")
                .and_then(Value::as_str)
                .filter(|name| !name.is_empty())
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| format!("{fallback}{index}"));
            Ok(json!({
                "name": name,
                "logical_type": if header == "dimensionHeaders" { "string" } else { "number" },
                "nullable": true
            }))
        })
        .collect()
}

fn clip_value(value: Value, cap: Option<usize>, clips: &mut usize) -> Value {
    let Some(cap) = cap else { return value };
    if let Value::String(text) = &value {
        if text.chars().count() > cap {
            *clips += 1;
            return Value::String(text.chars().take(cap).collect());
        }
    }
    value
}

pub fn project(
    provider: &Value,
    controls: Controls,
    realtime: bool,
) -> Result<Projection, ApiError> {
    controls.validate()?;
    let provider = provider.as_object().ok_or(ApiError::InvalidResponse)?;
    let dimensions = columns(&Value::Object(provider.clone()), "dimensionHeaders", "dimension_")?;
    let metrics = columns(&Value::Object(provider.clone()), "metricHeaders", "metric_")?;
    let mut names = Vec::new();
    names.extend(dimensions.iter().chain(metrics.iter()).filter_map(|column| {
        column.get("name").and_then(Value::as_str).map(ToOwned::to_owned)
    }));
    let mut columns = dimensions;
    columns.extend(metrics);
    let raw_rows = provider.get("rows").and_then(Value::as_array);
    if provider.contains_key("rows") && raw_rows.is_none() {
        return Err(ApiError::InvalidResponse);
    }
    let raw_rows = raw_rows.map(Vec::as_slice).unwrap_or(&[]);
    if !raw_rows.is_empty() && names.is_empty() {
        return Err(ApiError::InvalidResponse);
    }
    let row_count_total = if let Some(value) = provider.get("rowCount") {
        let count = value.as_u64().ok_or(ApiError::InvalidResponse)?;
        usize::try_from(count).map_err(|_| ApiError::InvalidResponse)?
    } else {
        raw_rows.len()
    };
    if row_count_total < raw_rows.len() {
        return Err(ApiError::InvalidResponse);
    }
    let cap = controls.max_rows.unwrap_or(DEFAULT_ROWS);
    let mut rows = Vec::new();
    let mut tuples = Vec::new();
    let mut clipped = 0usize;
    for row in raw_rows.iter().take(cap) {
        let dimension_values = row
            .get("dimensionValues")
            .and_then(Value::as_array)
            .ok_or(ApiError::InvalidResponse)?;
        let metric_values = row
            .get("metricValues")
            .and_then(Value::as_array)
            .ok_or(ApiError::InvalidResponse)?;
        if dimension_values.len() + metric_values.len() != names.len() {
            return Err(ApiError::InvalidResponse);
        }
        let mut object = Map::new();
        let mut tuple = Vec::new();
        for (name, cell) in names
            .iter()
            .zip(dimension_values.iter().chain(metric_values.iter()))
        {
            let value = cell.get("value").cloned().ok_or(ApiError::InvalidResponse)?;
            let clipped_value = clip_value(value, controls.max_cell_chars, &mut clipped);
            object.insert(name.clone(), clipped_value.clone());
            tuple.push(clipped_value);
        }
        rows.push(Value::Object(object));
        tuples.push(Value::Array(tuple));
    }
    let returned = rows.len();
    let truncated = row_count_total > returned;
    let mode = controls.output_mode.unwrap_or(OutputMode::Rows);
    let data = if controls.summary_only {
        Value::Null
    } else {
        match mode {
            OutputMode::Rows => Value::Array(rows),
            OutputMode::Tuples => Value::Array(tuples.clone()),
            OutputMode::Scalar => tuples
                .first()
                .and_then(Value::as_array)
                .and_then(|values| values.first())
                .cloned()
                .unwrap_or(Value::Null),
            OutputMode::Compact => json!({
                "columns": names,
                "tuples": tuples,
                "row_count": returned
            }),
        }
    };
    let mut ga4 = Map::new();
    for field in [
        "metadata", "propertyQuota", "quota", "totals", "maximums",
        "minimums", "aggregates", "warnings",
    ] {
        if let Some(value) = provider.get(field) {
            ga4.insert(field.to_owned(), value.clone());
        }
    }
    let mut meta = json!({
        "elapsed_ms": 0,
        "elapsed_ms_measured": false,
        "output_mode": mode.label(),
        "summary_only": controls.summary_only,
        "row_count_total": row_count_total,
        "row_count_returned": returned,
        "truncated": truncated,
        "next_cursor": null,
        "query_hash": null,
        "columns": columns,
        "ga4": ga4
    });
    if let Some(cell_cap) = controls.max_cell_chars {
        meta["cell_clipping"] = json!({
            "enabled": true,
            "max_cell_chars": cell_cap,
            "clipped_cells": clipped,
            "applied": clipped > 0
        });
    }
    if truncated {
        meta["query_hints"] = json!([if realtime {
            "Realtime reports have no offset cursor; narrow filters or increase limit."
        } else {
            "Cursor projection is not enabled; use provider-shaped offset/limit for pagination."
        }]);
    }
    Ok(Projection { data, meta })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Value {
        json!({
            "dimensionHeaders": [{"name":"country"}],
            "metricHeaders": [{"name":"activeUsers","type":"TYPE_INTEGER"}],
            "rows": [
                {"dimensionValues":[{"value":"México"}],"metricValues":[{"value":"17"}]},
                {"dimensionValues":[{"value":"United States"}],"metricValues":[{"value":"23"}]}
            ],
            "rowCount": 3,
            "propertyQuota": {"concurrentRequests":{"remaining":9}},
            "warnings": ["synthetic upstream warning"]
        })
    }

    fn controls() -> Controls {
        Controls {
            output_mode: Some(OutputMode::Rows),
            summary_only: false,
            max_rows: Some(1),
            max_cell_chars: None,
            cursor_supplied: false,
        }
    }

    #[test]
    fn projections_keep_true_counts_and_upstream_quota() {
        let rows = project(&fixture(), controls(), false).unwrap();
        assert_eq!(rows.data[0]["country"], "México");
        assert_eq!(rows.meta["row_count_total"], 3);
        assert_eq!(rows.meta["row_count_returned"], 1);
        assert_eq!(rows.meta["truncated"], true);
        assert!(rows.meta["next_cursor"].is_null());
        assert_eq!(rows.meta["ga4"]["propertyQuota"]["concurrentRequests"]["remaining"], 9);
        assert_eq!(rows.meta["ga4"]["warnings"][0], "synthetic upstream warning");

        let mut tuples = controls();
        tuples.output_mode = Some(OutputMode::Tuples);
        tuples.max_rows = Some(2);
        assert_eq!(project(&fixture(), tuples, false).unwrap().data[1][1], "23");

        tuples.output_mode = Some(OutputMode::Scalar);
        assert_eq!(project(&fixture(), tuples, false).unwrap().data, "México");

        tuples.output_mode = Some(OutputMode::Compact);
        assert_eq!(project(&fixture(), tuples, false).unwrap().data["row_count"], 2);
    }

    #[test]
    fn summary_and_unicode_clipping_preserve_counts_without_row_payload() {
        let mut options = controls();
        options.max_cell_chars = Some(3);
        let projected = project(&fixture(), options, true).unwrap();
        assert_eq!(projected.data[0]["country"], "Méx");
        assert_eq!(projected.meta["cell_clipping"]["clipped_cells"], 1);
        options.summary_only = true;
        let summary = project(&fixture(), options, true).unwrap();
        assert!(summary.data.is_null());
        assert_eq!(summary.meta["row_count_returned"], 1);
        assert!(summary.meta["next_cursor"].is_null());
    }

    #[test]
    fn rejects_invalid_controls_and_malformed_provider_shapes() {
        let mut options = controls();
        options.max_rows = Some(0);
        assert_eq!(options.validate(), Err(ApiError::InvalidRequest));
        options.max_rows = Some(1);
        options.cursor_supplied = true;
        assert_eq!(options.validate(), Err(ApiError::InvalidRequest));
        options.cursor_supplied = false;
        assert!(project(&json!({"rows":[{}],"rowCount":1}), options, false).is_err());
        assert!(project(&json!({"rows":[],"rowCount":-1}), options, false).is_err());
    }
}
