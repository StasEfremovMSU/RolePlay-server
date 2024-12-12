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
pub enum DataMapIdDeleteResponse {
    /// Data map deleted successfully.
    Status200_DataMapDeletedSuccessfully
    ,
    /// Data map not found.
    Status404_DataMapNotFound
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DataMapIdGetResponse {
    /// Data map retrieved successfully.
    Status200_DataMapRetrievedSuccessfully
    (models::DataMap)
    ,
    /// Data map not found.
    Status404_DataMapNotFound
    ,
    /// Invalid input.
    Status400_InvalidInput
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DataMapIdPutResponse {
    /// Data map updated successfully.
    Status200_DataMapUpdatedSuccessfully
    (models::DataMap)
    ,
    /// Data map not found.
    Status404_DataMapNotFound
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
pub enum DataMapPostResponse {
    /// Data map created successfully.
    Status201_DataMapCreatedSuccessfully
    (models::DataMap)
    ,
    /// Invalid input.
    Status400_InvalidInput
}


/// DataMap
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait DataMap {
    /// Delete a data map.
    ///
    /// DataMapIdDelete - DELETE /data-map/{id}
    async fn data_map_id_delete(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::DataMapIdDeletePathParams,
    ) -> Result<DataMapIdDeleteResponse, ()>;

    /// Get a Data map by ID.
    ///
    /// DataMapIdGet - GET /data-map/{id}
    async fn data_map_id_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::DataMapIdGetPathParams,
    ) -> Result<DataMapIdGetResponse, ()>;

    /// Update a data map by ID.
    ///
    /// DataMapIdPut - PUT /data-map/{id}
    async fn data_map_id_put(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
      path_params: models::DataMapIdPutPathParams,
            body: models::DataMap,
    ) -> Result<DataMapIdPutResponse, ()>;

    /// Add a new data map.
    ///
    /// DataMapPost - POST /data-map
    async fn data_map_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: models::DataMap,
    ) -> Result<DataMapPostResponse, ()>;
}
