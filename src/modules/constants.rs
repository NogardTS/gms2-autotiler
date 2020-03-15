use crate::modules::structs::*;

pub const LAST_FILES: &str = "last_files.json";
pub const BACKUP_FOLDER: &str = "backups";
pub const EMPTY_TILE: i64 = 2147483648;

lazy_static!{
    pub static ref DEFAULT_LAYER: Layer = serde_json::from_str::<Layer>(r#"{"__type":"GMRTileLayer_Model:#YoYoStudio.MVCFormat","name":"PLACEHOLDER_NAME","id":"373ee757-9b40-4e32-9c9f-b117f894f029","depth":0,"grid_x":16,"grid_y":16,"hierarchyFrozen":false,"hierarchyVisible":true,"inheritLayerDepth":false,"inheritLayerSettings":false,"inheritSubLayers":false,"inheritVisibility":false,"layers":[],"m_parentID":"00000000-0000-0000-0000-000000000000","m_serialiseFrozen":false,"modelName":"GMRTileLayer","prev_tileheight":16,"prev_tilewidth":16,"mvc":"1.0","tiles":{"SerialiseData":null,"SerialiseHeight":19,"SerialiseWidth":25,"TileSerialiseData":[]},"tilesetId":"99f4a5c3-9756-44aa-bcfb-dec594dbaf9a","userdefined_depth":false,"visible":true,"x":0,"y":0}"#).unwrap();
}