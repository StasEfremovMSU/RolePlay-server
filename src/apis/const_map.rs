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
pub enum ConstMapIdDeleteResponse {
    /// Const map deleted successfully.
    Status200_ConstMapDeletedSuccessfully
    ,
    /// Const map not found.
    Status404_ConstMapNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ConstMapIdGetResponse {
    /// Const map retrieved successfully.
    Status200_ConstMapRetrievedSuccessfully
    (models::ConstMap)
    ,
    /// Const map not found.
    Status404_ConstMapNotFound
    ,
    /// Invalid input.
    Status400_InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ConstMapIdPutResponse {
    /// Const map updated successfully.
    Status200_ConstMapUpdatedSuccessfully
    (models::ConstMap)
    ,
    /// Const map not found.
    Status404_ConstMapNotFound
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
pub enum ConstMapsPostResponse {
    /// Const map created successfully.
    Status201_ConstMapCreatedSuccessfully
    (models::ConstMap)
    ,
    /// Invalid input.
    Status400_InvalidInput
}


/// ConstMap
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait ConstMap {
    /// Delete a const map.
    ///
    /// ConstMapIdDelete - DELETE /const_map/{id}
    async fn const_map_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ConstMapIdDeletePathParams,
    ) -> Result<ConstMapIdDeleteResponse, ()>;

    /// Get a Const map by ID.
    ///
    /// ConstMapIdGet - GET /const_map/{id}
    async fn const_map_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ConstMapIdGetPathParams,
    ) -> Result<ConstMapIdGetResponse, ()>;

    /// Update a const map by ID.
    ///
    /// ConstMapIdPut - PUT /const_map/{id}
    async fn const_map_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::ConstMapIdPutPathParams,
            body: models::ConstMap,
    ) -> Result<ConstMapIdPutResponse, ()>;

    /// Add a new const map.
    ///
    /// ConstMapsPost - POST /const_maps
    async fn const_maps_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::ConstMap,
    ) -> Result<ConstMapsPostResponse, ()>;
}
