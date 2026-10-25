//! Explicit Contract V1 tabular projection for GA4 reports.
//! A tool's default response stays provider-shaped unless the caller opts in.

use crate::google::ApiError;
use rmcp::schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

const DEFAULT_ROWS: usize = 100;
const MAX_ROWS: usize = 1000;
const MAX_CELL_CHARS: usize = 2048;
const MAX_CURSOR_OFFSET: u64 = 10_000_000;

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
            || self.cursor_supplied
            || self.summary_only
            || self.max_rows.is_some()
            || self.max_cell_chars.is_some()
    }

    pub fn validate(self) -> Result<(), ApiError> {
        if self.max_rows.is_some_and(|n| n == 0 || n > MAX_ROWS)
            || self
                .max_cell_chars
                .is_some_and(|n| n == 0 || n > MAX_CELL_CHARS)
        {
            return Err(ApiError::InvalidRequest);
        }
        Ok(())
    }

    pub fn request_limit(self, existing: Option<u64>) -> Result<Option<u64>, ApiError> {
        self.validate()?;
        if existing.is_some_and(|n| n == 0 || n > 250_000) {
            return Err(ApiError::InvalidRequest);
        }
        if !self.active() {
            return Ok(existing);
        }
        let local = self.max_rows.unwrap_or(DEFAULT_ROWS) as u64;
        Ok(Some(existing.map_or(local, |n| n.min(local))))
    }
}

pub struct Projection {
    pub data: Value,
    pub meta: Value,
}

pub struct Pagination {
    pub query_hash: String,
    pub offset: u64,
}

/// A continuation binds to the normalized property and query, never a row from a
/// different filter/order. Paging fields are excluded so page sizes may change.
pub fn prepare_historical(
    request: &mut Value,
    property_id: &str,
    cursor: Option<&str>,
    explicit_offset: Option<u64>,
) -> Result<Pagination, ApiError> {
    let property = crate::google::property_name(property_id)?;
    prepare_resource(request, &property, cursor, explicit_offset)
}

/// Account and property data-access reports use the same bounded cursor format
/// with disjoint canonical resource prefixes in the query fingerprint.
pub fn prepare_access(
    request: &mut Value,
    entity_id: &str,
    account: bool,
    cursor: Option<&str>,
    explicit_offset: Option<u64>,
) -> Result<Pagination, ApiError> {
    let entity = if account {
        crate::google::account_name(entity_id)?
    } else {
        crate::google::property_name(entity_id)?
    };
    prepare_resource(request, &entity, cursor, explicit_offset)
}

