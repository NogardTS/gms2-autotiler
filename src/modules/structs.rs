use serde::{Deserialize, Serialize};
use serde_jsonrc::value::Value;
use serde_with::skip_serializing_none;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LastFiles {
    pub room_path: String,
    pub block_path: String,
    pub tileset_path: String,
}

impl Default for LastFiles {
    fn default() -> Self {
        LastFiles {
            room_path: "C:\\".to_string(),
            block_path: "C:\\".to_string(),
            tileset_path: "C:\\".to_string(),
        }
    }
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub layers: VecDeque<Layer>,
    pub room_settings: RoomSettings,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub tileset_id: Option<TilesetId>,
    pub x: Option<i64>,
    pub y: Option<i64>,
    pub tiles: Option<Tiles>,
    pub visible: Option<bool>,
    pub depth: Option<i64>,
    pub userdefined_depth: Option<bool>,
    pub inherit_layer_depth: Option<bool>,
    pub inherit_layer_settings: Option<bool>,
    pub instances: Option<Vec<Instance>>,
    pub grid_x: Option<i64>,
    pub grid_y: Option<i64>,
    pub layers: Option<Vec<::serde_jsonrc::Value>>,
    pub hierarchy_frozen: Option<bool>,
    pub resource_version: Option<String>,
    pub name: Option<String>,
    pub tags: Option<Vec<::serde_jsonrc::Value>>,
    pub resource_type: Option<String>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TilesetId {
    pub name: String,
    pub path: String,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tiles {
    #[serde(rename = "SerialiseWidth")]
    pub serialise_width: i64,
    #[serde(rename = "SerialiseHeight")]
    pub serialise_height: i64,
    #[serde(rename = "TileSerialiseData")]
    pub tile_serialise_data: Vec<i64>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomSettings {
    pub inherit_room_settings: bool,
    #[serde(rename = "Width")]
    pub width: i64,
    #[serde(rename = "Height")]
    pub height: i64,
    pub persistent: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub object_id: ObjectId,
    pub scale_x: f64,
    pub scale_y: f64,
    pub x: f64,
    pub y: f64,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
pub struct ObjectId {
    pub name: String,
    pub path: String,
}
