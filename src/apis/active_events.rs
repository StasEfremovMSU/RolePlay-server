use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ActiveEventsCampaignIdGetResponse {
    /// List of all active events for campaign ID
    Status200_ListOfAllActiveEventsForCampaignID
    (Vec<models::ActiveEvent>)
    ,
    /// Campaign not found
    Status404_CampaignNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ActiveEventsFractionIdGetResponse {
    /// List of all active events for fraction ID
    Status200_ListOfAllActiveEventsForFractionID
    (Vec<models::ActiveEvent>)
    ,
    /// Fraction not found
    Status404_FractionNotFound
}


/// ActiveEvents
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ActiveEvents {
    /// Get all active events for campaign ID.
    ///
    /// ActiveEventsCampaignIdGet - GET /active-events/campaign/{id}
    async fn active_events_campaign_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ActiveEventsCampaignIdGetPathParams,
    ) -> Result<ActiveEventsCampaignIdGetResponse, ()>;

    /// Get all active events for fraction ID.
    ///
    /// ActiveEventsFractionIdGet - GET /active-events/fraction/{id}
    async fn active_events_fraction_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ActiveEventsFractionIdGetPathParams,
    ) -> Result<ActiveEventsFractionIdGetResponse, ()>;
}
