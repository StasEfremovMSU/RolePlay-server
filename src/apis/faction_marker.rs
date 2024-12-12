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
pub enum FactionMarkerFractionIdGetResponse {
    /// List of all faction markers
    Status200_ListOfAllFactionMarkers
    (Vec<models::FactionMarker>)
    ,
    /// Fraction not found
    Status404_FractionNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FactionMarkerIdDeleteResponse {
    /// Faction marker deleted successfully.
    Status204_FactionMarkerDeletedSuccessfully
    ,
    /// Faction marker not found.
    Status404_FactionMarkerNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FactionMarkerPostResponse {
    /// Faction marker created
    Status201_FactionMarkerCreated
    (models::FactionMarker)
    ,
    /// Invalid input.
    Status400_InvalidInput
}


/// FactionMarker
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait FactionMarker {
    /// Get all faction markers for fraction.
    ///
    /// FactionMarkerFractionIdGet - GET /faction-marker/fraction/{id}
    async fn faction_marker_fraction_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::FactionMarkerFractionIdGetPathParams,
    ) -> Result<FactionMarkerFractionIdGetResponse, ()>;

    /// Delete a faction marker by ID.
    ///
    /// FactionMarkerIdDelete - DELETE /faction-marker/{id}
    async fn faction_marker_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::FactionMarkerIdDeletePathParams,
    ) -> Result<FactionMarkerIdDeleteResponse, ()>;

    /// Add a new faction marker.
    ///
    /// FactionMarkerPost - POST /faction-marker
    async fn faction_marker_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::FactionMarker,
    ) -> Result<FactionMarkerPostResponse, ()>;
}
