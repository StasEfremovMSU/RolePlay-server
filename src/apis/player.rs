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
pub enum PlayerIdDeleteResponse {
    /// Player deleted
    Status204_PlayerDeleted
    ,
    /// Player not found
    Status404_PlayerNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PlayerIdGetResponse {
    /// Player data
    Status200_PlayerData
    (models::Player)
    ,
    /// Player not found
    Status404_PlayerNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PlayerIdPutResponse {
    /// Player updated
    Status200_PlayerUpdated
    ,
    /// Player not found
    Status404_PlayerNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PlayerPostResponse {
    /// Player created
    Status201_PlayerCreated
    ,
    /// Invalid input.
    Status400_InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum PlayersGetResponse {
    /// List of all players
    Status200_ListOfAllPlayers
    (Vec<models::Player>)
}


/// Player
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Player {
    /// Delete a player by ID.
    ///
    /// PlayerIdDelete - DELETE /player/{id}
    async fn player_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PlayerIdDeletePathParams,
    ) -> Result<PlayerIdDeleteResponse, ()>;

    /// Get a player by ID.
    ///
    /// PlayerIdGet - GET /player/{id}
    async fn player_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PlayerIdGetPathParams,
    ) -> Result<PlayerIdGetResponse, ()>;

    /// Update a player by ID.
    ///
    /// PlayerIdPut - PUT /player/{id}
    async fn player_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::PlayerIdPutPathParams,
            body: models::Player,
    ) -> Result<PlayerIdPutResponse, ()>;

    /// Add a new player.
    ///
    /// PlayerPost - POST /player
    async fn player_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::Player,
    ) -> Result<PlayerPostResponse, ()>;

    /// Get all players.
    ///
    /// PlayersGet - GET /players
    async fn players_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<PlayersGetResponse, ()>;
}
