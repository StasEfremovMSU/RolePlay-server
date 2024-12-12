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
pub enum FactionPlayerFractionIdGetResponse {
    /// List of all faction players for fraction id
    Status200_ListOfAllFactionPlayersForFractionId
    (Vec<models::FactionPlayer>)
    ,
    /// fraction not found
    Status404_FractionNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FactionPlayerPostResponse {
    /// Faction player created
    Status201_FactionPlayerCreated
    ,
    /// Invalid input.
    Status400_InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FactionPlayersIdDeleteResponse {
    /// Faction player deleted
    Status204_FactionPlayerDeleted
    ,
    /// Faction player not found
    Status404_FactionPlayerNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FactionPlayersIdGetResponse {
    /// Faction player data
    Status200_FactionPlayerData
    (models::FactionPlayer)
    ,
    /// Faction player not found
    Status404_FactionPlayerNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FactionPlayersIdPutResponse {
    /// Faction player updated
    Status200_FactionPlayerUpdated
    ,
    /// Faction player not found
    Status404_FactionPlayerNotFound
}


/// FactionPlayer
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait FactionPlayer {
    /// Get all faction players for fraction ID.
    ///
    /// FactionPlayerFractionIdGet - GET /faction-player/fraction/{id}
    async fn faction_player_fraction_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::FactionPlayerFractionIdGetPathParams,
    ) -> Result<FactionPlayerFractionIdGetResponse, ()>;

    /// Add a new faction player.
    ///
    /// FactionPlayerPost - POST /faction-player
    async fn faction_player_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::FactionPlayer,
    ) -> Result<FactionPlayerPostResponse, ()>;

    /// Delete a faction player by ID.
    ///
    /// FactionPlayersIdDelete - DELETE /faction-players/{id}
    async fn faction_players_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::FactionPlayersIdDeletePathParams,
    ) -> Result<FactionPlayersIdDeleteResponse, ()>;

    /// Get a faction player by ID.
    ///
    /// FactionPlayersIdGet - GET /faction-players/{id}
    async fn faction_players_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::FactionPlayersIdGetPathParams,
    ) -> Result<FactionPlayersIdGetResponse, ()>;

    /// Update a faction player by ID.
    ///
    /// FactionPlayersIdPut - PUT /faction-players/{id}
    async fn faction_players_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::FactionPlayersIdPutPathParams,
            body: models::FactionPlayer,
    ) -> Result<FactionPlayersIdPutResponse, ()>;
}
