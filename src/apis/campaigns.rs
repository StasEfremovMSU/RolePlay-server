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
pub enum CampaignIdDeleteResponse {
    /// Campaign deleted successfully.
    Status200_CampaignDeletedSuccessfully
    ,
    /// Campaign not found.
    Status404_CampaignNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CampaignIdGetResponse {
    /// Campaign retrieved successfully.
    Status200_CampaignRetrievedSuccessfully
    (models::Campaign)
    ,
    /// Campaign not found.
    Status404_CampaignNotFound
    ,
    /// Invalid input.
    Status400_InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CampaignIdPutResponse {
    /// Campaign updated successfully.
    Status200_CampaignUpdatedSuccessfully
    (models::Campaign)
    ,
    /// Campaign not found.
    Status404_CampaignNotFound
    ,
    /// Invalid input.
    Status400_InvalidInput
    ,
    /// Server error.
    Status500_ServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CampaignPostResponse {
    /// Campaign created successfully.
    Status201_CampaignCreatedSuccessfully
    (models::Campaign)
    ,
    /// Invalid input.
    Status400_InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CampaignsMegaCampaignIdGetResponse {
    /// Campaigns retrieved successfully.
    Status200_CampaignsRetrievedSuccessfully
    (Vec<models::Campaign>)
    ,
    /// Mega campaign not found.
    Status404_MegaCampaignNotFound
    ,
    /// Invalid input.
    Status400_InvalidInput
}


/// Campaigns
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Campaigns {
    /// Delete a campaign by ID.
    ///
    /// CampaignIdDelete - DELETE /campaign/{id}
    async fn campaign_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::CampaignIdDeletePathParams,
    ) -> Result<CampaignIdDeleteResponse, ()>;

    /// Get a campaign by ID.
    ///
    /// CampaignIdGet - GET /campaign/{id}
    async fn campaign_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::CampaignIdGetPathParams,
    ) -> Result<CampaignIdGetResponse, ()>;

    /// Update a campaign by ID.
    ///
    /// CampaignIdPut - PUT /campaign/{id}
    async fn campaign_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::CampaignIdPutPathParams,
            body: models::Campaign,
    ) -> Result<CampaignIdPutResponse, ()>;

    /// Add a new campaign.
    ///
    /// CampaignPost - POST /campaign
    async fn campaign_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::Campaign,
    ) -> Result<CampaignPostResponse, ()>;

    /// Get a campaigns by mega camp ID.
    ///
    /// CampaignsMegaCampaignIdGet - GET /campaigns/mega-campaign/{id}
    async fn campaigns_mega_campaign_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::CampaignsMegaCampaignIdGetPathParams,
    ) -> Result<CampaignsMegaCampaignIdGetResponse, ()>;
}
