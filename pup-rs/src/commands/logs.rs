use anyhow::{bail, Result};
use datadog_api_client::datadogV2::api_logs::{ListLogsOptionalParams, LogsAPI};
use datadog_api_client::datadogV2::api_logs_archives::LogsArchivesAPI;
use datadog_api_client::datadogV2::api_logs_custom_destinations::LogsCustomDestinationsAPI;
use datadog_api_client::datadogV2::api_logs_metrics::LogsMetricsAPI;
use datadog_api_client::datadogV2::model::{
    LogsAggregateRequest, LogsAggregationFunction, LogsCompute, LogsListRequest,
    LogsListRequestPage, LogsQueryFilter, LogsSort,
};

use crate::client;
use crate::config::Config;
use crate::formatter;
use crate::util;

pub async fn search(
    cfg: &Config,
    query: String,
    from: String,
    to: String,
    limit: i32,
) -> Result<()> {
    // Logs search API doesn't support OAuth/bearer - force API keys
    if !cfg.has_api_keys() {
        bail!(
            "logs search requires API key authentication (DD_API_KEY + DD_APP_KEY).\n\
             This endpoint does not support bearer token auth."
        );
    }

    let dd_cfg = client::make_dd_config(cfg);
    // Force API key auth only - do NOT use bearer middleware
    let api = LogsAPI::with_config(dd_cfg);

    let from_ms = util::parse_time_to_unix_millis(&from)?;
    let to_ms = util::parse_time_to_unix_millis(&to)?;

    let body = LogsListRequest::new()
        .filter(
            LogsQueryFilter::new()
                .query(query)
                .from(from_ms.to_string())
                .to(to_ms.to_string()),
        )
        .page(LogsListRequestPage::new().limit(limit))
        .sort(LogsSort::TIMESTAMP_DESCENDING);

    let params = ListLogsOptionalParams::default().body(body);

    let resp = api
        .list_logs(params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to search logs: {:?}", e))?;

    formatter::output(cfg, &resp)?;
    Ok(())
}

/// Alias for `search` with the same interface.
pub async fn list(
    cfg: &Config,
    query: String,
    from: String,
    to: String,
    limit: i32,
) -> Result<()> {
    search(cfg, query, from, to, limit).await
}

/// Alias for `search` with the same interface.
pub async fn query(
    cfg: &Config,
    query: String,
    from: String,
    to: String,
    limit: i32,
) -> Result<()> {
    search(cfg, query, from, to, limit).await
}

pub async fn aggregate(
    cfg: &Config,
    query: String,
    from: String,
    to: String,
) -> Result<()> {
    if !cfg.has_api_keys() {
        bail!(
            "logs aggregate requires API key authentication (DD_API_KEY + DD_APP_KEY).\n\
             This endpoint does not support bearer token auth."
        );
    }

    let dd_cfg = client::make_dd_config(cfg);
    let api = LogsAPI::with_config(dd_cfg);

    let from_ms = util::parse_time_to_unix_millis(&from)?;
    let to_ms = util::parse_time_to_unix_millis(&to)?;

    let body = LogsAggregateRequest::new()
        .filter(
            LogsQueryFilter::new()
                .query(query)
                .from(from_ms.to_string())
                .to(to_ms.to_string()),
        )
        .compute(vec![LogsCompute::new(LogsAggregationFunction::COUNT)]);

    let resp = api
        .aggregate_logs(body)
        .await
        .map_err(|e| anyhow::anyhow!("failed to aggregate logs: {:?}", e))?;

    formatter::output(cfg, &resp)?;
    Ok(())
}

pub async fn archives_list(cfg: &Config) -> Result<()> {
    if !cfg.has_api_keys() {
        bail!(
            "logs archives list requires API key authentication (DD_API_KEY + DD_APP_KEY).\n\
             This endpoint does not support bearer token auth."
        );
    }

    let dd_cfg = client::make_dd_config(cfg);
    let api = LogsArchivesAPI::with_config(dd_cfg);

    let resp = api
        .list_logs_archives()
        .await
        .map_err(|e| anyhow::anyhow!("failed to list log archives: {:?}", e))?;

    formatter::output(cfg, &resp)?;
    Ok(())
}

pub async fn archives_get(cfg: &Config, archive_id: &str) -> Result<()> {
    if !cfg.has_api_keys() {
        bail!(
            "logs archives get requires API key authentication (DD_API_KEY + DD_APP_KEY).\n\
             This endpoint does not support bearer token auth."
        );
    }

    let dd_cfg = client::make_dd_config(cfg);
    let api = LogsArchivesAPI::with_config(dd_cfg);

    let resp = api
        .get_logs_archive(archive_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to get log archive: {:?}", e))?;

    formatter::output(cfg, &resp)?;
    Ok(())
}

pub async fn archives_delete(cfg: &Config, archive_id: &str) -> Result<()> {
    if !cfg.has_api_keys() {
        bail!(
            "logs archives delete requires API key authentication (DD_API_KEY + DD_APP_KEY).\n\
             This endpoint does not support bearer token auth."
        );
    }

    let dd_cfg = client::make_dd_config(cfg);
    let api = LogsArchivesAPI::with_config(dd_cfg);

    api.delete_logs_archive(archive_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to delete log archive: {:?}", e))?;

    eprintln!("Log archive {archive_id} deleted.");
    Ok(())
}

pub async fn custom_destinations_list(cfg: &Config) -> Result<()> {
    if !cfg.has_api_keys() {
        bail!(
            "logs custom-destinations list requires API key authentication (DD_API_KEY + DD_APP_KEY).\n\
             This endpoint does not support bearer token auth."
        );
    }

    let dd_cfg = client::make_dd_config(cfg);
    let api = LogsCustomDestinationsAPI::with_config(dd_cfg);

    let resp = api
        .list_logs_custom_destinations()
        .await
        .map_err(|e| anyhow::anyhow!("failed to list custom destinations: {:?}", e))?;

    formatter::output(cfg, &resp)?;
    Ok(())
}

pub async fn custom_destinations_get(cfg: &Config, destination_id: &str) -> Result<()> {
    if !cfg.has_api_keys() {
        bail!(
            "logs custom-destinations get requires API key authentication (DD_API_KEY + DD_APP_KEY).\n\
             This endpoint does not support bearer token auth."
        );
    }

    let dd_cfg = client::make_dd_config(cfg);
    let api = LogsCustomDestinationsAPI::with_config(dd_cfg);

    let resp = api
        .get_logs_custom_destination(destination_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to get custom destination: {:?}", e))?;

    formatter::output(cfg, &resp)?;
    Ok(())
}

pub async fn metrics_list(cfg: &Config) -> Result<()> {
    if !cfg.has_api_keys() {
        bail!(
            "logs metrics list requires API key authentication (DD_API_KEY + DD_APP_KEY).\n\
             This endpoint does not support bearer token auth."
        );
    }

    let dd_cfg = client::make_dd_config(cfg);
    let api = LogsMetricsAPI::with_config(dd_cfg);

    let resp = api
        .list_logs_metrics()
        .await
        .map_err(|e| anyhow::anyhow!("failed to list log-based metrics: {:?}", e))?;

    formatter::output(cfg, &resp)?;
    Ok(())
}

pub async fn metrics_get(cfg: &Config, metric_id: &str) -> Result<()> {
    if !cfg.has_api_keys() {
        bail!(
            "logs metrics get requires API key authentication (DD_API_KEY + DD_APP_KEY).\n\
             This endpoint does not support bearer token auth."
        );
    }

    let dd_cfg = client::make_dd_config(cfg);
    let api = LogsMetricsAPI::with_config(dd_cfg);

    let resp = api
        .get_logs_metric(metric_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to get log-based metric: {:?}", e))?;

    formatter::output(cfg, &resp)?;
    Ok(())
}
