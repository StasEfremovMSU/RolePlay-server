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
pub enum MegaCampainIdDeleteResponse {
    /// Mega campaign deleted successfully.
    Status200_MegaCampaignDeletedSuccessfully
    ,
    /// Mega campaign not found.
    Status404_MegaCampaignNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MegaCampainIdPutResponse {
    /// Mega campaign updated successfully.
    Status200_MegaCampaignUpdatedSuccessfully
    (models::MegaCampaign)
    ,
    /// Invalid input.
    Status400_InvalidInput
    ,
    /// Mega campaign not found.
    Status404_MegaCampaignNotFound
    ,
    /// Server error.
    Status500_ServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MegaCampainPostResponse {
    /// Mega campaign created successfully.
    Status201_MegaCampaignCreatedSuccessfully
    (models::MegaCampaign)
    ,
    /// Invalid input.
    Status400_InvalidInput
}


/// MegaCampaign
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait MegaCampaign {
    /// Delete a mega campaign.
    ///
    /// MegaCampainIdDelete - DELETE /mega-campain/{id}
    async fn mega_campain_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::MegaCampainIdDeletePathParams,
    ) -> Result<MegaCampainIdDeleteResponse, ()>;

    /// Update a mega campaign.
    ///
    /// MegaCampainIdPut - PUT /mega-campain/{id}
    async fn mega_campain_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::MegaCampainIdPutPathParams,
            body: models::MegaCampaign,
    ) -> Result<MegaCampainIdPutResponse, ()>;

    /// Add a new mega campaign.
    ///
    /// MegaCampainPost - POST /mega-campain
    async fn mega_campain_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::MegaCampaign,
    ) -> Result<MegaCampainPostResponse, ()>;
}
