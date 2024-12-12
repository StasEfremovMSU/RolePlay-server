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
pub enum NewsEventsCampaignIdGetResponse {
    /// Campaigns retrieved successfully.
    Status200_CampaignsRetrievedSuccessfully
    (Vec<models::NewsEvent>)
    ,
    /// Campain not found.
    Status404_CampainNotFound
    ,
    /// Invalid input.
    Status400_InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum NewsEventsDeleteResponse {
    /// News event deleted successfully.
    Status200_NewsEventDeletedSuccessfully
    ,
    /// News event not found.
    Status404_NewsEventNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum NewsEventsPostResponse {
    /// News event created successfully.
    Status201_NewsEventCreatedSuccessfully
    (models::NewsEvent)
    ,
    /// Invalid input.
    Status400_InvalidInput
}


/// NewsEvents
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait NewsEvents {
    /// Get a list of all news for campaign ID.
    ///
    /// NewsEventsCampaignIdGet - GET /news-events/campaign/{id}
    async fn news_events_campaign_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::NewsEventsCampaignIdGetPathParams,
    ) -> Result<NewsEventsCampaignIdGetResponse, ()>;

    /// Delete a news event.
    ///
    /// NewsEventsDelete - DELETE /news-events
    async fn news_events_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      query_params: models::NewsEventsDeleteQueryParams,
    ) -> Result<NewsEventsDeleteResponse, ()>;

    /// Add a new news event.
    ///
    /// NewsEventsPost - POST /news-events
    async fn news_events_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::NewsEvent,
    ) -> Result<NewsEventsPostResponse, ()>;
}
