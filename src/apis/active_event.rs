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
pub enum ActiveEventsIdDeleteResponse {
    /// Active event deleted
    Status204_ActiveEventDeleted
    ,
    /// Active event not found
    Status404_ActiveEventNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ActiveEventsIdGetResponse {
    /// Active event data
    Status200_ActiveEventData
    (models::ActiveEvent)
    ,
    /// Active event not found
    Status404_ActiveEventNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ActiveEventsPostResponse {
    /// Active event created
    Status201_ActiveEventCreated
    ,
    /// Invalid input.
    Status400_InvalidInput
}


/// ActiveEvent
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ActiveEvent {
    /// Delete an active event by ID.
    ///
    /// ActiveEventsIdDelete - DELETE /active-events/{id}
    async fn active_events_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ActiveEventsIdDeletePathParams,
    ) -> Result<ActiveEventsIdDeleteResponse, ()>;

    /// Get an active event by ID.
    ///
    /// ActiveEventsIdGet - GET /active-events/{id}
    async fn active_events_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ActiveEventsIdGetPathParams,
    ) -> Result<ActiveEventsIdGetResponse, ()>;

    /// Add a new active event.
    ///
    /// ActiveEventsPost - POST /active-events
    async fn active_events_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::ActiveEvent,
    ) -> Result<ActiveEventsPostResponse, ()>;
}
