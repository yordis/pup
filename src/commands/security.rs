use anyhow::Result;
use datadog_api_client::datadogV2::api_entity_risk_scores::{
    EntityRiskScoresAPI, ListEntityRiskScoresOptionalParams,
};
use datadog_api_client::datadogV2::api_security_monitoring::{
    ListFindingsOptionalParams, ListSecurityMonitoringRulesOptionalParams,
    SearchSecurityMonitoringSignalsOptionalParams, SecurityMonitoringAPI,
};
use datadog_api_client::datadogV2::model::{
    SecurityMonitoringRuleBulkExportAttributes, SecurityMonitoringRuleBulkExportData,
    SecurityMonitoringRuleBulkExportDataType, SecurityMonitoringRuleBulkExportPayload,
    SecurityMonitoringSignalListRequest, SecurityMonitoringSignalListRequestFilter,
    SecurityMonitoringSignalListRequestPage, SecurityMonitoringSignalsSort,
};

use crate::client;
use crate::config::Config;
use crate::formatter;
use crate::util;

pub async fn rules_list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => SecurityMonitoringAPI::with_client_and_config(dd_cfg, c),
        None => SecurityMonitoringAPI::with_config(dd_cfg),
    };
    let resp = api
        .list_security_monitoring_rules(ListSecurityMonitoringRulesOptionalParams::default())
        .await
        .map_err(|e| anyhow::anyhow!("failed to list rules: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn rules_get(cfg: &Config, rule_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => SecurityMonitoringAPI::with_client_and_config(dd_cfg, c),
        None => SecurityMonitoringAPI::with_config(dd_cfg),
    };
    let resp = api
        .get_security_monitoring_rule(rule_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to get rule: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn signals_search(
    cfg: &Config,
    query: String,
    from: String,
    to: String,
    limit: i32,
) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => SecurityMonitoringAPI::with_client_and_config(dd_cfg, c),
        None => SecurityMonitoringAPI::with_config(dd_cfg),
    };

    let from_dt =
        chrono::DateTime::from_timestamp_millis(util::parse_time_to_unix_millis(&from)?).unwrap();
    let to_dt =
        chrono::DateTime::from_timestamp_millis(util::parse_time_to_unix_millis(&to)?).unwrap();

    let body = SecurityMonitoringSignalListRequest::new()
        .filter(
            SecurityMonitoringSignalListRequestFilter::new()
                .query(query)
                .from(from_dt)
                .to(to_dt),
        )
        .page(SecurityMonitoringSignalListRequestPage::new().limit(limit))
        .sort(SecurityMonitoringSignalsSort::TIMESTAMP_DESCENDING);

    let params = SearchSecurityMonitoringSignalsOptionalParams::default().body(body);
    let resp = api
        .search_security_monitoring_signals(params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to search signals: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn findings_search(
    cfg: &Config,
    query: Option<String>,
    limit: i64,
) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => SecurityMonitoringAPI::with_client_and_config(dd_cfg, c),
        None => SecurityMonitoringAPI::with_config(dd_cfg),
    };
    let mut params = ListFindingsOptionalParams::default().page_limit(limit);
    if let Some(q) = query {
        params = params.filter_tags(q);
    }
    let resp = api
        .list_findings(params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to search findings: {e:?}"))?;
    formatter::output(cfg, &resp)
}

// ---- Bulk Export ----

pub async fn rules_bulk_export(cfg: &Config, rule_ids: Vec<String>) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => SecurityMonitoringAPI::with_client_and_config(dd_cfg, c),
        None => SecurityMonitoringAPI::with_config(dd_cfg),
    };
    let attrs = SecurityMonitoringRuleBulkExportAttributes::new(rule_ids);
    let data = SecurityMonitoringRuleBulkExportData::new(
        attrs,
        SecurityMonitoringRuleBulkExportDataType::SECURITY_MONITORING_RULES_BULK_EXPORT,
    );
    let body = SecurityMonitoringRuleBulkExportPayload::new(data);
    let resp = api
        .bulk_export_security_monitoring_rules(body)
        .await
        .map_err(|e| anyhow::anyhow!("failed to bulk export security rules: {e:?}"))?;
    // resp is Vec<u8> (ZIP data), output as raw bytes to stdout
    let output = String::from_utf8_lossy(&resp);
    println!("{output}");
    Ok(())
}

// ---- Content Packs ----

pub async fn content_packs_list(cfg: &Config) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => SecurityMonitoringAPI::with_client_and_config(dd_cfg, c),
        None => SecurityMonitoringAPI::with_config(dd_cfg),
    };
    let resp = api
        .get_content_packs_states()
        .await
        .map_err(|e| anyhow::anyhow!("failed to list content packs: {e:?}"))?;
    formatter::output(cfg, &resp)
}

pub async fn content_packs_activate(cfg: &Config, pack_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => SecurityMonitoringAPI::with_client_and_config(dd_cfg, c),
        None => SecurityMonitoringAPI::with_config(dd_cfg),
    };
    api.activate_content_pack(pack_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to activate content pack: {e:?}"))?;
    println!("Content pack '{pack_id}' activated successfully.");
    Ok(())
}

pub async fn content_packs_deactivate(cfg: &Config, pack_id: &str) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => SecurityMonitoringAPI::with_client_and_config(dd_cfg, c),
        None => SecurityMonitoringAPI::with_config(dd_cfg),
    };
    api.deactivate_content_pack(pack_id.to_string())
        .await
        .map_err(|e| anyhow::anyhow!("failed to deactivate content pack: {e:?}"))?;
    println!("Content pack '{pack_id}' deactivated successfully.");
    Ok(())
}

// ---- Risk Scores ----

pub async fn risk_scores_list(cfg: &Config, query: Option<String>) -> Result<()> {
    let dd_cfg = client::make_dd_config(cfg);
    let api = match client::make_bearer_client(cfg) {
        Some(c) => EntityRiskScoresAPI::with_client_and_config(dd_cfg, c),
        None => EntityRiskScoresAPI::with_config(dd_cfg),
    };
    let mut params = ListEntityRiskScoresOptionalParams::default();
    if let Some(q) = query {
        params = params.filter_query(q);
    }
    let resp = api
        .list_entity_risk_scores(params)
        .await
        .map_err(|e| anyhow::anyhow!("failed to list entity risk scores: {e:?}"))?;
    formatter::output(cfg, &resp)
}
