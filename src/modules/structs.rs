use serde::{Serialize, Deserialize};
use serde_json::value::Value;
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
            tileset_path: "C:\\".to_string()
        }
    }
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub name: String,
    pub id: String,
    pub creation_code_file: String,
    pub inherit_code: bool,
    pub inherit_creation_order: bool,
    pub inherit_layers: bool,
    #[serde(rename = "instanceCreationOrderIDs")]
    pub instance_creation_order_ids: Vec<String>,
    #[serde(rename = "IsDnD")]
    pub is_dn_d: bool,
    pub layers: VecDeque<Layer>,
    pub model_name: String,
    pub parent_id: String,
    pub physics_settings: PhysicsSettings,
    pub room_settings: RoomSettings,
    pub mvc: String,
    pub views: Vec<View>,
    pub view_settings: ViewSettings,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    #[serde(rename = "__type")]
    pub type_field: String,
    pub name: String,
    pub id: String,
    pub depth: i64,
    #[serde(rename = "grid_x")]
    pub grid_x: i64,
    #[serde(rename = "grid_y")]
    pub grid_y: i64,
    pub hierarchy_frozen: bool,
    pub hierarchy_visible: bool,
    pub inherit_layer_depth: bool,
    pub inherit_layer_settings: bool,
    pub inherit_sub_layers: bool,
    pub inherit_visibility: bool,
    pub layers: VecDeque<Layer>,
    #[serde(rename = "m_parentID")]
    pub m_parent_id: String,
    #[serde(rename = "m_serialiseFrozen")]
    pub m_serialise_frozen: bool,
    pub model_name: String,
    #[serde(rename = "prev_tileheight")]
    pub prev_tileheight: Option<i64>,
    #[serde(rename = "prev_tilewidth")]
    pub prev_tilewidth: Option<i64>,
    pub mvc: String,
    pub tiles: Option<Tiles>,
    pub tileset_id: Option<String>,
    #[serde(rename = "userdefined_depth")]
    pub userdefined_depth: bool,
    pub visible: bool,
    pub x: Option<i64>,
    pub y: Option<i64>,
    #[serde(default)]
    pub instances: Vec<Instance>,
    #[serde(rename = "animationFPS")]
    pub animation_fps: Option<i64>,
    pub animation_speed_type: Option<String>,
    pub colour: Option<Colour2>,
    pub hspeed: Option<i64>,
    pub htiled: Option<bool>,
    pub sprite_id: Option<String>,
    pub stretch: Option<bool>,
    #[serde(rename = "userdefined_animFPS")]
    pub userdefined_anim_fps: Option<bool>,
    pub vspeed: Option<i64>,
    pub vtiled: Option<bool>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tiles {
    #[serde(rename = "SerialiseData")]
    pub serialise_data: ::serde_json::Value,
    #[serde(rename = "SerialiseHeight")]
    pub serialise_height: i64,
    #[serde(rename = "SerialiseWidth")]
    pub serialise_width: i64,
    #[serde(rename = "TileSerialiseData")]
    pub tile_serialise_data: Vec<i64>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub name: String,
    pub id: String,
    pub colour: Colour,
    pub creation_code_file: String,
    pub creation_code_type: String,
    pub ignore: bool,
    pub image_index: f64,
    pub image_speed: f64,
    pub inherit_code: bool,
    pub inherit_item_settings: bool,
    #[serde(rename = "IsDnD")]
    pub is_dn_d: bool,
    #[serde(rename = "m_originalParentID")]
    pub m_original_parent_id: String,
    #[serde(rename = "m_serialiseFrozen")]
    pub m_serialise_frozen: bool,
    pub model_name: String,
    #[serde(rename = "name_with_no_file_rename")]
    pub name_with_no_file_rename: String,
    pub obj_id: String,
    pub properties: ::serde_json::Value,
    pub rotation: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub mvc: String,
    pub x: f64,
    pub y: f64,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colour {
    #[serde(rename = "Value")]
    pub value: i64,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colour2 {
    #[serde(rename = "Value")]
    pub value: i64,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicsSettings {
    pub id: String,
    pub inherit_physics_settings: bool,
    pub model_name: String,
    #[serde(rename = "PhysicsWorld")]
    pub physics_world: bool,
    #[serde(rename = "PhysicsWorldGravityX")]
    pub physics_world_gravity_x: i64,
    #[serde(rename = "PhysicsWorldGravityY")]
    pub physics_world_gravity_y: i64,
    #[serde(rename = "PhysicsWorldPixToMeters")]
    pub physics_world_pix_to_meters: f64,
    pub mvc: String,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomSettings {
    pub id: String,
    #[serde(rename = "Height")]
    pub height: i64,
    pub inherit_room_settings: bool,
    pub model_name: String,
    pub persistent: bool,
    pub mvc: String,
    #[serde(rename = "Width")]
    pub width: i64,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    pub id: String,
    pub hborder: i64,
    pub hport: i64,
    pub hspeed: i64,
    pub hview: i64,
    pub inherit: bool,
    pub model_name: String,
    pub obj_id: String,
    pub mvc: String,
    pub vborder: i64,
    pub visible: bool,
    pub vspeed: i64,
    pub wport: i64,
    pub wview: i64,
    pub xport: i64,
    pub xview: i64,
    pub yport: i64,
    pub yview: i64,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewSettings {
    pub id: String,
    pub clear_display_buffer: bool,
    pub clear_view_background: bool,
    pub enable_views: bool,
    pub inherit_view_settings: bool,
    pub model_name: String,
    pub mvc: String,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tileset {
    pub id: String,
    pub model_name: String,
    pub mvc: String,
    pub name: String,
    #[serde(rename = "auto_tile_sets")]
    pub auto_tile_sets: Vec<::serde_json::Value>,
    pub macro_page_tiles: MacroPageTiles,
    #[serde(rename = "out_columns")]
    pub out_columns: i64,
    #[serde(rename = "out_tilehborder")]
    pub out_tilehborder: i64,
    #[serde(rename = "out_tilevborder")]
    pub out_tilevborder: i64,
    pub sprite_id: String,
    #[serde(rename = "sprite_no_export")]
    pub sprite_no_export: bool,
    pub texture_group_id: String,
    #[serde(rename = "tile_animation")]
    pub tile_animation: TileAnimation,
    #[serde(rename = "tile_animation_frames")]
    pub tile_animation_frames: Vec<::serde_json::Value>,
    #[serde(rename = "tile_animation_speed")]
    pub tile_animation_speed: i64,
    #[serde(rename = "tile_count")]
    pub tile_count: i64,
    pub tileheight: i64,
    pub tilehsep: i64,
    pub tilevsep: i64,
    pub tilewidth: i64,
    pub tilexoff: i64,
    pub tileyoff: i64,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroPageTiles {
    #[serde(rename = "SerialiseData")]
    pub serialise_data: ::serde_json::Value,
    #[serde(rename = "SerialiseHeight")]
    pub serialise_height: i64,
    #[serde(rename = "SerialiseWidth")]
    pub serialise_width: i64,
    #[serde(rename = "TileSerialiseData")]
    pub tile_serialise_data: Vec<i64>,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TileAnimation {
    #[serde(rename = "AnimationCreationOrder")]
    pub animation_creation_order: ::serde_json::Value,
    #[serde(rename = "FrameData")]
    pub frame_data: Vec<i64>,
    #[serde(rename = "SerialiseFrameCount")]
    pub serialise_frame_count: i64,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    pub id: String,
    pub model_name: String,
    pub mvc: String,
    pub name: String,
    pub event_list: Vec<EventList>,
    pub mask_sprite_id: String,
    pub overridden_properties: ::serde_json::Value,
    pub parent_object_id: String,
    pub persistent: bool,
    pub physics_angular_damping: f64,
    pub physics_density: f64,
    pub physics_friction: f64,
    pub physics_group: i64,
    pub physics_kinematic: bool,
    pub physics_linear_damping: f64,
    pub physics_object: bool,
    pub physics_restitution: f64,
    pub physics_sensor: bool,
    pub physics_shape: i64,
    pub physics_shape_points: Option<Vec<::serde_json::Value>>,
    pub physics_start_awake: bool,
    pub properties: ::serde_json::Value,
    pub solid: bool,
    pub sprite_id: String,
    pub visible: bool,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventList {
    pub id: String,
    pub model_name: String,
    pub mvc: String,
    #[serde(rename = "IsDnD")]
    pub is_dn_d: bool,
    pub collision_object_id: String,
    pub enumb: i64,
    pub eventtype: i64,
    #[serde(rename = "m_owner")]
    pub m_owner: String,
    #[serde(flatten)]
    pub extra: Option<HashMap<String, Value>>,
}