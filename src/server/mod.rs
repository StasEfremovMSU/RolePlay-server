use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::{apis, models};


/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: apis::active_event::ActiveEvent + apis::active_events::ActiveEvents + apis::campaigns::Campaigns + apis::const_map::ConstMap + apis::data_map::DataMap + apis::faction_marker::FactionMarker + apis::faction_player::FactionPlayer + apis::fraction::Fraction + apis::match_event::MatchEvent + apis::mega_campaign::MegaCampaign + apis::mega_campaigns::MegaCampaigns + apis::news_events::NewsEvents + apis::player::Player + 'static,
{
    // build our application with a route
    Router::new()
        .route("/active-events",
            post(active_events_post::<I, A>)
        )
        .route("/active-events/:id",
            delete(active_events_id_delete::<I, A>).get(active_events_id_get::<I, A>)
        )
        .route("/active-events/campaign/:id",
            get(active_events_campaign_id_get::<I, A>)
        )
        .route("/active-events/fraction/:id",
            get(active_events_fraction_id_get::<I, A>)
        )
        .route("/campaign",
            post(campaign_post::<I, A>)
        )
        .route("/campaign/:id",
            delete(campaign_id_delete::<I, A>).get(campaign_id_get::<I, A>).put(campaign_id_put::<I, A>)
        )
        .route("/campaigns/mega-campaign/:id",
            get(campaigns_mega_campaign_id_get::<I, A>)
        )
        .route("/const_map/:id",
            delete(const_map_id_delete::<I, A>).get(const_map_id_get::<I, A>).put(const_map_id_put::<I, A>)
        )
        .route("/const_maps",
            post(const_maps_post::<I, A>)
        )
        .route("/data-map",
            post(data_map_post::<I, A>)
        )
        .route("/data-map/:id",
            delete(data_map_id_delete::<I, A>).get(data_map_id_get::<I, A>).put(data_map_id_put::<I, A>)
        )
        .route("/faction-marker",
            post(faction_marker_post::<I, A>)
        )
        .route("/faction-marker/:id",
            delete(faction_marker_id_delete::<I, A>)
        )
        .route("/faction-marker/fraction/:id",
            get(faction_marker_fraction_id_get::<I, A>)
        )
        .route("/faction-player",
            post(faction_player_post::<I, A>)
        )
        .route("/faction-player/fraction/:id",
            get(faction_player_fraction_id_get::<I, A>)
        )
        .route("/faction-players/:id",
            delete(faction_players_id_delete::<I, A>).get(faction_players_id_get::<I, A>).put(faction_players_id_put::<I, A>)
        )
        .route("/fraction",
            post(fraction_post::<I, A>)
        )
        .route("/fraction/:id",
            delete(fraction_id_delete::<I, A>).get(fraction_id_get::<I, A>).put(fraction_id_put::<I, A>)
        )
        .route("/fractions/campaign/:id",
            get(fractions_campaign_id_get::<I, A>)
        )
        .route("/match-events",
            post(match_events_post::<I, A>)
        )
        .route("/match-events/:id",
            delete(match_events_id_delete::<I, A>).get(match_events_id_get::<I, A>)
        )
        .route("/match-events/fraction/:id",
            get(match_events_fraction_id_get::<I, A>)
        )
        .route("/match-events/player/:id",
            get(match_events_player_id_get::<I, A>)
        )
        .route("/mega-campain",
            post(mega_campain_post::<I, A>)
        )
        .route("/mega-campain/:id",
            delete(mega_campain_id_delete::<I, A>).put(mega_campain_id_put::<I, A>)
        )
        .route("/mega-campains",
            get(mega_campains_get::<I, A>)
        )
        .route("/mega-campains/for-player/:id",
            get(mega_campains_for_player_id_get::<I, A>)
        )
        .route("/mega-campains/open",
            get(mega_campains_open_get::<I, A>)
        )
        .route("/news-events",
            delete(news_events_delete::<I, A>).post(news_events_post::<I, A>)
        )
        .route("/news-events/campaign/:id",
            get(news_events_campaign_id_get::<I, A>)
        )
        .route("/player",
            post(player_post::<I, A>)
        )
        .route("/player/:id",
            delete(player_id_delete::<I, A>).get(player_id_get::<I, A>).put(player_id_put::<I, A>)
        )
        .route("/players",
            get(players_get::<I, A>)
        )
        .with_state(api_impl)
}