fn prepare_resource(
    request: &mut Value,
    property: &str,
    cursor: Option<&str>,
    explicit_offset: Option<u64>,
) -> Result<Pagination, ApiError> {
    let object = request.as_object_mut().ok_or(ApiError::InvalidRequest)?;
    let mut query = object.clone();
    query.remove("offset");
    query.remove("limit");
    let canonical = serde_json::to_vec(&json!({"property": property, "request": query}))
        .map_err(|_| ApiError::InvalidRequest)?;
    let query_hash = format!("{:x}", Sha256::digest(canonical));
    let offset = if let Some(raw) = cursor {
        if explicit_offset.is_some() || raw.len() > 100 {
            return Err(ApiError::InvalidRequest);
        }
        let parts = raw.split(':').collect::<Vec<_>>();
        if parts.len() != 3
            || parts[0] != "v1"
            || parts[1] != query_hash
            || parts[2].is_empty()
            || !parts[2].bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(ApiError::InvalidRequest);
        }
        parts[2]
            .parse::<u64>()
            .map_err(|_| ApiError::InvalidRequest)?
    } else {
        explicit_offset.unwrap_or(0)
    };
    if offset > MAX_CURSOR_OFFSET {
        return Err(ApiError::InvalidRequest);
    }
    if offset > 0 || explicit_offset.is_some() {
        object.insert("offset".to_owned(), json!(offset.to_string()));
    } else {
        object.remove("offset");
    }
    Ok(Pagination { query_hash, offset })
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
            if !item.is_object() {
                return Err(ApiError::InvalidResponse);
            }
            let access_field = if header == "dimensionHeaders" {
                "dimensionName"
            } else {
                "metricName"
            };
            let name = item
                .get("name")
                .or_else(|| item.get(access_field))
                .and_then(Value::as_str)
                .filter(|name| !name.is_empty())
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| format!("{fallback}{index}"));
            let mut column = json!({
                "name": name,
                "logical_type": if header == "dimensionHeaders" { "string" } else { "number" },
                "nullable": true
            });
            if header == "metricHeaders" {
                if let Some(kind) = item.get("type") {
                    let kind = kind.as_str().ok_or(ApiError::InvalidResponse)?;
                    column["ga4_type"] = json!(kind);
                }
            }
            Ok(column)
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
    pagination: Option<&Pagination>,
) -> Result<Projection, ApiError> {
    controls.validate()?;
    let provider = provider.as_object().ok_or(ApiError::InvalidResponse)?;
    let dimensions = columns(
        &Value::Object(provider.clone()),
        "dimensionHeaders",
        "dimension_",
    )?;
    let metrics = columns(&Value::Object(provider.clone()), "metricHeaders", "metric_")?;
    let dimension_count = dimensions.len();
    let mut names = Vec::new();
    names.extend(
        dimensions
            .iter()
            .chain(metrics.iter())
            .filter_map(|column| {
                column
                    .get("name")
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned)
            }),
    );
    if names.iter().collect::<HashSet<_>>().len() != names.len() {
        return Err(ApiError::InvalidResponse);
    }
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
    let row_count_total_known = provider.contains_key("rowCount");
    let row_count_total = if let Some(value) = provider.get("rowCount") {
        let count = value
            .as_u64()
            .or_else(|| value.as_str().and_then(|text| text.parse::<u64>().ok()))
            .ok_or(ApiError::InvalidResponse)?;
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
        let dimension_values: &[Value] = match row.get("dimensionValues") {
            Some(values) => values
                .as_array()
                .map(Vec::as_slice)
                .ok_or(ApiError::InvalidResponse)?,
            None => &[],
        };
        let metric_values: &[Value] = match row.get("metricValues") {
            Some(values) => values
                .as_array()
                .map(Vec::as_slice)
                .ok_or(ApiError::InvalidResponse)?,
            None => &[],
        };
        if dimension_values.len() + metric_values.len() != names.len() {
            return Err(ApiError::InvalidResponse);
        }
        let mut object = Map::new();
        let mut tuple = Vec::new();
        for (index, (name, cell)) in names
            .iter()
            .zip(dimension_values.iter().chain(metric_values.iter()))
            .enumerate()
        {
            let value = cell
                .get("value")
                .cloned()
                .ok_or(ApiError::InvalidResponse)?;
            // Metric values are decimal/int64 strings in GA4. Shortening them
            // would silently change their numeric meaning and precision.
            let clipped_value = if index < dimension_count {
                clip_value(value, controls.max_cell_chars, &mut clipped)
            } else {
                value
            };
            object.insert(name.clone(), clipped_value.clone());
            tuple.push(clipped_value);
        }
        rows.push(Value::Object(object));
        tuples.push(Value::Array(tuple));
    }
    let returned = rows.len();
    // A provider total counts from offset zero, not from the current page.
    // Never report a final historical page as truncated merely because prior
    // pages accounted for most of the provider's rows.
    let truncated = if let Some(page) = pagination {
        if row_count_total_known {
            page.offset
                .checked_add(returned as u64)
                .is_some_and(|end| end < row_count_total as u64)
        } else {
            raw_rows.len() > returned
        }
    } else {
        row_count_total > returned
    };
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
        "metadata",
        "pivotHeaders",
        "propertyQuota",
        "quota",
        "totals",
        "maximums",
        "minimums",
        "aggregates",
        "warnings",
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
        "row_count_total_known": row_count_total_known,
        "row_count_returned": returned,
        "truncated": truncated,
        "next_cursor": null,
        "query_hash": null,
        "columns": columns,
        "ga4": ga4
    });
    if !realtime && row_count_total_known {
        if let Some(page) = pagination {
            meta["query_hash"] = json!(page.query_hash);
            if let Some(next) = page.offset.checked_add(returned as u64) {
                if returned > 0 && next < row_count_total as u64 && next <= MAX_CURSOR_OFFSET {
                    meta["next_cursor"] = json!(format!("v1:{}:{next}", page.query_hash));
                }
            }
        }
    }
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
            "Continue with meta.next_cursor if present, or provider-shaped offset/limit."
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
        let rows = project(&fixture(), controls(), false, None).unwrap();
        assert_eq!(rows.data[0]["country"], "México");
        assert_eq!(rows.meta["row_count_total"], 3);
        assert_eq!(rows.meta["row_count_returned"], 1);
        assert_eq!(rows.meta["truncated"], true);
        assert!(rows.meta["next_cursor"].is_null());
        assert_eq!(
            rows.meta["ga4"]["propertyQuota"]["concurrentRequests"]["remaining"],
            9
        );
        assert_eq!(
            rows.meta["ga4"]["warnings"][0],
            "synthetic upstream warning"
        );

        let mut tuples = controls();
        tuples.output_mode = Some(OutputMode::Tuples);
        tuples.max_rows = Some(2);
        assert_eq!(
            project(&fixture(), tuples, false, None).unwrap().data[1][1],
            "23"
        );

        tuples.output_mode = Some(OutputMode::Scalar);
        assert_eq!(
            project(&fixture(), tuples, false, None).unwrap().data,
            "México"
        );

        tuples.output_mode = Some(OutputMode::Compact);
        assert_eq!(
            project(&fixture(), tuples, false, None).unwrap().data["row_count"],
            2
        );
    }

    #[test]
    fn bounded_projection_limits_are_explicit_in_all_tabular_modes() {
        let provider = json!({
            "dimensionHeaders":[{"name":"country"}],
            "metricHeaders":[{"name":"activeUsers","type":"TYPE_INTEGER"}],
            "rows":[
                {"dimensionValues":[{"value":"México"}],"metricValues":[{"value":"125"}]},
                {"dimensionValues":[{"value":"Canada"}],"metricValues":[{"value":"12"}]},
                {"dimensionValues":[{"value":"France"}],"metricValues":[{"value":"3"}]}
            ],
            "rowCount":7
        });
        for mode in [
            OutputMode::Rows,
            OutputMode::Tuples,
            OutputMode::Scalar,
            OutputMode::Compact,
        ] {
            for summary_only in [false, true] {
                let bounded = Controls {
                    output_mode: Some(mode),
                    summary_only,
                    max_rows: Some(2),
                    max_cell_chars: Some(2),
                    cursor_supplied: false,
                };
                assert_eq!(bounded.request_limit(Some(250_000)).unwrap(), Some(2));
                let result = project(&provider, bounded, false, None).unwrap();
                assert_eq!(result.meta["row_count_total"], 7);
                assert_eq!(result.meta["row_count_returned"], 2);
                assert_eq!(result.meta["truncated"], true);
                assert_eq!(result.meta["cell_clipping"]["clipped_cells"], 2);
                assert_eq!(result.meta["cell_clipping"]["applied"], true);
                assert_eq!(result.meta["cell_clipping"]["max_cell_chars"], 2);
                if summary_only {
                    assert!(result.data.is_null());
                } else {
                    assert!(!result.data.is_null());
                }
            }
        }
        assert_eq!(provider["rows"][0]["dimensionValues"][0]["value"], "México");
        assert_eq!(provider["rows"][0]["metricValues"][0]["value"], "125");

        let mut invalid = controls();
        for rows in [0, MAX_ROWS + 1] {
            invalid.max_rows = Some(rows);
            assert_eq!(invalid.validate(), Err(ApiError::InvalidRequest));
        }
        invalid.max_rows = Some(1);
        for chars in [0, MAX_CELL_CHARS + 1] {
            invalid.max_cell_chars = Some(chars);
            assert_eq!(invalid.validate(), Err(ApiError::InvalidRequest));
        }
    }

    #[test]
    fn numeric_metric_precision_and_currency_metadata_survive_text_clipping() {
        let response = json!({
            "dimensionHeaders":[{"name":"country"}],
            "metricHeaders":[{"name":"totalRevenue","type":"TYPE_CURRENCY"}],
            "rows":[
                {
                    "dimensionValues":[{"value":"México, Estados Unidos"}],
                    "metricValues":[{"value":"12345678901234567890.123456789"}]
                }
            ],
            "rowCount":1,
            "metadata":{"currencyCode":"MXN","timeZone":"America/Mexico_City"}
        });
        let options = Controls {
            output_mode: Some(OutputMode::Rows),
            summary_only: false,
            max_rows: Some(1),
            max_cell_chars: Some(3),
            cursor_supplied: false,
        };
        let output = project(&response, options, false, None).unwrap();
        assert_eq!(output.data[0]["country"], "Méx");
        assert_eq!(
            output.data[0]["totalRevenue"],
            "12345678901234567890.123456789"
        );
        assert_eq!(output.meta["cell_clipping"]["clipped_cells"], 1);
        assert_eq!(output.meta["ga4"]["metadata"], response["metadata"]);
        assert_eq!(output.meta["columns"][1]["logical_type"], "number");
        assert_eq!(output.meta["columns"][1]["ga4_type"], "TYPE_CURRENCY");

        let mut tuples = options;
        tuples.output_mode = Some(OutputMode::Tuples);
        let tuple = project(&response, tuples, false, None).unwrap();
        assert_eq!(tuple.data[0][1], "12345678901234567890.123456789");
    }

    #[test]
    fn historical_last_page_is_not_marked_truncated_and_accepts_string_totals() {
        let response = json!({
            "dimensionHeaders":[{"name":"country"}],
            "metricHeaders":[{"name":"activeUsers"}],
            "rows":[
                {"dimensionValues":[{"value":"Canada"}],"metricValues":[{"value":"2"}]}
            ],
            "rowCount":"3"
        });
        let last = Pagination {
            query_hash: "fixture-query".into(),
            offset: 2,
        };
        let projected = project(&response, controls(), false, Some(&last)).unwrap();
        assert_eq!(projected.data[0]["country"], "Canada");
        assert_eq!(projected.meta["row_count_total"], 3);
        assert_eq!(projected.meta["row_count_total_known"], true);
        assert_eq!(projected.meta["row_count_returned"], 1);
        assert_eq!(projected.meta["truncated"], false);
        assert!(projected.meta["next_cursor"].is_null());

        let middle = Pagination {
            query_hash: "fixture-query".into(),
            offset: 1,
        };
        let result = project(&response, controls(), false, Some(&middle)).unwrap();
        assert_eq!(result.meta["truncated"], true);
        assert_eq!(result.meta["next_cursor"], "v1:fixture-query:2");

        let mut unknown = response;
        unknown.as_object_mut().unwrap().remove("rowCount");
        let no_total = project(&unknown, controls(), false, Some(&middle)).unwrap();
        assert_eq!(no_total.meta["row_count_total_known"], false);
        assert_eq!(no_total.meta["truncated"], false);
        assert!(no_total.meta["next_cursor"].is_null());
    }

    #[test]
    fn provider_aggregate_rows_survive_all_projection_modes_without_recomputation() {
        let original = json!({
            "dimensionHeaders": [{"name":"country"}],
            "metricHeaders": [{"name":"purchaseRevenue","type":"TYPE_CURRENCY"}],
            "rows": [
                {"dimensionValues":[{"value":"Mexico"}],"metricValues":[{"value":"12.75"}]}
            ],
            "rowCount": 18,
            "totals": [{"metricValues":[{"value":"125.50"}]}],
            "minimums": [{"metricValues":[{"value":"0.00"}]}],
            "maximums": [{"metricValues":[{"value":"105.75"}]}],
            "aggregates": [{"type":"TOTAL","value":"125.50"}],
            "propertyQuota": {"concurrentRequests":{"remaining":7}}
        });
        for output_mode in [
            OutputMode::Rows,
            OutputMode::Tuples,
            OutputMode::Scalar,
            OutputMode::Compact,
        ] {
            for summary_only in [false, true] {
                let options = Controls {
                    output_mode: Some(output_mode),
                    summary_only,
                    max_rows: Some(1),
                    max_cell_chars: None,
                    cursor_supplied: false,
                };
                let result = project(&original, options, false, None).unwrap();
                assert_eq!(result.meta["ga4"]["totals"], original["totals"]);
                assert_eq!(result.meta["ga4"]["minimums"], original["minimums"]);
                assert_eq!(result.meta["ga4"]["maximums"], original["maximums"]);
                assert_eq!(result.meta["ga4"]["aggregates"], original["aggregates"]);
                assert_eq!(
                    result.meta["ga4"]["propertyQuota"],
                    original["propertyQuota"]
                );
                assert_eq!(result.meta["row_count_total"], 18);
                assert_eq!(result.meta["row_count_total_known"], true);
                assert_eq!(result.meta["row_count_returned"], 1);
                assert_eq!(result.meta["truncated"], true);
                if summary_only {
                    assert!(result.data.is_null());
                } else {
                    assert!(!result.data.is_null());
                }
            }
        }
        // Projection must not mutate the provider response or fold aggregates
        // into ordinary rows where they would distort counts or double count.
        assert_eq!(original["rows"].as_array().unwrap().len(), 1);
        assert_eq!(original["totals"][0]["metricValues"][0]["value"], "125.50");

        let no_aggregate = json!({
            "metricHeaders":[{"name":"activeUsers"}],
            "rows":[{"metricValues":[{"value":"3"}]}]
        });
        let output = project(&no_aggregate, controls(), false, None).unwrap();
        assert!(output.meta["ga4"].as_object().unwrap().is_empty());
        assert_eq!(output.meta["row_count_total_known"], false);
    }

    #[test]
    fn summary_and_unicode_clipping_preserve_counts_without_row_payload() {
        let mut options = controls();
        options.max_cell_chars = Some(3);
        let projected = project(&fixture(), options, true, None).unwrap();
        assert_eq!(projected.data[0]["country"], "Méx");
        assert_eq!(projected.meta["cell_clipping"]["clipped_cells"], 1);
        options.summary_only = true;
        let summary = project(&fixture(), options, true, None).unwrap();
        assert!(summary.data.is_null());
        assert_eq!(summary.meta["row_count_returned"], 1);
        assert!(summary.meta["next_cursor"].is_null());
    }

    #[test]
    fn unknown_total_is_marked_unknown_and_metric_only_reports_are_valid() {
        let response = json!({
            "metricHeaders":[{"name":"activeUsers"}],
            "rows":[{"metricValues":[{"value":"12"}]}]
        });
        let projected = project(&response, controls(), false, None).unwrap();
        assert_eq!(projected.data[0]["activeUsers"], "12");
        assert_eq!(projected.meta["row_count_total"], 1);
        assert_eq!(projected.meta["row_count_total_known"], false);

        let duplicate = json!({
            "dimensionHeaders":[{"name":"activeUsers"}],
            "metricHeaders":[{"name":"activeUsers"}],
            "rows":[{"dimensionValues":[{"value":"US"}],"metricValues":[{"value":"12"}]}]
        });
        assert!(project(&duplicate, controls(), false, None).is_err());
    }

    #[test]
    fn admin_access_headers_and_resource_boundaries_have_distinct_cursors() {
        let response = json!({
            "dimensionHeaders":[{"dimensionName":"userEmail"}],
            "metricHeaders":[{"metricName":"accessCount"}],
            "rows":[{"dimensionValues":[{"value":"private@example.test"}],
                     "metricValues":[{"value":"5"}]}],
            "rowCount":2
        });
        let mut request = json!({
            "dateRanges":[{"startDate":"yesterday","endDate":"today"}],
            "dimensions":[{"dimensionName":"userEmail"}],
            "metrics":[{"metricName":"accessCount"}],
            "limit":"1"
        });
        let account = prepare_access(&mut request, "41", true, None, None).unwrap();
        let projected = project(&response, controls(), false, Some(&account)).unwrap();
        assert_eq!(projected.data[0]["userEmail"], "private@example.test");
        assert_eq!(projected.data[0]["accessCount"], "5");
        assert!(projected.meta["next_cursor"].as_str().is_some());
        let cursor = projected.meta["next_cursor"].as_str().unwrap();

        let mut same = request.clone();
        let second = prepare_access(&mut same, "accounts/41", true, Some(cursor), None).unwrap();
        assert_eq!(second.offset, 1);
        let mut different_resource = request.clone();
        assert!(prepare_access(&mut different_resource, "41", false, Some(cursor), None).is_err());
        let mut different_account = request;
        assert!(prepare_access(&mut different_account, "42", true, Some(cursor), None).is_err());
    }

    #[test]
    fn ga64_limit_accepts_provider_integer_width_but_bounds_outbound_rows() {
        let options = controls();
        assert_eq!(options.request_limit(Some(250_000)).unwrap(), Some(1));
        assert_eq!(options.request_limit(Some(1)).unwrap(), Some(1));
        for invalid in [0, 250_001, u64::from(u32::MAX) + 1, u64::MAX] {
            assert_eq!(
                options.request_limit(Some(invalid)),
                Err(ApiError::InvalidRequest)
            );
        }
        let default = Controls {
            output_mode: None,
            summary_only: false,
            max_rows: None,
            max_cell_chars: None,
            cursor_supplied: false,
        };
        assert_eq!(default.request_limit(Some(250_000)).unwrap(), Some(250_000));
    }

    #[test]
    fn historical_cursor_round_trip_enforces_query_hash_and_bounded_offset() {
        let base = json!({
            "dateRanges":[{"startDate":"yesterday","endDate":"today"}],
            "dimensions":[{"name":"country"}],
            "metrics":[{"name":"activeUsers"}],
            "limit":"1"
        });
        let mut first = base.clone();
        let one = prepare_historical(&mut first, "42", None, None).unwrap();
        let projected = project(&fixture(), controls(), false, Some(&one)).unwrap();
        let cursor = projected.meta["next_cursor"].as_str().unwrap().to_owned();
        assert!(cursor.starts_with("v1:"));
        assert!(cursor.ends_with(":1"));
        let mut second = base.clone();
        let two = prepare_historical(&mut second, "properties/42", Some(&cursor), None).unwrap();
        assert_eq!(two.offset, 1);
        assert_eq!(second["offset"], "1");
        assert_eq!(one.query_hash, two.query_hash);
        let third = project(&fixture(), controls(), false, Some(&two)).unwrap();
        assert!(third.meta["next_cursor"].as_str().unwrap().ends_with(":2"));

        let mut changed = base.clone();
        changed["metrics"] = json!([{"name":"screenPageViews"}]);
        assert!(prepare_historical(&mut changed, "42", Some(&cursor), None).is_err());
        let mut mixed = base.clone();
        assert!(prepare_historical(&mut mixed, "42", Some(&cursor), Some(1)).is_err());
        let mut huge = base;
        assert!(prepare_historical(&mut huge, "42", None, Some(MAX_CURSOR_OFFSET + 1)).is_err());
        let mut malformed = first;
        assert!(prepare_historical(&mut malformed, "42", Some("v1:broken:1"), None).is_err());
    }

    #[test]
    fn rejects_invalid_controls_and_malformed_provider_shapes() {
        let mut options = controls();
        options.max_rows = Some(0);
        assert_eq!(options.validate(), Err(ApiError::InvalidRequest));
        assert_eq!(
            options.request_limit(Some(0)),
            Err(ApiError::InvalidRequest)
        );
        options.max_rows = Some(1);
        options.cursor_supplied = true;
        assert!(options.validate().is_ok());
        options.cursor_supplied = false;
        assert!(project(&json!({"rows":[{}],"rowCount":1}), options, false, None).is_err());
        assert!(project(&json!({"rows":[],"rowCount":-1}), options, false, None).is_err());
    }
}
