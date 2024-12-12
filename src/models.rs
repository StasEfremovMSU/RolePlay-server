#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ActiveEventsIdDeletePathParams {
            /// The ID of the event
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ActiveEventsIdGetPathParams {
            /// The ID of the event
                pub id: i32,
    }


      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ActiveEventsCampaignIdGetPathParams {
            /// The ID of the campaign
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ActiveEventsFractionIdGetPathParams {
            /// The ID of the fraction
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct CampaignIdDeletePathParams {
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct CampaignIdGetPathParams {
            /// The ID of the campaign to retrieve.
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct CampaignIdPutPathParams {
            /// The ID of the campaign to update.
                pub id: i32,
    }


      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct CampaignsMegaCampaignIdGetPathParams {
            /// The ID of the mega campaign to retrieve its list campains.
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ConstMapIdDeletePathParams {
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ConstMapIdGetPathParams {
            /// The ID of the const map to retrieve.
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct ConstMapIdPutPathParams {
            /// The ID of the Const map to update.
                pub id: i32,
    }


      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct DataMapIdDeletePathParams {
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct DataMapIdGetPathParams {
            /// The ID of the data map to retrieve.
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct DataMapIdPutPathParams {
            /// The ID of the Data map to update.
                pub id: i32,
    }


      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct FactionMarkerFractionIdGetPathParams {
            /// The ID of the fraction
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct FactionMarkerIdDeletePathParams {
            /// The ID of the faction marker to delete.
                pub id: i32,
    }


      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct FactionPlayerFractionIdGetPathParams {
            /// The ID of the fraction
                pub id: i32,
    }


      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct FactionPlayersIdDeletePathParams {
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct FactionPlayersIdGetPathParams {
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct FactionPlayersIdPutPathParams {
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct FractionIdDeletePathParams {
            /// The ID of the fraction
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct FractionIdGetPathParams {
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct FractionIdPutPathParams {
            /// The ID of the fraction
                pub id: i32,
    }


      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct FractionsCampaignIdGetPathParams {
            /// The ID of the fraction
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct MatchEventsFractionIdGetPathParams {
            /// The ID of the fraction
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct MatchEventsIdDeletePathParams {
            /// ID of the match event to delete
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct MatchEventsIdGetPathParams {
            /// ID of the match event to get
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct MatchEventsPlayerIdGetPathParams {
            /// The ID of the player
                pub id: i32,
    }


      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct MegaCampainIdDeletePathParams {
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct MegaCampainIdPutPathParams {
            /// The ID of the mega campaign to update.
                pub id: i32,
    }


      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct MegaCampainsForPlayerIdGetPathParams {
            /// The ID of the player wich campains we want.
                pub id: i32,
    }


      
      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct NewsEventsCampaignIdGetPathParams {
            /// The ID of the campaign to retrieve its list news.
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct NewsEventsDeleteQueryParams {
                #[serde(rename = "id")]
                pub id: i32,
    }

      
      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct PlayerIdDeletePathParams {
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct PlayerIdGetPathParams {
                pub id: i32,
    }


      
    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))] 
    pub struct PlayerIdPutPathParams {
                pub id: i32,
    }


      
      


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ActiveEvent {
/// Unique identifier for the active event.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,

/// Reference to the associated DataMap. Cascade deletion on DataMap removal.
    #[serde(rename = "data_map_id")]
    pub data_map_id: i32,

/// Reference to the creator player. Set to NULL if the player is deleted.
    #[serde(rename = "creator_player_id")]
    pub creator_player_id: i32,

/// Reference to the associated faction. Cascade deletion on faction removal.
    #[serde(rename = "faction_id")]
    pub faction_id: i32,

/// Type of the event. Must be a positive integer.
    #[serde(rename = "event_type")]
    #[validate(
            range(min = 1),
        )]
    pub event_type: u32,

/// Indicates whether the event is visible to all factions.
    #[serde(rename = "is_visible_to_all")]
    pub is_visible_to_all: bool,

/// Start time of the event. Defaults to the current timestamp.
    #[serde(rename = "start_time")]
    pub start_time: chrono::DateTime::<chrono::Utc>,

/// Starting X coordinate. Must be non-negative.
    #[serde(rename = "start_x")]
    #[validate(
            range(min = 0),
        )]
    pub start_x: u32,

/// Starting Y coordinate. Must be non-negative.
    #[serde(rename = "start_y")]
    #[validate(
            range(min = 0),
        )]
    pub start_y: u32,

/// Starting Z coordinate. Must be non-negative.
    #[serde(rename = "start_z")]
    #[validate(
            range(min = 0),
        )]
    pub start_z: u32,

/// Ending X coordinate. Must be non-negative.
    #[serde(rename = "end_x")]
    #[validate(
            range(min = 0),
        )]
    pub end_x: u32,

/// Ending Y coordinate. Must be non-negative.
    #[serde(rename = "end_y")]
    #[validate(
            range(min = 0),
        )]
    pub end_y: u32,

/// Ending Z coordinate. Must be non-negative.
    #[serde(rename = "end_z")]
    #[validate(
            range(min = 0),
        )]
    pub end_z: u32,

}


impl ActiveEvent {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(data_map_id: i32, creator_player_id: i32, faction_id: i32, event_type: u32, is_visible_to_all: bool, start_time: chrono::DateTime::<chrono::Utc>, start_x: u32, start_y: u32, start_z: u32, end_x: u32, end_y: u32, end_z: u32, ) -> ActiveEvent {
        ActiveEvent {
            id: None,
            data_map_id,
            creator_player_id,
            faction_id,
            event_type,
            is_visible_to_all,
            start_time,
            start_x,
            start_y,
            start_z,
            end_x,
            end_y,
            end_z,
        }
    }
}

/// Converts the ActiveEvent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ActiveEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            Some("data_map_id".to_string()),
            Some(self.data_map_id.to_string()),


            Some("creator_player_id".to_string()),
            Some(self.creator_player_id.to_string()),


            Some("faction_id".to_string()),
            Some(self.faction_id.to_string()),


            Some("event_type".to_string()),
            Some(self.event_type.to_string()),


            Some("is_visible_to_all".to_string()),
            Some(self.is_visible_to_all.to_string()),

            // Skipping start_time in query parameter serialization


            Some("start_x".to_string()),
            Some(self.start_x.to_string()),


            Some("start_y".to_string()),
            Some(self.start_y.to_string()),


            Some("start_z".to_string()),
            Some(self.start_z.to_string()),


            Some("end_x".to_string()),
            Some(self.end_x.to_string()),


            Some("end_y".to_string()),
            Some(self.end_y.to_string()),


            Some("end_z".to_string()),
            Some(self.end_z.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ActiveEvent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ActiveEvent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub data_map_id: Vec<i32>,
            pub creator_player_id: Vec<i32>,
            pub faction_id: Vec<i32>,
            pub event_type: Vec<u32>,
            pub is_visible_to_all: Vec<bool>,
            pub start_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub start_x: Vec<u32>,
            pub start_y: Vec<u32>,
            pub start_z: Vec<u32>,
            pub end_x: Vec<u32>,
            pub end_y: Vec<u32>,
            pub end_z: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ActiveEvent".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "data_map_id" => intermediate_rep.data_map_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "creator_player_id" => intermediate_rep.creator_player_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "faction_id" => intermediate_rep.faction_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "event_type" => intermediate_rep.event_type.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "is_visible_to_all" => intermediate_rep.is_visible_to_all.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "start_time" => intermediate_rep.start_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "start_x" => intermediate_rep.start_x.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "start_y" => intermediate_rep.start_y.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "start_z" => intermediate_rep.start_z.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "end_x" => intermediate_rep.end_x.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "end_y" => intermediate_rep.end_y.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "end_z" => intermediate_rep.end_z.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ActiveEvent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ActiveEvent {
            id: intermediate_rep.id.into_iter().next(),
            data_map_id: intermediate_rep.data_map_id.into_iter().next().ok_or_else(|| "data_map_id missing in ActiveEvent".to_string())?,
            creator_player_id: intermediate_rep.creator_player_id.into_iter().next().ok_or_else(|| "creator_player_id missing in ActiveEvent".to_string())?,
            faction_id: intermediate_rep.faction_id.into_iter().next().ok_or_else(|| "faction_id missing in ActiveEvent".to_string())?,
            event_type: intermediate_rep.event_type.into_iter().next().ok_or_else(|| "event_type missing in ActiveEvent".to_string())?,
            is_visible_to_all: intermediate_rep.is_visible_to_all.into_iter().next().ok_or_else(|| "is_visible_to_all missing in ActiveEvent".to_string())?,
            start_time: intermediate_rep.start_time.into_iter().next().ok_or_else(|| "start_time missing in ActiveEvent".to_string())?,
            start_x: intermediate_rep.start_x.into_iter().next().ok_or_else(|| "start_x missing in ActiveEvent".to_string())?,
            start_y: intermediate_rep.start_y.into_iter().next().ok_or_else(|| "start_y missing in ActiveEvent".to_string())?,
            start_z: intermediate_rep.start_z.into_iter().next().ok_or_else(|| "start_z missing in ActiveEvent".to_string())?,
            end_x: intermediate_rep.end_x.into_iter().next().ok_or_else(|| "end_x missing in ActiveEvent".to_string())?,
            end_y: intermediate_rep.end_y.into_iter().next().ok_or_else(|| "end_y missing in ActiveEvent".to_string())?,
            end_z: intermediate_rep.end_z.into_iter().next().ok_or_else(|| "end_z missing in ActiveEvent".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ActiveEvent> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ActiveEvent>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ActiveEvent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ActiveEvent - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ActiveEvent> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ActiveEvent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ActiveEvent - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Campaign {
/// Unique identifier for the campaign.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,

/// Name of the campaign, required and non-empty.
    #[serde(rename = "name")]
    #[validate(
            length(min = 1, max = 255),
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

/// Detailed description of the campaign.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}


impl Campaign {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Campaign {
        Campaign {
            id: None,
            name: None,
            description: None,
        }
    }
}

/// Converts the Campaign value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Campaign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.description.as_ref().map(|description| {
                [
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Campaign value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Campaign {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub name: Vec<String>,
            pub description: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Campaign".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Campaign".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Campaign {
            id: intermediate_rep.id.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Campaign> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Campaign>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Campaign>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Campaign - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Campaign> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Campaign as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Campaign - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ChangeCampain {
/// Unique identifier for the change campaign.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,

/// ID of the faction, must be positive.
    #[serde(rename = "faction_id")]
    #[validate(
            range(min = 1),
        )]
    pub faction_id: u32,

/// ID of the associated campaign.
    #[serde(rename = "campaign_id")]
    pub campaign_id: i32,

/// Type of the event, must be positive.
    #[serde(rename = "event_type")]
    #[validate(
            range(min = 1),
        )]
    pub event_type: u32,

/// Time of the change, defaults to the current timestamp.
    #[serde(rename = "change_time")]
    pub change_time: chrono::DateTime::<chrono::Utc>,

/// Starting X coordinate, must be non-negative.
    #[serde(rename = "start_x")]
    #[validate(
            range(min = 0),
        )]
    pub start_x: u32,

/// Starting Y coordinate, must be non-negative.
    #[serde(rename = "start_y")]
    #[validate(
            range(min = 0),
        )]
    pub start_y: u32,

/// Starting Z coordinate, must be non-negative.
    #[serde(rename = "start_z")]
    #[validate(
            range(min = 0),
        )]
    pub start_z: u32,

/// Optional type, must be non-negative if specified.
    #[serde(rename = "type")]
    #[validate(
            range(min = 0),
        )]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub r#type: Option<Nullable<u32>>,

}


