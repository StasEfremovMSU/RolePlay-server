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
pub enum MegaCampainsForPlayerIdGetResponse {
    /// A list of mega campaigns.
    Status200_AListOfMegaCampaigns
    (Vec<models::MegaCampaign>)
    ,
    /// Server error.
    Status500_ServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MegaCampainsGetResponse {
    /// A list of mega campaigns.
    Status200_AListOfMegaCampaigns
    (Vec<models::MegaCampaign>)
    ,
    /// Server error.
    Status500_ServerError
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MegaCampainsOpenGetResponse {
    /// A list of mega campaigns.
    Status200_AListOfMegaCampaigns
    (Vec<models::MegaCampaign>)
    ,
    /// Server error.
    Status500_ServerError
}


/// MegaCampaigns
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait MegaCampaigns {
    /// Get all campaigns that player own or involved.
    ///
    /// MegaCampainsForPlayerIdGet - GET /mega-campains/for-player/{id}
    async fn mega_campains_for_player_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::MegaCampainsForPlayerIdGetPathParams,
    ) -> Result<MegaCampainsForPlayerIdGetResponse, ()>;

    /// Get all mega campaigns.
    ///
    /// MegaCampainsGet - GET /mega-campains
    async fn mega_campains_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<MegaCampainsGetResponse, ()>;

    /// Get all open campaigns.
    ///
    /// MegaCampainsOpenGet - GET /mega-campains/open
    async fn mega_campains_open_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<MegaCampainsOpenGetResponse, ()>;
}
