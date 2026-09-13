//! Merchant reports and payout control.
//!
//! These endpoints live on the merchant host ([`DEFAULT_MERCHANT_URL`]) and
//! use dedicated API versions: the report list is v2, the count v3.
//! See the methods on [`BepaidClient`].

use reqwest::Method;

use crate::client::BepaidClient;
use crate::error::BepaidError;
use crate::types::{
    ChannelBalance, ReportCountRequest, ReportCountResponse, ReportListRequest, ReportListResponse,
};

impl BepaidClient {
    /// List shop transactions for one report date (`API v2`).
    ///
    /// Result is best paginated by the `id` field of
    /// [`crate::types::ReportTransaction`] above ~1000 transactions.
    pub async fn get_reports(
        &self,
        req: ReportListRequest,
    ) -> Result<ReportListResponse, BepaidError> {
        self.request_json(
            Method::POST,
            &self.merchant("/api/reports"),
            Some(&req),
            Some("2"),
        )
        .await
    }

    /// Count shop transactions in a date range (`API v3`).
    pub async fn get_report_count(
        &self,
        req: ReportCountRequest,
    ) -> Result<ReportCountResponse, BepaidError> {
        self.request_json(
            Method::POST,
            &self.merchant("/api/reports/count"),
            Some(&req),
            Some("3"),
        )
        .await
    }

    /// List payout channel balances (payout control).
    pub async fn get_channel_balances(
        &self,
        gateway_id: i64,
        currency: Option<&str>,
    ) -> Result<Vec<ChannelBalance>, BepaidError> {
        let path = match currency {
            Some(c) => format!("/shop/channel_balances/?gateway_id={gateway_id}&currency={c}"),
            None => format!("/shop/channel_balances/?gateway_id={gateway_id}"),
        };
        self.request_json(Method::GET, &self.merchant(&path), None::<&u8>, None)
            .await
    }
}
