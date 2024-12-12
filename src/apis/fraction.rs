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
pub enum FractionIdDeleteResponse {
    /// Fraction deleted
    Status204_FractionDeleted
    ,
    /// Fraction not found
    Status404_FractionNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FractionIdGetResponse {
    /// Fraction data
    Status200_FractionData
    (models::Fraction)
    ,
    /// Fraction not found
    Status404_FractionNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FractionIdPutResponse {
    /// Fraction updated
    Status200_FractionUpdated
    ,
    /// Fraction not found
    Status404_FractionNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FractionPostResponse {
    /// Fraction created
    Status201_FractionCreated
    ,
    /// Invalid input.
    Status400_InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FractionsCampaignIdGetResponse {
    /// List of all fractions for campaign
    Status200_ListOfAllFractionsForCampaign
    (Vec<models::Fraction>)
    ,
    /// Campaign not found
    Status404_CampaignNotFound
}


/// Fraction
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Fraction {
    /// Delete a fraction by ID.
    ///
    /// FractionIdDelete - DELETE /fraction/{id}
    async fn fraction_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::FractionIdDeletePathParams,
    ) -> Result<FractionIdDeleteResponse, ()>;

    /// Get a fraction by ID.
    ///
    /// FractionIdGet - GET /fraction/{id}
    async fn fraction_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::FractionIdGetPathParams,
    ) -> Result<FractionIdGetResponse, ()>;

    /// Update a fraction by ID.
    ///
    /// FractionIdPut - PUT /fraction/{id}
    async fn fraction_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::FractionIdPutPathParams,
            body: models::Fraction,
    ) -> Result<FractionIdPutResponse, ()>;

    /// Add a new fraction.
    ///
    /// FractionPost - POST /fraction
    async fn fraction_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::Fraction,
    ) -> Result<FractionPostResponse, ()>;

    /// Get all fractions for campaign Id.
    ///
    /// FractionsCampaignIdGet - GET /fractions/campaign/{id}
    async fn fractions_campaign_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::FractionsCampaignIdGetPathParams,
    ) -> Result<FractionsCampaignIdGetResponse, ()>;
}