impl ChangeCampain {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(faction_id: u32, campaign_id: i32, event_type: u32, change_time: chrono::DateTime::<chrono::Utc>, start_x: u32, start_y: u32, start_z: u32, ) -> ChangeCampain {
        ChangeCampain {
            id: None,
            faction_id,
            campaign_id,
            event_type,
            change_time,
            start_x,
            start_y,
            start_z,
            r#type: None,
        }
    }
}

/// Converts the ChangeCampain value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ChangeCampain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            Some("faction_id".to_string()),
            Some(self.faction_id.to_string()),


            Some("campaign_id".to_string()),
            Some(self.campaign_id.to_string()),


            Some("event_type".to_string()),
            Some(self.event_type.to_string()),

            // Skipping change_time in query parameter serialization


            Some("start_x".to_string()),
            Some(self.start_x.to_string()),


            Some("start_y".to_string()),
            Some(self.start_y.to_string()),


            Some("start_z".to_string()),
            Some(self.start_z.to_string()),


            self.r#type.as_ref().map(|r#type| {
                [
                    "type".to_string(),
                    r#type.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ChangeCampain value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ChangeCampain {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub faction_id: Vec<u32>,
            pub campaign_id: Vec<i32>,
            pub event_type: Vec<u32>,
            pub change_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub start_x: Vec<u32>,
            pub start_y: Vec<u32>,
            pub start_z: Vec<u32>,
            pub r#type: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ChangeCampain".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "faction_id" => intermediate_rep.faction_id.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "campaign_id" => intermediate_rep.campaign_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "event_type" => intermediate_rep.event_type.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "change_time" => intermediate_rep.change_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "start_x" => intermediate_rep.start_x.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "start_y" => intermediate_rep.start_y.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "start_z" => intermediate_rep.start_z.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "type" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in ChangeCampain".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ChangeCampain".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ChangeCampain {
            id: intermediate_rep.id.into_iter().next(),
            faction_id: intermediate_rep.faction_id.into_iter().next().ok_or_else(|| "faction_id missing in ChangeCampain".to_string())?,
            campaign_id: intermediate_rep.campaign_id.into_iter().next().ok_or_else(|| "campaign_id missing in ChangeCampain".to_string())?,
            event_type: intermediate_rep.event_type.into_iter().next().ok_or_else(|| "event_type missing in ChangeCampain".to_string())?,
            change_time: intermediate_rep.change_time.into_iter().next().ok_or_else(|| "change_time missing in ChangeCampain".to_string())?,
            start_x: intermediate_rep.start_x.into_iter().next().ok_or_else(|| "start_x missing in ChangeCampain".to_string())?,
            start_y: intermediate_rep.start_y.into_iter().next().ok_or_else(|| "start_y missing in ChangeCampain".to_string())?,
            start_z: intermediate_rep.start_z.into_iter().next().ok_or_else(|| "start_z missing in ChangeCampain".to_string())?,
            r#type: std::result::Result::Err("Nullable types not supported in ChangeCampain".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ChangeCampain> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ChangeCampain>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ChangeCampain>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ChangeCampain - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ChangeCampain> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ChangeCampain as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ChangeCampain - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ConstMap {
/// Unique identifier for the constant map.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,

/// ID of the associated campaign. The map is deleted if the campaign is removed.
    #[serde(rename = "campaign_id")]
    pub campaign_id: i32,

/// File associated with the map, must not be empty.
    #[serde(rename = "file")]
    pub file: ByteArray,

}


