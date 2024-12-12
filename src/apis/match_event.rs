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
pub enum MatchEventsFractionIdGetResponse {
    /// List of all match events for fraction
    Status200_ListOfAllMatchEventsForFraction
    (Vec<models::MatchEvent>)
    ,
    /// Fraction not found
    Status404_FractionNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MatchEventsIdDeleteResponse {
    /// Match event deleted successfully
    Status204_MatchEventDeletedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MatchEventsIdGetResponse {
    /// List of all match events
    Status200_ListOfAllMatchEvents
    (Vec<models::MatchEvent>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MatchEventsPlayerIdGetResponse {
    /// List of all match events for player
    Status200_ListOfAllMatchEventsForPlayer
    (Vec<models::MatchEvent>)
    ,
    /// Player not found
    Status404_PlayerNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum MatchEventsPostResponse {
    /// Match event created successfully
    Status201_MatchEventCreatedSuccessfully
    (models::MatchEvent)
    ,
    /// Invalid input.
    Status400_InvalidInput
}


/// MatchEvent
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait MatchEvent {
    /// Get all match events for fraction.
    ///
    /// MatchEventsFractionIdGet - GET /match-events/fraction/{id}
    async fn match_events_fraction_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::MatchEventsFractionIdGetPathParams,
    ) -> Result<MatchEventsFractionIdGetResponse, ()>;

    /// Delete a match event by ID.
    ///
    /// MatchEventsIdDelete - DELETE /match-events/{id}
    async fn match_events_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::MatchEventsIdDeletePathParams,
    ) -> Result<MatchEventsIdDeleteResponse, ()>;

    /// Get all match events.
    ///
    /// MatchEventsIdGet - GET /match-events/{id}
    async fn match_events_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::MatchEventsIdGetPathParams,
    ) -> Result<MatchEventsIdGetResponse, ()>;

    /// Get all match events for player.
    ///
    /// MatchEventsPlayerIdGet - GET /match-events/player/{id}
    async fn match_events_player_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::MatchEventsPlayerIdGetPathParams,
    ) -> Result<MatchEventsPlayerIdGetResponse, ()>;

    /// Add a new match event.
    ///
    /// MatchEventsPost - POST /match-events
    async fn match_events_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::MatchEvent,
    ) -> Result<MatchEventsPostResponse, ()>;
}
