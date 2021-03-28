use crate::modules::structs::*;

pub const LAST_FILES: &str = "last_files.json";
pub const BACKUP_FOLDER: &str = "backups";
pub const EMPTY_TILE: i64 = 2147483648;

lazy_static! {
    pub static ref DEFAULT_LAYER: Layer = serde_jsonrc::from_str::<Layer>(r#"{"tilesetId":{"name":"tileset2","path":"tilesets/tileset2/tileset2.yy"},"x":0,"y":0,"tiles":{"SerialiseWidth":50,"SerialiseHeight":38,"TileSerialiseData":[]},"visible":true,"depth":0,"userdefinedDepth":false,"inheritLayerDepth":false,"inheritLayerSettings":false,"gridX":16,"gridY":16,"layers":[],"hierarchyFrozen":false,"resourceVersion":"1.0","name":"PLACEHOLDER_NAME","tags":[],"resourceType":"GMRTileLayer"}"#).unwrap();
}