impl ConstMap {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(campaign_id: i32, file: ByteArray, ) -> ConstMap {
        ConstMap {
            id: None,
            campaign_id,
            file,
        }
    }
}

/// Converts the ConstMap value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ConstMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            Some("campaign_id".to_string()),
            Some(self.campaign_id.to_string()),

            // Skipping file in query parameter serialization
            // Skipping file in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ConstMap value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ConstMap {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub campaign_id: Vec<i32>,
            pub file: Vec<ByteArray>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ConstMap".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "campaign_id" => intermediate_rep.campaign_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "file" => return std::result::Result::Err("Parsing binary data in this style is not supported in ConstMap".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ConstMap".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ConstMap {
            id: intermediate_rep.id.into_iter().next(),
            campaign_id: intermediate_rep.campaign_id.into_iter().next().ok_or_else(|| "campaign_id missing in ConstMap".to_string())?,
            file: intermediate_rep.file.into_iter().next().ok_or_else(|| "file missing in ConstMap".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ConstMap> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ConstMap>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ConstMap>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ConstMap - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ConstMap> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ConstMap as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ConstMap - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataMap {
/// Unique identifier for the data map.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,

/// Unique ID of the associated constant map. This is a one-to-one relationship. The data map is deleted if the constant map is removed.
    #[serde(rename = "const_map_id")]
    pub const_map_id: i32,

/// File associated with the data map, must not be empty.
    #[serde(rename = "file")]
    pub file: ByteArray,

}


impl DataMap {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(const_map_id: i32, file: ByteArray, ) -> DataMap {
        DataMap {
            id: None,
            const_map_id,
            file,
        }
    }
}

/// Converts the DataMap value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for DataMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            Some("const_map_id".to_string()),
            Some(self.const_map_id.to_string()),

            // Skipping file in query parameter serialization
            // Skipping file in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataMap value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataMap {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub const_map_id: Vec<i32>,
            pub file: Vec<ByteArray>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DataMap".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "const_map_id" => intermediate_rep.const_map_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "file" => return std::result::Result::Err("Parsing binary data in this style is not supported in DataMap".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataMap".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataMap {
            id: intermediate_rep.id.into_iter().next(),
            const_map_id: intermediate_rep.const_map_id.into_iter().next().ok_or_else(|| "const_map_id missing in DataMap".to_string())?,
            file: intermediate_rep.file.into_iter().next().ok_or_else(|| "file missing in DataMap".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataMap> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<DataMap>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DataMap>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DataMap - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<DataMap> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DataMap as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DataMap - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FactionMarker {
/// Unique identifier for the faction marker.
    #[serde(rename = "marker_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker_id: Option<i32>,

/// ID of the associated faction.
    #[serde(rename = "faction_id")]
    pub faction_id: i32,

/// ID of the author-player.
    #[serde(rename = "author_id")]
    pub author_id: i32,

/// Type of marker (e.g., text, zone).
    #[serde(rename = "marker_type")]
    pub marker_type: i32,

/// Marker text (up to 20 characters).
    #[serde(rename = "marker_text")]
    #[validate(
            length(max = 20),
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker_text: Option<String>,

/// X-coordinate of the marker.
    #[serde(rename = "marker_x")]
    pub marker_x: i32,

/// Y-coordinate of the marker.
    #[serde(rename = "marker_y")]
    pub marker_y: i32,

/// Z-coordinate of the marker.
    #[serde(rename = "marker_z")]
    pub marker_z: i32,

}


impl FactionMarker {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(faction_id: i32, author_id: i32, marker_type: i32, marker_x: i32, marker_y: i32, marker_z: i32, ) -> FactionMarker {
        FactionMarker {
            marker_id: None,
            faction_id,
            author_id,
            marker_type,
            marker_text: None,
            marker_x,
            marker_y,
            marker_z,
        }
    }
}

/// Converts the FactionMarker value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for FactionMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.marker_id.as_ref().map(|marker_id| {
                [
                    "marker_id".to_string(),
                    marker_id.to_string(),
                ].join(",")
            }),


            Some("faction_id".to_string()),
            Some(self.faction_id.to_string()),


            Some("author_id".to_string()),
            Some(self.author_id.to_string()),


            Some("marker_type".to_string()),
            Some(self.marker_type.to_string()),


            self.marker_text.as_ref().map(|marker_text| {
                [
                    "marker_text".to_string(),
                    marker_text.to_string(),
                ].join(",")
            }),


            Some("marker_x".to_string()),
            Some(self.marker_x.to_string()),


            Some("marker_y".to_string()),
            Some(self.marker_y.to_string()),


            Some("marker_z".to_string()),
            Some(self.marker_z.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FactionMarker value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FactionMarker {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub marker_id: Vec<i32>,
            pub faction_id: Vec<i32>,
            pub author_id: Vec<i32>,
            pub marker_type: Vec<i32>,
            pub marker_text: Vec<String>,
            pub marker_x: Vec<i32>,
            pub marker_y: Vec<i32>,
            pub marker_z: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing FactionMarker".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "marker_id" => intermediate_rep.marker_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "faction_id" => intermediate_rep.faction_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "author_id" => intermediate_rep.author_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "marker_type" => intermediate_rep.marker_type.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "marker_text" => intermediate_rep.marker_text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "marker_x" => intermediate_rep.marker_x.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "marker_y" => intermediate_rep.marker_y.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "marker_z" => intermediate_rep.marker_z.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing FactionMarker".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FactionMarker {
            marker_id: intermediate_rep.marker_id.into_iter().next(),
            faction_id: intermediate_rep.faction_id.into_iter().next().ok_or_else(|| "faction_id missing in FactionMarker".to_string())?,
            author_id: intermediate_rep.author_id.into_iter().next().ok_or_else(|| "author_id missing in FactionMarker".to_string())?,
            marker_type: intermediate_rep.marker_type.into_iter().next().ok_or_else(|| "marker_type missing in FactionMarker".to_string())?,
            marker_text: intermediate_rep.marker_text.into_iter().next(),
            marker_x: intermediate_rep.marker_x.into_iter().next().ok_or_else(|| "marker_x missing in FactionMarker".to_string())?,
            marker_y: intermediate_rep.marker_y.into_iter().next().ok_or_else(|| "marker_y missing in FactionMarker".to_string())?,
            marker_z: intermediate_rep.marker_z.into_iter().next().ok_or_else(|| "marker_z missing in FactionMarker".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FactionMarker> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<FactionMarker>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<FactionMarker>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for FactionMarker - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<FactionMarker> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FactionMarker as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into FactionMarker - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct FactionPlayer {
/// Unique identifier for the player in the faction.
    #[serde(rename = "player_faction_id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_faction_id: Option<i32>,

/// ID of the player.
    #[serde(rename = "player_id")]
    pub player_id: i32,

/// ID of the associated faction.
    #[serde(rename = "faction_id")]
    pub faction_id: i32,

/// Status of the player's ground base.
    #[serde(rename = "ground_base_status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ground_base_status: Option<String>,

/// X-coordinate of the player's ground base.
    #[serde(rename = "ground_base_x")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ground_base_x: Option<f32>,

/// Y-coordinate of the player's ground base.
    #[serde(rename = "ground_base_y")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ground_base_y: Option<f32>,

/// Z-coordinate of the player's ground base.
    #[serde(rename = "ground_base_z")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ground_base_z: Option<f32>,

/// Status of the player's air base.
    #[serde(rename = "air_base_status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub air_base_status: Option<String>,

/// Patrol type of the player's air base.
    #[serde(rename = "air_base_patrol_type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub air_base_patrol_type: Option<String>,

/// General status of the player.
    #[serde(rename = "status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<i32>,

/// Player's own resources.
    #[serde(rename = "self_resources")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub self_resources: Option<i32>,

}


impl FactionPlayer {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(player_id: i32, faction_id: i32, ) -> FactionPlayer {
        FactionPlayer {
            player_faction_id: None,
            player_id,
            faction_id,
            ground_base_status: None,
            ground_base_x: None,
            ground_base_y: None,
            ground_base_z: None,
            air_base_status: None,
            air_base_patrol_type: None,
            status: None,
            self_resources: None,
        }
    }
}

/// Converts the FactionPlayer value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for FactionPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.player_faction_id.as_ref().map(|player_faction_id| {
                [
                    "player_faction_id".to_string(),
                    player_faction_id.to_string(),
                ].join(",")
            }),


            Some("player_id".to_string()),
            Some(self.player_id.to_string()),


            Some("faction_id".to_string()),
            Some(self.faction_id.to_string()),


            self.ground_base_status.as_ref().map(|ground_base_status| {
                [
                    "ground_base_status".to_string(),
                    ground_base_status.to_string(),
                ].join(",")
            }),


            self.ground_base_x.as_ref().map(|ground_base_x| {
                [
                    "ground_base_x".to_string(),
                    ground_base_x.to_string(),
                ].join(",")
            }),


            self.ground_base_y.as_ref().map(|ground_base_y| {
                [
                    "ground_base_y".to_string(),
                    ground_base_y.to_string(),
                ].join(",")
            }),


            self.ground_base_z.as_ref().map(|ground_base_z| {
                [
                    "ground_base_z".to_string(),
                    ground_base_z.to_string(),
                ].join(",")
            }),


            self.air_base_status.as_ref().map(|air_base_status| {
                [
                    "air_base_status".to_string(),
                    air_base_status.to_string(),
                ].join(",")
            }),


            self.air_base_patrol_type.as_ref().map(|air_base_patrol_type| {
                [
                    "air_base_patrol_type".to_string(),
                    air_base_patrol_type.to_string(),
                ].join(",")
            }),


            self.status.as_ref().map(|status| {
                [
                    "status".to_string(),
                    status.to_string(),
                ].join(",")
            }),


            self.self_resources.as_ref().map(|self_resources| {
                [
                    "self_resources".to_string(),
                    self_resources.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a FactionPlayer value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for FactionPlayer {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub player_faction_id: Vec<i32>,
            pub player_id: Vec<i32>,
            pub faction_id: Vec<i32>,
            pub ground_base_status: Vec<String>,
            pub ground_base_x: Vec<f32>,
            pub ground_base_y: Vec<f32>,
            pub ground_base_z: Vec<f32>,
            pub air_base_status: Vec<String>,
            pub air_base_patrol_type: Vec<String>,
            pub status: Vec<i32>,
            pub self_resources: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing FactionPlayer".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "player_faction_id" => intermediate_rep.player_faction_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "player_id" => intermediate_rep.player_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "faction_id" => intermediate_rep.faction_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ground_base_status" => intermediate_rep.ground_base_status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ground_base_x" => intermediate_rep.ground_base_x.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ground_base_y" => intermediate_rep.ground_base_y.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "ground_base_z" => intermediate_rep.ground_base_z.push(<f32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "air_base_status" => intermediate_rep.air_base_status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "air_base_patrol_type" => intermediate_rep.air_base_patrol_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "self_resources" => intermediate_rep.self_resources.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing FactionPlayer".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(FactionPlayer {
            player_faction_id: intermediate_rep.player_faction_id.into_iter().next(),
            player_id: intermediate_rep.player_id.into_iter().next().ok_or_else(|| "player_id missing in FactionPlayer".to_string())?,
            faction_id: intermediate_rep.faction_id.into_iter().next().ok_or_else(|| "faction_id missing in FactionPlayer".to_string())?,
            ground_base_status: intermediate_rep.ground_base_status.into_iter().next(),
            ground_base_x: intermediate_rep.ground_base_x.into_iter().next(),
            ground_base_y: intermediate_rep.ground_base_y.into_iter().next(),
            ground_base_z: intermediate_rep.ground_base_z.into_iter().next(),
            air_base_status: intermediate_rep.air_base_status.into_iter().next(),
            air_base_patrol_type: intermediate_rep.air_base_patrol_type.into_iter().next(),
            status: intermediate_rep.status.into_iter().next(),
            self_resources: intermediate_rep.self_resources.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<FactionPlayer> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<FactionPlayer>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<FactionPlayer>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for FactionPlayer - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<FactionPlayer> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <FactionPlayer as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into FactionPlayer - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Fraction {
/// Unique identifier for the fraction.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,

/// ID of the associated campaign. The fraction is deleted if the campaign is removed.
    #[serde(rename = "campaign_id")]
    pub campaign_id: i32,

/// Name of the fraction. Cannot be empty.
    #[serde(rename = "name")]
    #[validate(
            length(max = 255),
        )]
    pub name: String,

/// Predetermined type ID. Must be positive.
    #[serde(rename = "type_id")]
    pub type_id: i32,

/// JSON object containing custom data about the fraction.
    #[serde(rename = "custom_data")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_data: Option<std::collections::HashMap<String, crate::types::Object>>,

/// Color code of the fraction. Must be positive.
    #[serde(rename = "color_code")]
    pub color_code: i32,

/// Short description of the fraction (up to 40 characters).
    #[serde(rename = "short_description")]
    #[validate(
            length(max = 40),
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub short_description: Option<String>,

/// Full description of the fraction (up to 200 characters).
    #[serde(rename = "description")]
    #[validate(
            length(max = 200),
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

}


impl Fraction {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(campaign_id: i32, name: String, type_id: i32, color_code: i32, ) -> Fraction {
        Fraction {
            id: None,
            campaign_id,
            name,
            type_id,
            custom_data: None,
            color_code,
            short_description: None,
            description: None,
        }
    }
}

/// Converts the Fraction value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            Some("campaign_id".to_string()),
            Some(self.campaign_id.to_string()),


            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("type_id".to_string()),
            Some(self.type_id.to_string()),

            // Skipping custom_data in query parameter serialization
            // Skipping custom_data in query parameter serialization


            Some("color_code".to_string()),
            Some(self.color_code.to_string()),


            self.short_description.as_ref().map(|short_description| {
                [
                    "short_description".to_string(),
                    short_description.to_string(),
                ].join(",")
            }),


            self.description.as_ref().map(|description| {
                [
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Fraction value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Fraction {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub campaign_id: Vec<i32>,
            pub name: Vec<String>,
            pub type_id: Vec<i32>,
            pub custom_data: Vec<std::collections::HashMap<String, crate::types::Object>>,
            pub color_code: Vec<i32>,
            pub short_description: Vec<String>,
            pub description: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Fraction".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "campaign_id" => intermediate_rep.campaign_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type_id" => intermediate_rep.type_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "custom_data" => return std::result::Result::Err("Parsing a container in this style is not supported in Fraction".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "color_code" => intermediate_rep.color_code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "short_description" => intermediate_rep.short_description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Fraction".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Fraction {
            id: intermediate_rep.id.into_iter().next(),
            campaign_id: intermediate_rep.campaign_id.into_iter().next().ok_or_else(|| "campaign_id missing in Fraction".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in Fraction".to_string())?,
            type_id: intermediate_rep.type_id.into_iter().next().ok_or_else(|| "type_id missing in Fraction".to_string())?,
            custom_data: intermediate_rep.custom_data.into_iter().next(),
            color_code: intermediate_rep.color_code.into_iter().next().ok_or_else(|| "color_code missing in Fraction".to_string())?,
            short_description: intermediate_rep.short_description.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Fraction> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Fraction>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Fraction>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Fraction - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Fraction> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Fraction as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Fraction - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MainCampaignPlayerMapping {
/// ID of the campaign associated with the player. Deleted if the campaign is removed.
    #[serde(rename = "campaign_id")]
    pub campaign_id: i32,

/// ID of the player associated with the campaign. Deleted if the player is removed.
    #[serde(rename = "player_id")]
    pub player_id: i32,

/// Role ID of the player within the campaign.
    #[serde(rename = "role_id")]
    pub role_id: i32,

}


impl MainCampaignPlayerMapping {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(campaign_id: i32, player_id: i32, role_id: i32, ) -> MainCampaignPlayerMapping {
        MainCampaignPlayerMapping {
            campaign_id,
            player_id,
            role_id,
        }
    }
}

/// Converts the MainCampaignPlayerMapping value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MainCampaignPlayerMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("campaign_id".to_string()),
            Some(self.campaign_id.to_string()),


            Some("player_id".to_string()),
            Some(self.player_id.to_string()),


            Some("role_id".to_string()),
            Some(self.role_id.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MainCampaignPlayerMapping value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MainCampaignPlayerMapping {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub campaign_id: Vec<i32>,
            pub player_id: Vec<i32>,
            pub role_id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MainCampaignPlayerMapping".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "campaign_id" => intermediate_rep.campaign_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "player_id" => intermediate_rep.player_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "role_id" => intermediate_rep.role_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MainCampaignPlayerMapping".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MainCampaignPlayerMapping {
            campaign_id: intermediate_rep.campaign_id.into_iter().next().ok_or_else(|| "campaign_id missing in MainCampaignPlayerMapping".to_string())?,
            player_id: intermediate_rep.player_id.into_iter().next().ok_or_else(|| "player_id missing in MainCampaignPlayerMapping".to_string())?,
            role_id: intermediate_rep.role_id.into_iter().next().ok_or_else(|| "role_id missing in MainCampaignPlayerMapping".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MainCampaignPlayerMapping> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MainCampaignPlayerMapping>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MainCampaignPlayerMapping>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MainCampaignPlayerMapping - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MainCampaignPlayerMapping> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MainCampaignPlayerMapping as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MainCampaignPlayerMapping - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MatchEvent {
/// Unique identifier for the match event.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,

/// Array of player IDs for team 1. Must contain exactly 6 elements.
    #[serde(rename = "team1_player_ids")]
    #[validate(
            length(min = 6, max = 6),
        )]
    pub team1_player_ids: Vec<i32>,

/// Array of player IDs for team 2. Must contain exactly 6 elements.
    #[serde(rename = "team2_player_ids")]
    #[validate(
            length(min = 6, max = 6),
        )]
    pub team2_player_ids: Vec<i32>,

/// ID of the faction associated with team 1. Nullified on faction deletion.
    #[serde(rename = "faction1_id")]
    pub faction1_id: i32,

/// ID of the faction associated with team 2. Nullified on faction deletion.
    #[serde(rename = "faction2_id")]
    pub faction2_id: i32,

/// Match result. 0 = draw, 1 = team 1 wins, 2 = team 2 wins.
/// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "match_result")]
    pub match_result: i32,

/// Score for team 1. Must be non-negative.
    #[serde(rename = "team1_score")]
    #[validate(
            range(min = 0),
        )]
    pub team1_score: u32,

/// Score for team 2. Must be non-negative.
    #[serde(rename = "team2_score")]
    #[validate(
            range(min = 0),
        )]
    pub team2_score: u32,

/// Losses for team 1. Must be non-negative.
    #[serde(rename = "team1_losses")]
    #[validate(
            range(min = 0),
        )]
    pub team1_losses: u32,

/// Losses for team 2. Must be non-negative.
    #[serde(rename = "team2_losses")]
    #[validate(
            range(min = 0),
        )]
    pub team2_losses: u32,

/// Array of hero IDs participating in the match. Must contain exactly 12 elements.
    #[serde(rename = "hero_ids")]
    #[validate(
            length(min = 12, max = 12),
        )]
    pub hero_ids: Vec<i32>,

}


impl MatchEvent {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(team1_player_ids: Vec<i32>, team2_player_ids: Vec<i32>, faction1_id: i32, faction2_id: i32, match_result: i32, team1_score: u32, team2_score: u32, team1_losses: u32, team2_losses: u32, hero_ids: Vec<i32>, ) -> MatchEvent {
        MatchEvent {
            id: None,
            team1_player_ids,
            team2_player_ids,
            faction1_id,
            faction2_id,
            match_result,
            team1_score,
            team2_score,
            team1_losses,
            team2_losses,
            hero_ids,
        }
    }
}

/// Converts the MatchEvent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MatchEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            Some("team1_player_ids".to_string()),
            Some(self.team1_player_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),


            Some("team2_player_ids".to_string()),
            Some(self.team2_player_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),


            Some("faction1_id".to_string()),
            Some(self.faction1_id.to_string()),


            Some("faction2_id".to_string()),
            Some(self.faction2_id.to_string()),


            Some("match_result".to_string()),
            Some(self.match_result.to_string()),


            Some("team1_score".to_string()),
            Some(self.team1_score.to_string()),


            Some("team2_score".to_string()),
            Some(self.team2_score.to_string()),


            Some("team1_losses".to_string()),
            Some(self.team1_losses.to_string()),


            Some("team2_losses".to_string()),
            Some(self.team2_losses.to_string()),


            Some("hero_ids".to_string()),
            Some(self.hero_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MatchEvent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MatchEvent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub team1_player_ids: Vec<Vec<i32>>,
            pub team2_player_ids: Vec<Vec<i32>>,
            pub faction1_id: Vec<i32>,
            pub faction2_id: Vec<i32>,
            pub match_result: Vec<i32>,
            pub team1_score: Vec<u32>,
            pub team2_score: Vec<u32>,
            pub team1_losses: Vec<u32>,
            pub team2_losses: Vec<u32>,
            pub hero_ids: Vec<Vec<i32>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MatchEvent".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "team1_player_ids" => return std::result::Result::Err("Parsing a container in this style is not supported in MatchEvent".to_string()),
                    "team2_player_ids" => return std::result::Result::Err("Parsing a container in this style is not supported in MatchEvent".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "faction1_id" => intermediate_rep.faction1_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "faction2_id" => intermediate_rep.faction2_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "match_result" => intermediate_rep.match_result.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "team1_score" => intermediate_rep.team1_score.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "team2_score" => intermediate_rep.team2_score.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "team1_losses" => intermediate_rep.team1_losses.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "team2_losses" => intermediate_rep.team2_losses.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "hero_ids" => return std::result::Result::Err("Parsing a container in this style is not supported in MatchEvent".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MatchEvent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MatchEvent {
            id: intermediate_rep.id.into_iter().next(),
            team1_player_ids: intermediate_rep.team1_player_ids.into_iter().next().ok_or_else(|| "team1_player_ids missing in MatchEvent".to_string())?,
            team2_player_ids: intermediate_rep.team2_player_ids.into_iter().next().ok_or_else(|| "team2_player_ids missing in MatchEvent".to_string())?,
            faction1_id: intermediate_rep.faction1_id.into_iter().next().ok_or_else(|| "faction1_id missing in MatchEvent".to_string())?,
            faction2_id: intermediate_rep.faction2_id.into_iter().next().ok_or_else(|| "faction2_id missing in MatchEvent".to_string())?,
            match_result: intermediate_rep.match_result.into_iter().next().ok_or_else(|| "match_result missing in MatchEvent".to_string())?,
            team1_score: intermediate_rep.team1_score.into_iter().next().ok_or_else(|| "team1_score missing in MatchEvent".to_string())?,
            team2_score: intermediate_rep.team2_score.into_iter().next().ok_or_else(|| "team2_score missing in MatchEvent".to_string())?,
            team1_losses: intermediate_rep.team1_losses.into_iter().next().ok_or_else(|| "team1_losses missing in MatchEvent".to_string())?,
            team2_losses: intermediate_rep.team2_losses.into_iter().next().ok_or_else(|| "team2_losses missing in MatchEvent".to_string())?,
            hero_ids: intermediate_rep.hero_ids.into_iter().next().ok_or_else(|| "hero_ids missing in MatchEvent".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MatchEvent> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MatchEvent>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MatchEvent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MatchEvent - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MatchEvent> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MatchEvent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MatchEvent - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MegaCampaign {
/// Unique identifier for the mega campaign.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,

/// Name of the mega campaign, required.
    #[serde(rename = "name")]
    #[validate(
            length(max = 255),
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

/// Detailed description of the mega campaign.
    #[serde(rename = "description")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description: Option<String>,

/// Description aimed at new players.
    #[serde(rename = "descriptionForNewPlayers")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub description_for_new_players: Option<String>,

/// Reference to the creator's player ID. Null if the creator is deleted.
    #[serde(rename = "creatorId")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creator_id: Option<Nullable<i32>>,

/// Binary data for the menu image.
    #[serde(rename = "menuImage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub menu_image: Option<ByteArray>,

/// Type of campaign
/// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "typeOpened")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_opened: Option<i32>,

/// Maximum number of players allowed, must be positive.
    #[serde(rename = "playerLimit")]
    #[validate(
            range(min = 1),
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub player_limit: Option<u32>,

}


impl MegaCampaign {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> MegaCampaign {
        MegaCampaign {
            id: None,
            name: None,
            description: None,
            description_for_new_players: None,
            creator_id: None,
            menu_image: None,
            type_opened: None,
            player_limit: None,
        }
    }
}

/// Converts the MegaCampaign value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MegaCampaign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),


            self.description.as_ref().map(|description| {
                [
                    "description".to_string(),
                    description.to_string(),
                ].join(",")
            }),


            self.description_for_new_players.as_ref().map(|description_for_new_players| {
                [
                    "descriptionForNewPlayers".to_string(),
                    description_for_new_players.to_string(),
                ].join(",")
            }),


            self.creator_id.as_ref().map(|creator_id| {
                [
                    "creatorId".to_string(),
                    creator_id.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),

            // Skipping menuImage in query parameter serialization
            // Skipping menuImage in query parameter serialization


            self.type_opened.as_ref().map(|type_opened| {
                [
                    "typeOpened".to_string(),
                    type_opened.to_string(),
                ].join(",")
            }),


            self.player_limit.as_ref().map(|player_limit| {
                [
                    "playerLimit".to_string(),
                    player_limit.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MegaCampaign value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MegaCampaign {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub name: Vec<String>,
            pub description: Vec<String>,
            pub description_for_new_players: Vec<String>,
            pub creator_id: Vec<i32>,
            pub menu_image: Vec<ByteArray>,
            pub type_opened: Vec<i32>,
            pub player_limit: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MegaCampaign".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "description" => intermediate_rep.description.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "descriptionForNewPlayers" => intermediate_rep.description_for_new_players.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "creatorId" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in MegaCampaign".to_string()),
                    "menuImage" => return std::result::Result::Err("Parsing binary data in this style is not supported in MegaCampaign".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "typeOpened" => intermediate_rep.type_opened.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "playerLimit" => intermediate_rep.player_limit.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MegaCampaign".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MegaCampaign {
            id: intermediate_rep.id.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            description: intermediate_rep.description.into_iter().next(),
            description_for_new_players: intermediate_rep.description_for_new_players.into_iter().next(),
            creator_id: std::result::Result::Err("Nullable types not supported in MegaCampaign".to_string())?,
            menu_image: intermediate_rep.menu_image.into_iter().next(),
            type_opened: intermediate_rep.type_opened.into_iter().next(),
            player_limit: intermediate_rep.player_limit.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MegaCampaign> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MegaCampaign>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MegaCampaign>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MegaCampaign - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MegaCampaign> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MegaCampaign as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MegaCampaign - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NewsEvent {
/// Unique identifier for the news event.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,

/// ID of the associated campaign. If the campaign is deleted, the event is also removed.
    #[serde(rename = "campaign_id")]
    #[serde(deserialize_with = "deserialize_optional_nullable")]
    #[serde(default = "default_optional_nullable")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub campaign_id: Option<Nullable<i32>>,

/// ID of the author, must be positive.
    #[serde(rename = "author_id")]
    #[validate(
            range(min = 1),
        )]
    pub author_id: u32,

/// Name of the author, must be non-empty.
    #[serde(rename = "author_name")]
    #[validate(
            length(max = 255),
        )]
    pub author_name: String,

/// Text of the news event, must be non-empty.
    #[serde(rename = "text")]
    #[validate(
            length(min = 1),
        )]
    pub text: String,

/// Type of the event, must be non-empty and up to 50 characters.
    #[serde(rename = "event_type")]
    #[validate(
            length(max = 50),
        )]
    pub event_type: String,

/// Logged event type ID, must be positive.
    #[serde(rename = "log_event_type_todo")]
    #[validate(
            range(min = 1),
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_event_type_todo: Option<u32>,

}


impl NewsEvent {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(author_id: u32, author_name: String, text: String, event_type: String, ) -> NewsEvent {
        NewsEvent {
            id: None,
            campaign_id: None,
            author_id,
            author_name,
            text,
            event_type,
            log_event_type_todo: None,
        }
    }
}

/// Converts the NewsEvent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NewsEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.campaign_id.as_ref().map(|campaign_id| {
                [
                    "campaign_id".to_string(),
                    campaign_id.as_ref().map_or("null".to_string(), |x| x.to_string()),
                ].join(",")
            }),


            Some("author_id".to_string()),
            Some(self.author_id.to_string()),


            Some("author_name".to_string()),
            Some(self.author_name.to_string()),


            Some("text".to_string()),
            Some(self.text.to_string()),


            Some("event_type".to_string()),
            Some(self.event_type.to_string()),


            self.log_event_type_todo.as_ref().map(|log_event_type_todo| {
                [
                    "log_event_type_todo".to_string(),
                    log_event_type_todo.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NewsEvent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NewsEvent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub campaign_id: Vec<i32>,
            pub author_id: Vec<u32>,
            pub author_name: Vec<String>,
            pub text: Vec<String>,
            pub event_type: Vec<String>,
            pub log_event_type_todo: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NewsEvent".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "campaign_id" => return std::result::Result::Err("Parsing a nullable type in this style is not supported in NewsEvent".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "author_id" => intermediate_rep.author_id.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "author_name" => intermediate_rep.author_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "event_type" => intermediate_rep.event_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "log_event_type_todo" => intermediate_rep.log_event_type_todo.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NewsEvent".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NewsEvent {
            id: intermediate_rep.id.into_iter().next(),
            campaign_id: std::result::Result::Err("Nullable types not supported in NewsEvent".to_string())?,
            author_id: intermediate_rep.author_id.into_iter().next().ok_or_else(|| "author_id missing in NewsEvent".to_string())?,
            author_name: intermediate_rep.author_name.into_iter().next().ok_or_else(|| "author_name missing in NewsEvent".to_string())?,
            text: intermediate_rep.text.into_iter().next().ok_or_else(|| "text missing in NewsEvent".to_string())?,
            event_type: intermediate_rep.event_type.into_iter().next().ok_or_else(|| "event_type missing in NewsEvent".to_string())?,
            log_event_type_todo: intermediate_rep.log_event_type_todo.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NewsEvent> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NewsEvent>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NewsEvent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for NewsEvent - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NewsEvent> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NewsEvent as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into NewsEvent - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}




#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Player {
/// Unique identifier for the player.
    #[serde(rename = "id")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub id: Option<i32>,

/// Player's name, required.
    #[serde(rename = "name")]
    #[validate(
            length(max = 255),
        )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

/// JPEG avatar of the player stored as binary data.
    #[serde(rename = "avatar")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub avatar: Option<ByteArray>,

/// Player's statistics stored in JSONB format.
    #[serde(rename = "statistics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub statistics: Option<std::collections::HashMap<String, crate::types::Object>>,

/// Composition of the player's army stored in JSONB format.
    #[serde(rename = "army")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub army: Option<std::collections::HashMap<String, crate::types::Object>>,

}


impl Player {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> Player {
        Player {
            id: None,
            name: None,
            avatar: None,
            statistics: None,
            army: None,
        }
    }
}

/// Converts the Player value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            self.id.as_ref().map(|id| {
                [
                    "id".to_string(),
                    id.to_string(),
                ].join(",")
            }),


            self.name.as_ref().map(|name| {
                [
                    "name".to_string(),
                    name.to_string(),
                ].join(",")
            }),

            // Skipping avatar in query parameter serialization
            // Skipping avatar in query parameter serialization

            // Skipping statistics in query parameter serialization
            // Skipping statistics in query parameter serialization

            // Skipping army in query parameter serialization
            // Skipping army in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Player value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Player {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub name: Vec<String>,
            pub avatar: Vec<ByteArray>,
            pub statistics: Vec<std::collections::HashMap<String, crate::types::Object>>,
            pub army: Vec<std::collections::HashMap<String, crate::types::Object>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Player".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "avatar" => return std::result::Result::Err("Parsing binary data in this style is not supported in Player".to_string()),
                    "statistics" => return std::result::Result::Err("Parsing a container in this style is not supported in Player".to_string()),
                    "army" => return std::result::Result::Err("Parsing a container in this style is not supported in Player".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Player".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Player {
            id: intermediate_rep.id.into_iter().next(),
            name: intermediate_rep.name.into_iter().next(),
            avatar: intermediate_rep.avatar.into_iter().next(),
            statistics: intermediate_rep.statistics.into_iter().next(),
            army: intermediate_rep.army.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Player> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Player>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Player>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Player - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Player> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Player as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Player - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}