#[tracing::instrument(skip_all)]
fn active_events_id_delete_validation(
  path_params: models::ActiveEventsIdDeletePathParams,
) -> std::result::Result<(
  models::ActiveEventsIdDeletePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// ActiveEventsIdDelete - DELETE /active-events/{id}
#[tracing::instrument(skip_all)]
async fn active_events_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::ActiveEventsIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::active_event::ActiveEvent,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    active_events_id_delete_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().active_events_id_delete(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::active_event::ActiveEventsIdDeleteResponse::Status204_ActiveEventDeleted
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::active_event::ActiveEventsIdDeleteResponse::Status404_ActiveEventNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn active_events_id_get_validation(
  path_params: models::ActiveEventsIdGetPathParams,
) -> std::result::Result<(
  models::ActiveEventsIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// ActiveEventsIdGet - GET /active-events/{id}
#[tracing::instrument(skip_all)]
async fn active_events_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::ActiveEventsIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::active_event::ActiveEvent,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    active_events_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().active_events_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::active_event::ActiveEventsIdGetResponse::Status200_ActiveEventData
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::active_event::ActiveEventsIdGetResponse::Status404_ActiveEventNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct ActiveEventsPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::ActiveEvent,
    }


#[tracing::instrument(skip_all)]
fn active_events_post_validation(
        body: models::ActiveEvent,
) -> std::result::Result<(
        models::ActiveEvent,
), ValidationErrors>
{
              let b = ActiveEventsPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// ActiveEventsPost - POST /active-events
#[tracing::instrument(skip_all)]
async fn active_events_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::ActiveEvent>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::active_event::ActiveEvent,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    active_events_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().active_events_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::active_event::ActiveEventsPostResponse::Status201_ActiveEventCreated
                                                => {
                                                  let mut response = response.status(201);
                                                  response.body(Body::empty())
                                                },
                                                apis::active_event::ActiveEventsPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn active_events_campaign_id_get_validation(
  path_params: models::ActiveEventsCampaignIdGetPathParams,
) -> std::result::Result<(
  models::ActiveEventsCampaignIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// ActiveEventsCampaignIdGet - GET /active-events/campaign/{id}
#[tracing::instrument(skip_all)]
async fn active_events_campaign_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::ActiveEventsCampaignIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::active_events::ActiveEvents,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    active_events_campaign_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().active_events_campaign_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::active_events::ActiveEventsCampaignIdGetResponse::Status200_ListOfAllActiveEventsForCampaignID
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::active_events::ActiveEventsCampaignIdGetResponse::Status404_CampaignNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn active_events_fraction_id_get_validation(
  path_params: models::ActiveEventsFractionIdGetPathParams,
) -> std::result::Result<(
  models::ActiveEventsFractionIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// ActiveEventsFractionIdGet - GET /active-events/fraction/{id}
#[tracing::instrument(skip_all)]
async fn active_events_fraction_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::ActiveEventsFractionIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::active_events::ActiveEvents,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    active_events_fraction_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().active_events_fraction_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::active_events::ActiveEventsFractionIdGetResponse::Status200_ListOfAllActiveEventsForFractionID
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::active_events::ActiveEventsFractionIdGetResponse::Status404_FractionNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn campaign_id_delete_validation(
  path_params: models::CampaignIdDeletePathParams,
) -> std::result::Result<(
  models::CampaignIdDeletePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// CampaignIdDelete - DELETE /campaign/{id}
#[tracing::instrument(skip_all)]
async fn campaign_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::CampaignIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::campaigns::Campaigns,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    campaign_id_delete_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().campaign_id_delete(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::campaigns::CampaignIdDeleteResponse::Status200_CampaignDeletedSuccessfully
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::campaigns::CampaignIdDeleteResponse::Status404_CampaignNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn campaign_id_get_validation(
  path_params: models::CampaignIdGetPathParams,
) -> std::result::Result<(
  models::CampaignIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// CampaignIdGet - GET /campaign/{id}
#[tracing::instrument(skip_all)]
async fn campaign_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::CampaignIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::campaigns::Campaigns,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    campaign_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().campaign_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::campaigns::CampaignIdGetResponse::Status200_CampaignRetrievedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::campaigns::CampaignIdGetResponse::Status404_CampaignNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::campaigns::CampaignIdGetResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct CampaignIdPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Campaign,
    }


#[tracing::instrument(skip_all)]
fn campaign_id_put_validation(
  path_params: models::CampaignIdPutPathParams,
        body: models::Campaign,
) -> std::result::Result<(
  models::CampaignIdPutPathParams,
        models::Campaign,
), ValidationErrors>
{
  path_params.validate()?;
              let b = CampaignIdPutBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
    body,
))
}
/// CampaignIdPut - PUT /campaign/{id}
#[tracing::instrument(skip_all)]
async fn campaign_id_put<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::CampaignIdPutPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::Campaign>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::campaigns::Campaigns,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    campaign_id_put_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().campaign_id_put(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::campaigns::CampaignIdPutResponse::Status200_CampaignUpdatedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::campaigns::CampaignIdPutResponse::Status404_CampaignNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::campaigns::CampaignIdPutResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::campaigns::CampaignIdPutResponse::Status500_ServerError
                                                => {
                                                  let mut response = response.status(500);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct CampaignPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Campaign,
    }


#[tracing::instrument(skip_all)]
fn campaign_post_validation(
        body: models::Campaign,
) -> std::result::Result<(
        models::Campaign,
), ValidationErrors>
{
              let b = CampaignPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// CampaignPost - POST /campaign
#[tracing::instrument(skip_all)]
async fn campaign_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::Campaign>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::campaigns::Campaigns,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    campaign_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().campaign_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::campaigns::CampaignPostResponse::Status201_CampaignCreatedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::campaigns::CampaignPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn campaigns_mega_campaign_id_get_validation(
  path_params: models::CampaignsMegaCampaignIdGetPathParams,
) -> std::result::Result<(
  models::CampaignsMegaCampaignIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// CampaignsMegaCampaignIdGet - GET /campaigns/mega-campaign/{id}
#[tracing::instrument(skip_all)]
async fn campaigns_mega_campaign_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::CampaignsMegaCampaignIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::campaigns::Campaigns,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    campaigns_mega_campaign_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().campaigns_mega_campaign_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::campaigns::CampaignsMegaCampaignIdGetResponse::Status200_CampaignsRetrievedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::campaigns::CampaignsMegaCampaignIdGetResponse::Status404_MegaCampaignNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::campaigns::CampaignsMegaCampaignIdGetResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn const_map_id_delete_validation(
  path_params: models::ConstMapIdDeletePathParams,
) -> std::result::Result<(
  models::ConstMapIdDeletePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// ConstMapIdDelete - DELETE /const_map/{id}
#[tracing::instrument(skip_all)]
async fn const_map_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::ConstMapIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::const_map::ConstMap,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    const_map_id_delete_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().const_map_id_delete(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::const_map::ConstMapIdDeleteResponse::Status200_ConstMapDeletedSuccessfully
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::const_map::ConstMapIdDeleteResponse::Status404_ConstMapNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn const_map_id_get_validation(
  path_params: models::ConstMapIdGetPathParams,
) -> std::result::Result<(
  models::ConstMapIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// ConstMapIdGet - GET /const_map/{id}
#[tracing::instrument(skip_all)]
async fn const_map_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::ConstMapIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::const_map::ConstMap,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    const_map_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().const_map_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::const_map::ConstMapIdGetResponse::Status200_ConstMapRetrievedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::const_map::ConstMapIdGetResponse::Status404_ConstMapNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::const_map::ConstMapIdGetResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct ConstMapIdPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::ConstMap,
    }


#[tracing::instrument(skip_all)]
fn const_map_id_put_validation(
  path_params: models::ConstMapIdPutPathParams,
        body: models::ConstMap,
) -> std::result::Result<(
  models::ConstMapIdPutPathParams,
        models::ConstMap,
), ValidationErrors>
{
  path_params.validate()?;
              let b = ConstMapIdPutBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
    body,
))
}
/// ConstMapIdPut - PUT /const_map/{id}
#[tracing::instrument(skip_all)]
async fn const_map_id_put<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::ConstMapIdPutPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::ConstMap>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::const_map::ConstMap,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    const_map_id_put_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().const_map_id_put(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::const_map::ConstMapIdPutResponse::Status200_ConstMapUpdatedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::const_map::ConstMapIdPutResponse::Status404_ConstMapNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::const_map::ConstMapIdPutResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::const_map::ConstMapIdPutResponse::Status500_ServerError
                                                => {
                                                  let mut response = response.status(500);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct ConstMapsPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::ConstMap,
    }


#[tracing::instrument(skip_all)]
fn const_maps_post_validation(
        body: models::ConstMap,
) -> std::result::Result<(
        models::ConstMap,
), ValidationErrors>
{
              let b = ConstMapsPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// ConstMapsPost - POST /const_maps
#[tracing::instrument(skip_all)]
async fn const_maps_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::ConstMap>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::const_map::ConstMap,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    const_maps_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().const_maps_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::const_map::ConstMapsPostResponse::Status201_ConstMapCreatedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::const_map::ConstMapsPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn data_map_id_delete_validation(
  path_params: models::DataMapIdDeletePathParams,
) -> std::result::Result<(
  models::DataMapIdDeletePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// DataMapIdDelete - DELETE /data-map/{id}
#[tracing::instrument(skip_all)]
async fn data_map_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::DataMapIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::data_map::DataMap,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    data_map_id_delete_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().data_map_id_delete(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::data_map::DataMapIdDeleteResponse::Status200_DataMapDeletedSuccessfully
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::data_map::DataMapIdDeleteResponse::Status404_DataMapNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn data_map_id_get_validation(
  path_params: models::DataMapIdGetPathParams,
) -> std::result::Result<(
  models::DataMapIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// DataMapIdGet - GET /data-map/{id}
#[tracing::instrument(skip_all)]
async fn data_map_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::DataMapIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::data_map::DataMap,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    data_map_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().data_map_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::data_map::DataMapIdGetResponse::Status200_DataMapRetrievedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::data_map::DataMapIdGetResponse::Status404_DataMapNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::data_map::DataMapIdGetResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct DataMapIdPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::DataMap,
    }


#[tracing::instrument(skip_all)]
fn data_map_id_put_validation(
  path_params: models::DataMapIdPutPathParams,
        body: models::DataMap,
) -> std::result::Result<(
  models::DataMapIdPutPathParams,
        models::DataMap,
), ValidationErrors>
{
  path_params.validate()?;
              let b = DataMapIdPutBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
    body,
))
}
/// DataMapIdPut - PUT /data-map/{id}
#[tracing::instrument(skip_all)]
async fn data_map_id_put<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::DataMapIdPutPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::DataMap>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::data_map::DataMap,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    data_map_id_put_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().data_map_id_put(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::data_map::DataMapIdPutResponse::Status200_DataMapUpdatedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::data_map::DataMapIdPutResponse::Status404_DataMapNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::data_map::DataMapIdPutResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::data_map::DataMapIdPutResponse::Status500_ServerError
                                                => {
                                                  let mut response = response.status(500);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct DataMapPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::DataMap,
    }


#[tracing::instrument(skip_all)]
fn data_map_post_validation(
        body: models::DataMap,
) -> std::result::Result<(
        models::DataMap,
), ValidationErrors>
{
              let b = DataMapPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// DataMapPost - POST /data-map
#[tracing::instrument(skip_all)]
async fn data_map_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::DataMap>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::data_map::DataMap,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    data_map_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().data_map_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::data_map::DataMapPostResponse::Status201_DataMapCreatedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::data_map::DataMapPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn faction_marker_fraction_id_get_validation(
  path_params: models::FactionMarkerFractionIdGetPathParams,
) -> std::result::Result<(
  models::FactionMarkerFractionIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// FactionMarkerFractionIdGet - GET /faction-marker/fraction/{id}
#[tracing::instrument(skip_all)]
async fn faction_marker_fraction_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::FactionMarkerFractionIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::faction_marker::FactionMarker,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    faction_marker_fraction_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().faction_marker_fraction_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::faction_marker::FactionMarkerFractionIdGetResponse::Status200_ListOfAllFactionMarkers
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::faction_marker::FactionMarkerFractionIdGetResponse::Status404_FractionNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn faction_marker_id_delete_validation(
  path_params: models::FactionMarkerIdDeletePathParams,
) -> std::result::Result<(
  models::FactionMarkerIdDeletePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// FactionMarkerIdDelete - DELETE /faction-marker/{id}
#[tracing::instrument(skip_all)]
async fn faction_marker_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::FactionMarkerIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::faction_marker::FactionMarker,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    faction_marker_id_delete_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().faction_marker_id_delete(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::faction_marker::FactionMarkerIdDeleteResponse::Status204_FactionMarkerDeletedSuccessfully
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::faction_marker::FactionMarkerIdDeleteResponse::Status404_FactionMarkerNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct FactionMarkerPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::FactionMarker,
    }


#[tracing::instrument(skip_all)]
fn faction_marker_post_validation(
        body: models::FactionMarker,
) -> std::result::Result<(
        models::FactionMarker,
), ValidationErrors>
{
              let b = FactionMarkerPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// FactionMarkerPost - POST /faction-marker
#[tracing::instrument(skip_all)]
async fn faction_marker_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::FactionMarker>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::faction_marker::FactionMarker,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    faction_marker_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().faction_marker_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::faction_marker::FactionMarkerPostResponse::Status201_FactionMarkerCreated
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::faction_marker::FactionMarkerPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn faction_player_fraction_id_get_validation(
  path_params: models::FactionPlayerFractionIdGetPathParams,
) -> std::result::Result<(
  models::FactionPlayerFractionIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// FactionPlayerFractionIdGet - GET /faction-player/fraction/{id}
#[tracing::instrument(skip_all)]
async fn faction_player_fraction_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::FactionPlayerFractionIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::faction_player::FactionPlayer,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    faction_player_fraction_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().faction_player_fraction_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::faction_player::FactionPlayerFractionIdGetResponse::Status200_ListOfAllFactionPlayersForFractionId
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::faction_player::FactionPlayerFractionIdGetResponse::Status404_FractionNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct FactionPlayerPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::FactionPlayer,
    }


#[tracing::instrument(skip_all)]
fn faction_player_post_validation(
        body: models::FactionPlayer,
) -> std::result::Result<(
        models::FactionPlayer,
), ValidationErrors>
{
              let b = FactionPlayerPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// FactionPlayerPost - POST /faction-player
#[tracing::instrument(skip_all)]
async fn faction_player_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::FactionPlayer>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::faction_player::FactionPlayer,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    faction_player_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().faction_player_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::faction_player::FactionPlayerPostResponse::Status201_FactionPlayerCreated
                                                => {
                                                  let mut response = response.status(201);
                                                  response.body(Body::empty())
                                                },
                                                apis::faction_player::FactionPlayerPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn faction_players_id_delete_validation(
  path_params: models::FactionPlayersIdDeletePathParams,
) -> std::result::Result<(
  models::FactionPlayersIdDeletePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// FactionPlayersIdDelete - DELETE /faction-players/{id}
#[tracing::instrument(skip_all)]
async fn faction_players_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::FactionPlayersIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::faction_player::FactionPlayer,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    faction_players_id_delete_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().faction_players_id_delete(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::faction_player::FactionPlayersIdDeleteResponse::Status204_FactionPlayerDeleted
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::faction_player::FactionPlayersIdDeleteResponse::Status404_FactionPlayerNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn faction_players_id_get_validation(
  path_params: models::FactionPlayersIdGetPathParams,
) -> std::result::Result<(
  models::FactionPlayersIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// FactionPlayersIdGet - GET /faction-players/{id}
#[tracing::instrument(skip_all)]
async fn faction_players_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::FactionPlayersIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::faction_player::FactionPlayer,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    faction_players_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().faction_players_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::faction_player::FactionPlayersIdGetResponse::Status200_FactionPlayerData
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::faction_player::FactionPlayersIdGetResponse::Status404_FactionPlayerNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct FactionPlayersIdPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::FactionPlayer,
    }


#[tracing::instrument(skip_all)]
fn faction_players_id_put_validation(
  path_params: models::FactionPlayersIdPutPathParams,
        body: models::FactionPlayer,
) -> std::result::Result<(
  models::FactionPlayersIdPutPathParams,
        models::FactionPlayer,
), ValidationErrors>
{
  path_params.validate()?;
              let b = FactionPlayersIdPutBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
    body,
))
}
/// FactionPlayersIdPut - PUT /faction-players/{id}
#[tracing::instrument(skip_all)]
async fn faction_players_id_put<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::FactionPlayersIdPutPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::FactionPlayer>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::faction_player::FactionPlayer,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    faction_players_id_put_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().faction_players_id_put(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::faction_player::FactionPlayersIdPutResponse::Status200_FactionPlayerUpdated
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::faction_player::FactionPlayersIdPutResponse::Status404_FactionPlayerNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn fraction_id_delete_validation(
  path_params: models::FractionIdDeletePathParams,
) -> std::result::Result<(
  models::FractionIdDeletePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// FractionIdDelete - DELETE /fraction/{id}
#[tracing::instrument(skip_all)]
async fn fraction_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::FractionIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::fraction::Fraction,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    fraction_id_delete_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().fraction_id_delete(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::fraction::FractionIdDeleteResponse::Status204_FractionDeleted
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::fraction::FractionIdDeleteResponse::Status404_FractionNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn fraction_id_get_validation(
  path_params: models::FractionIdGetPathParams,
) -> std::result::Result<(
  models::FractionIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// FractionIdGet - GET /fraction/{id}
#[tracing::instrument(skip_all)]
async fn fraction_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::FractionIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::fraction::Fraction,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    fraction_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().fraction_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::fraction::FractionIdGetResponse::Status200_FractionData
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::fraction::FractionIdGetResponse::Status404_FractionNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct FractionIdPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Fraction,
    }


#[tracing::instrument(skip_all)]
fn fraction_id_put_validation(
  path_params: models::FractionIdPutPathParams,
        body: models::Fraction,
) -> std::result::Result<(
  models::FractionIdPutPathParams,
        models::Fraction,
), ValidationErrors>
{
  path_params.validate()?;
              let b = FractionIdPutBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
    body,
))
}
/// FractionIdPut - PUT /fraction/{id}
#[tracing::instrument(skip_all)]
async fn fraction_id_put<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::FractionIdPutPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::Fraction>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::fraction::Fraction,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    fraction_id_put_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().fraction_id_put(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::fraction::FractionIdPutResponse::Status200_FractionUpdated
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::fraction::FractionIdPutResponse::Status404_FractionNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct FractionPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Fraction,
    }


#[tracing::instrument(skip_all)]
fn fraction_post_validation(
        body: models::Fraction,
) -> std::result::Result<(
        models::Fraction,
), ValidationErrors>
{
              let b = FractionPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// FractionPost - POST /fraction
#[tracing::instrument(skip_all)]
async fn fraction_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::Fraction>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::fraction::Fraction,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    fraction_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().fraction_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::fraction::FractionPostResponse::Status201_FractionCreated
                                                => {
                                                  let mut response = response.status(201);
                                                  response.body(Body::empty())
                                                },
                                                apis::fraction::FractionPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn fractions_campaign_id_get_validation(
  path_params: models::FractionsCampaignIdGetPathParams,
) -> std::result::Result<(
  models::FractionsCampaignIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// FractionsCampaignIdGet - GET /fractions/campaign/{id}
#[tracing::instrument(skip_all)]
async fn fractions_campaign_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::FractionsCampaignIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::fraction::Fraction,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    fractions_campaign_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().fractions_campaign_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::fraction::FractionsCampaignIdGetResponse::Status200_ListOfAllFractionsForCampaign
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::fraction::FractionsCampaignIdGetResponse::Status404_CampaignNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn match_events_fraction_id_get_validation(
  path_params: models::MatchEventsFractionIdGetPathParams,
) -> std::result::Result<(
  models::MatchEventsFractionIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// MatchEventsFractionIdGet - GET /match-events/fraction/{id}
#[tracing::instrument(skip_all)]
async fn match_events_fraction_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::MatchEventsFractionIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::match_event::MatchEvent,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    match_events_fraction_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().match_events_fraction_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::match_event::MatchEventsFractionIdGetResponse::Status200_ListOfAllMatchEventsForFraction
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::match_event::MatchEventsFractionIdGetResponse::Status404_FractionNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn match_events_id_delete_validation(
  path_params: models::MatchEventsIdDeletePathParams,
) -> std::result::Result<(
  models::MatchEventsIdDeletePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// MatchEventsIdDelete - DELETE /match-events/{id}
#[tracing::instrument(skip_all)]
async fn match_events_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::MatchEventsIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::match_event::MatchEvent,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    match_events_id_delete_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().match_events_id_delete(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::match_event::MatchEventsIdDeleteResponse::Status204_MatchEventDeletedSuccessfully
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn match_events_id_get_validation(
  path_params: models::MatchEventsIdGetPathParams,
) -> std::result::Result<(
  models::MatchEventsIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// MatchEventsIdGet - GET /match-events/{id}
#[tracing::instrument(skip_all)]
async fn match_events_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::MatchEventsIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::match_event::MatchEvent,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    match_events_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().match_events_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::match_event::MatchEventsIdGetResponse::Status200_ListOfAllMatchEvents
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn match_events_player_id_get_validation(
  path_params: models::MatchEventsPlayerIdGetPathParams,
) -> std::result::Result<(
  models::MatchEventsPlayerIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// MatchEventsPlayerIdGet - GET /match-events/player/{id}
#[tracing::instrument(skip_all)]
async fn match_events_player_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::MatchEventsPlayerIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::match_event::MatchEvent,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    match_events_player_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().match_events_player_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::match_event::MatchEventsPlayerIdGetResponse::Status200_ListOfAllMatchEventsForPlayer
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::match_event::MatchEventsPlayerIdGetResponse::Status404_PlayerNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct MatchEventsPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::MatchEvent,
    }


#[tracing::instrument(skip_all)]
fn match_events_post_validation(
        body: models::MatchEvent,
) -> std::result::Result<(
        models::MatchEvent,
), ValidationErrors>
{
              let b = MatchEventsPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// MatchEventsPost - POST /match-events
#[tracing::instrument(skip_all)]
async fn match_events_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::MatchEvent>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::match_event::MatchEvent,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    match_events_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().match_events_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::match_event::MatchEventsPostResponse::Status201_MatchEventCreatedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::match_event::MatchEventsPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn mega_campain_id_delete_validation(
  path_params: models::MegaCampainIdDeletePathParams,
) -> std::result::Result<(
  models::MegaCampainIdDeletePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// MegaCampainIdDelete - DELETE /mega-campain/{id}
#[tracing::instrument(skip_all)]
async fn mega_campain_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::MegaCampainIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::mega_campaign::MegaCampaign,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    mega_campain_id_delete_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().mega_campain_id_delete(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::mega_campaign::MegaCampainIdDeleteResponse::Status200_MegaCampaignDeletedSuccessfully
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::mega_campaign::MegaCampainIdDeleteResponse::Status404_MegaCampaignNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct MegaCampainIdPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::MegaCampaign,
    }


#[tracing::instrument(skip_all)]
fn mega_campain_id_put_validation(
  path_params: models::MegaCampainIdPutPathParams,
        body: models::MegaCampaign,
) -> std::result::Result<(
  models::MegaCampainIdPutPathParams,
        models::MegaCampaign,
), ValidationErrors>
{
  path_params.validate()?;
              let b = MegaCampainIdPutBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
    body,
))
}
/// MegaCampainIdPut - PUT /mega-campain/{id}
#[tracing::instrument(skip_all)]
async fn mega_campain_id_put<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::MegaCampainIdPutPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::MegaCampaign>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::mega_campaign::MegaCampaign,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    mega_campain_id_put_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().mega_campain_id_put(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::mega_campaign::MegaCampainIdPutResponse::Status200_MegaCampaignUpdatedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::mega_campaign::MegaCampainIdPutResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                                apis::mega_campaign::MegaCampainIdPutResponse::Status404_MegaCampaignNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::mega_campaign::MegaCampainIdPutResponse::Status500_ServerError
                                                => {
                                                  let mut response = response.status(500);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct MegaCampainPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::MegaCampaign,
    }


#[tracing::instrument(skip_all)]
fn mega_campain_post_validation(
        body: models::MegaCampaign,
) -> std::result::Result<(
        models::MegaCampaign,
), ValidationErrors>
{
              let b = MegaCampainPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// MegaCampainPost - POST /mega-campain
#[tracing::instrument(skip_all)]
async fn mega_campain_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::MegaCampaign>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::mega_campaign::MegaCampaign,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    mega_campain_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().mega_campain_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::mega_campaign::MegaCampainPostResponse::Status201_MegaCampaignCreatedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::mega_campaign::MegaCampainPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn mega_campains_for_player_id_get_validation(
  path_params: models::MegaCampainsForPlayerIdGetPathParams,
) -> std::result::Result<(
  models::MegaCampainsForPlayerIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// MegaCampainsForPlayerIdGet - GET /mega-campains/for-player/{id}
#[tracing::instrument(skip_all)]
async fn mega_campains_for_player_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::MegaCampainsForPlayerIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::mega_campaigns::MegaCampaigns,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    mega_campains_for_player_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().mega_campains_for_player_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::mega_campaigns::MegaCampainsForPlayerIdGetResponse::Status200_AListOfMegaCampaigns
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::mega_campaigns::MegaCampainsForPlayerIdGetResponse::Status500_ServerError
                                                => {
                                                  let mut response = response.status(500);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn mega_campains_get_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}
/// MegaCampainsGet - GET /mega-campains
#[tracing::instrument(skip_all)]
async fn mega_campains_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::mega_campaigns::MegaCampaigns,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    mega_campains_get_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().mega_campains_get(
      method,
      host,
      cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::mega_campaigns::MegaCampainsGetResponse::Status200_AListOfMegaCampaigns
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::mega_campaigns::MegaCampainsGetResponse::Status500_ServerError
                                                => {
                                                  let mut response = response.status(500);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn mega_campains_open_get_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}
/// MegaCampainsOpenGet - GET /mega-campains/open
#[tracing::instrument(skip_all)]
async fn mega_campains_open_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::mega_campaigns::MegaCampaigns,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    mega_campains_open_get_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().mega_campains_open_get(
      method,
      host,
      cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::mega_campaigns::MegaCampainsOpenGetResponse::Status200_AListOfMegaCampaigns
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::mega_campaigns::MegaCampainsOpenGetResponse::Status500_ServerError
                                                => {
                                                  let mut response = response.status(500);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn news_events_campaign_id_get_validation(
  path_params: models::NewsEventsCampaignIdGetPathParams,
) -> std::result::Result<(
  models::NewsEventsCampaignIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// NewsEventsCampaignIdGet - GET /news-events/campaign/{id}
#[tracing::instrument(skip_all)]
async fn news_events_campaign_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::NewsEventsCampaignIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::news_events::NewsEvents,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    news_events_campaign_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().news_events_campaign_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::news_events::NewsEventsCampaignIdGetResponse::Status200_CampaignsRetrievedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::news_events::NewsEventsCampaignIdGetResponse::Status404_CampainNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                                apis::news_events::NewsEventsCampaignIdGetResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn news_events_delete_validation(
  query_params: models::NewsEventsDeleteQueryParams,
) -> std::result::Result<(
  models::NewsEventsDeleteQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}
/// NewsEventsDelete - DELETE /news-events
#[tracing::instrument(skip_all)]
async fn news_events_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<models::NewsEventsDeleteQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::news_events::NewsEvents,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    news_events_delete_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().news_events_delete(
      method,
      host,
      cookies,
        query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::news_events::NewsEventsDeleteResponse::Status200_NewsEventDeletedSuccessfully
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::news_events::NewsEventsDeleteResponse::Status404_NewsEventNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct NewsEventsPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::NewsEvent,
    }


#[tracing::instrument(skip_all)]
fn news_events_post_validation(
        body: models::NewsEvent,
) -> std::result::Result<(
        models::NewsEvent,
), ValidationErrors>
{
              let b = NewsEventsPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// NewsEventsPost - POST /news-events
#[tracing::instrument(skip_all)]
async fn news_events_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::NewsEvent>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::news_events::NewsEvents,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    news_events_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().news_events_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::news_events::NewsEventsPostResponse::Status201_NewsEventCreatedSuccessfully
                                                    (body)
                                                => {
                                                  let mut response = response.status(201);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::news_events::NewsEventsPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn player_id_delete_validation(
  path_params: models::PlayerIdDeletePathParams,
) -> std::result::Result<(
  models::PlayerIdDeletePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// PlayerIdDelete - DELETE /player/{id}
#[tracing::instrument(skip_all)]
async fn player_id_delete<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::PlayerIdDeletePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::player::Player,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    player_id_delete_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().player_id_delete(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::player::PlayerIdDeleteResponse::Status204_PlayerDeleted
                                                => {
                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                                apis::player::PlayerIdDeleteResponse::Status404_PlayerNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn player_id_get_validation(
  path_params: models::PlayerIdGetPathParams,
) -> std::result::Result<(
  models::PlayerIdGetPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}
/// PlayerIdGet - GET /player/{id}
#[tracing::instrument(skip_all)]
async fn player_id_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::PlayerIdGetPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::player::Player,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    player_id_get_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().player_id_get(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::player::PlayerIdGetResponse::Status200_PlayerData
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                                apis::player::PlayerIdGetResponse::Status404_PlayerNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PlayerIdPutBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Player,
    }


#[tracing::instrument(skip_all)]
fn player_id_put_validation(
  path_params: models::PlayerIdPutPathParams,
        body: models::Player,
) -> std::result::Result<(
  models::PlayerIdPutPathParams,
        models::Player,
), ValidationErrors>
{
  path_params.validate()?;
              let b = PlayerIdPutBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
    body,
))
}
/// PlayerIdPut - PUT /player/{id}
#[tracing::instrument(skip_all)]
async fn player_id_put<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::PlayerIdPutPathParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::Player>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::player::Player,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    player_id_put_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().player_id_put(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::player::PlayerIdPutResponse::Status200_PlayerUpdated
                                                => {
                                                  let mut response = response.status(200);
                                                  response.body(Body::empty())
                                                },
                                                apis::player::PlayerIdPutResponse::Status404_PlayerNotFound
                                                => {
                                                  let mut response = response.status(404);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct PlayerPostBodyValidator<'a> {
            #[validate(nested)]
          body: &'a models::Player,
    }


#[tracing::instrument(skip_all)]
fn player_post_validation(
        body: models::Player,
) -> std::result::Result<(
        models::Player,
), ValidationErrors>
{
              let b = PlayerPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}
/// PlayerPost - POST /player
#[tracing::instrument(skip_all)]
async fn player_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::Player>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::player::Player,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    player_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().player_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::player::PlayerPostResponse::Status201_PlayerCreated
                                                => {
                                                  let mut response = response.status(201);
                                                  response.body(Body::empty())
                                                },
                                                apis::player::PlayerPostResponse::Status400_InvalidInput
                                                => {
                                                  let mut response = response.status(400);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn players_get_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}
/// PlayersGet - GET /players
#[tracing::instrument(skip_all)]
async fn players_get<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where
    I: AsRef<A> + Send + Sync,
    A: apis::player::Player,
{


      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move ||
    players_get_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST);
  };

  let result = api_impl.as_ref().players_get(
      method,
      host,
      cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                apis::player::PlayersGetResponse::Status200_ListOfAllPlayers
                                                    (body)
                                                => {
                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

