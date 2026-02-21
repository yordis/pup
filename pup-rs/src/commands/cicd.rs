use anyhow::Result;
use datadog_api_client::datadogV2::api_ci_visibility_pipelines::{
    CIVisibilityPipelinesAPI, SearchCIAppPipelineEventsOptionalParams,
};
use datadog_api_client::datadogV2::api_ci_visibility_tests::{
    CIVisibilityTestsAPI, ListCIAppTestEventsOptionalParams,
};
use datadog_api_client::datadogV2::model::{
    CIAppPipelineEventsRequest, CIAppPipelinesQueryFilter, CIAppQueryPageOptions, CIAppSort,
};

use crate::client;
use crate::config::Config;
use crate::formatter;
use crate::util;

pub async fn pipelines_list(
    cfg: &Config,
    query: Option<String>,
    from: String,
    to: String,
    limit: i32,
) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => CIVisibilityPipelinesAPI::with_client_and_config(dd_cfg, c),
        None => CIVisibilityPipelinesAPI::with_config(dd_cfg),
    };

    let from_ms = util::parse_time_to_unix_millis(&from)?;
    let to_ms = util::parse_time_to_unix_millis(&to)?;
    let from_str = chrono::DateTime::from_timestamp_millis(from_ms).unwrap().to_rfc3339();
    let to_str = chrono::DateTime::from_timestamp_millis(to_ms).unwrap().to_rfc3339();

    let mut filter = CIAppPipelinesQueryFilter::new().from(from_str).to(to_str);
    if let Some(q) = query {
        filter = filter.query(q);
    }

    let body = CIAppPipelineEventsRequest::new()
        .filter(filter)
        .page(CIAppQueryPageOptions::new().limit(limit))
        .sort(CIAppSort::TIMESTAMP_DESCENDING);

    let params = SearchCIAppPipelineEventsOptionalParams::default().body(body);
    let resp = api
        .search_ci_app_pipeline_events(params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list pipelines: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn tests_list(
    cfg: &Config,
    query: Option<String>,
    from: String,
    to: String,
    limit: i32,
) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => CIVisibilityTestsAPI::with_client_and_config(dd_cfg, c),
        None => CIVisibilityTestsAPI::with_config(dd_cfg),
    };

    let from_dt =
        chrono::DateTime::from_timestamp_millis(util::parse_time_to_unix_millis(&from)?).unwrap();
    let to_dt =
        chrono::DateTime::from_timestamp_millis(util::parse_time_to_unix_millis(&to)?).unwrap();

    let mut params = ListCIAppTestEventsOptionalParams::default()
        .filter_from(from_dt)
        .filter_to(to_dt)
        .page_limit(limit);
    if let Some(q) = query {
        params = params.filter_query(q);
    }

    let resp = api
        .list_ci_app_test_events(params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list tests: {e:?}"))?;
    formatter::output(cfg, &resp)
}
