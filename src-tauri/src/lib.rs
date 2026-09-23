use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use log::{debug, error, info};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const SAVE_FILENAME: &str = "global-v35";
const RESOURCES_ASSETS: &str = "resources.assets";
const SHARED_ASSETS: &str = "sharedassets0.assets";
const GLOBAL_MANAGERS_ASSETS: &str = "globalgamemanagers.assets";
const EXPECTED_RESOURCES_SIZE: u64 = 59_975_696;
const EXPECTED_SHARED_SIZE: u64 = 59_876_160;

#[derive(Clone, Copy)]
struct FloatTarget {
    file: &'static str,
    offset: usize,
}

#[derive(Clone, Copy)]
struct ComponentFloatTarget {
    file: &'static str,
    game_object: &'static str,
    relative_offset: usize,
}

const GRIFFIN_NAMES: [&str; 5] = [
    "Griffin P1",
    "Griffin Greece",
    "Griffin P2",
    "Griffin Skull P2",
    "Griffin Skull P1",
];
const GRIFFIN_RUN: [ComponentFloatTarget; 5] =
    component_targets(RESOURCES_ASSETS, GRIFFIN_NAMES, 196);
const GRIFFIN_FOREST: [ComponentFloatTarget; 5] =
    component_targets(RESOURCES_ASSETS, GRIFFIN_NAMES, 200);
const GRIFFIN_RUN_STAMINA: [ComponentFloatTarget; 5] =
    component_targets(RESOURCES_ASSETS, GRIFFIN_NAMES, 208);
const GRIFFIN_SKILL_COST: [ComponentFloatTarget; 5] =
    component_targets(RESOURCES_ASSETS, GRIFFIN_NAMES, 32);
const HORSE_RUN: [ComponentFloatTarget; 4] = [
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P1 Greece",
        relative_offset: 188,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P2 Greece",
        relative_offset: 188,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P2",
        relative_offset: 196,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "Horse Regular",
        relative_offset: 196,
    },
];
const HORSE_RUN_STAMINA: [ComponentFloatTarget; 4] = [
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P1 Greece",
        relative_offset: 200,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P2 Greece",
        relative_offset: 200,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "Horse Regular P2",
        relative_offset: 208,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "Horse Regular",
        relative_offset: 208,
    },
];
const WARHORSE_NAMES: [&str; 2] = ["Warhorse P1", "Warhorse P2"];
const WARHORSE_RUN: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, WARHORSE_NAMES, 196);
const WARHORSE_RUN_STAMINA: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, WARHORSE_NAMES, 208);
const WARHORSE_SKILL_NAMES: [&str; 4] = [
    "Warhorse P1",
    "Warhorse P2",
    "Warhorse Plague P1",
    "Warhorse Plague P2",
];
const WARHORSE_SKILL_COST: [ComponentFloatTarget; 4] =
    component_targets(RESOURCES_ASSETS, WARHORSE_SKILL_NAMES, 32);
const WARHORSE_COOLDOWN: [FloatTarget; 2] = targets(RESOURCES_ASSETS, [58179764, 58288052]);
const WARHORSE_PLAGUE_COOLDOWN: [FloatTarget; 2] = targets(RESOURCES_ASSETS, [58169220, 58169108]);
const WARHORSE_RANGE: [FloatTarget; 4] =
    targets(RESOURCES_ASSETS, [58179816, 58288104, 58169272, 58169160]);
const WARHORSE_DURATION: [FloatTarget; 1] = targets(RESOURCES_ASSETS, [55809144]);
const ARCHER_NAMES: [&str; 3] = ["Archer_norselands", "Archer_Soldier_norselands", "Archer"];
const ARCHER_SHOOT_PREP: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 72);
const ARCHER_SHOOT_COOLDOWN: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 76);
const ARCHER_KNIGHT_COOLDOWN: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 88);
const ARCHER_INTERVAL_MIN: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 104);
const ARCHER_INTERVAL_MAX: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 108);
const ARCHER_FORMATION_INTERVAL_MIN: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 112);
const ARCHER_FORMATION_INTERVAL_MAX: [ComponentFloatTarget; 3] =
    component_targets(RESOURCES_ASSETS, ARCHER_NAMES, 116);
const BUILDER_NAMES: [&str; 2] = ["Worker_norselands", "Worker"];
const BUILDER_WALK_SPEED: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, BUILDER_NAMES, 40);
const BUILDER_RUN_SPEED: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, BUILDER_NAMES, 44);
const BUILDER_WORK_TIME: [ComponentFloatTarget; 2] =
    component_targets(RESOURCES_ASSETS, BUILDER_NAMES, 48);
const BAG_SCALE: [ComponentFloatTarget; 12] = [
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagGem",
        relative_offset: 40,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagGem",
        relative_offset: 44,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagGem",
        relative_offset: 48,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagCoin",
        relative_offset: 40,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagCoin",
        relative_offset: 44,
    },
    ComponentFloatTarget {
        file: SHARED_ASSETS,
        game_object: "BagCoin",
        relative_offset: 48,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_Norseland",
        relative_offset: 40,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_Norseland",
        relative_offset: 44,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_Norseland",
        relative_offset: 48,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_greece",
        relative_offset: 40,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_greece",
        relative_offset: 44,
    },
    ComponentFloatTarget {
        file: RESOURCES_ASSETS,
        game_object: "BagCoin_greece",
        relative_offset: 48,
    },
];

const fn targets<const N: usize>(file: &'static str, offsets: [usize; N]) -> [FloatTarget; N] {
    let mut result = [FloatTarget { file, offset: 0 }; N];
    let mut index = 0;
    while index < N {
        result[index] = FloatTarget {
            file,
            offset: offsets[index],
        };
        index += 1;
    }
    result
}

const fn component_targets<const N: usize>(
    file: &'static str,
    game_objects: [&'static str; N],
    relative_offset: usize,
) -> [ComponentFloatTarget; N] {
    let mut result = [ComponentFloatTarget {
        file,
        game_object: "",
        relative_offset,
    }; N];
    let mut index = 0;
    while index < N {
        result[index] = ComponentFloatTarget {
            file,
            game_object: game_objects[index],
            relative_offset,
        };
        index += 1;
    }
    result
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssetSettings {
    griffin_run_speed: f32,
    griffin_forest_multiplier: f32,
    griffin_run_stamina_rate: f32,
    griffin_skill_stamina_cost: f32,
    horse_run_speed: f32,
    horse_run_stamina_rate: f32,
    warhorse_run_speed: f32,
    warhorse_run_stamina_rate: f32,
    warhorse_skill_stamina_cost: f32,
    warhorse_cooldown: f32,
    warhorse_plague_cooldown: f32,
    warhorse_buff_duration: f32,
    warhorse_buff_range: f32,
    archer_shoot_prep_time: f32,
    archer_shoot_cooldown_time: f32,
    archer_shoot_cooldown_with_knight_time: f32,
    archer_interval_min: f32,
    archer_interval_max: f32,
    archer_formation_interval_min: f32,
    archer_formation_interval_max: f32,
    builder_walk_speed: f32,
    builder_run_speed: f32,
    builder_work_time: f32,
    bag_scale: f32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AssetResponse {
    data_directory: String,
    settings: AssetSettings,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AssetApplyResponse {
    resources_backup: String,
    shared_assets_backup: String,
    settings: AssetSettings,
}

#[derive(Serialize)]
struct LoadResponse {
    path: String,
    data: Value,
}

#[tauri::command]
fn load_save_file(path: String) -> Result<LoadResponse, String> {
    let path_buf = PathBuf::from(&path);
    validate_path(&path_buf)?;

    info!("Opening save file at {}", path_buf.display());

    let file = std::fs::File::open(&path_buf).map_err(|err| {
        error!("Unable to open {}: {}", path_buf.display(), err);
        err.to_string()
    })?;
    let mut decoder = GzDecoder::new(file);
    let mut json = String::new();
    decoder.read_to_string(&mut json).map_err(|err| {
        error!("Failed to decompress {}: {}", path_buf.display(), err);
        err.to_string()
    })?;
    debug!(
        "Decompression complete ({} JSON bytes)",
        json.as_bytes().len()
    );
    let data: Value = serde_json::from_str(&json).map_err(|err| {
        error!("JSON invalide dans {}: {}", path_buf.display(), err);
        err.to_string()
    })?;
    if let Some(obj) = data.as_object() {
        info!("File loaded with {} top-level keys", obj.len());
    }

    Ok(LoadResponse { path, data })
}

#[tauri::command]
fn select_save_file() -> Result<LoadResponse, String> {
    let Some(selected) = FileDialog::new().set_title("Choose global-v35").pick_file() else {
        return Err("Selection cancelled".into());
    };

    let path = selected.to_string_lossy().to_string();
    info!("Selected file: {}", path);
    load_save_file(path)
}

#[tauri::command]
fn save_save_file(path: String, data: Value) -> Result<String, String> {
    let path_buf = PathBuf::from(&path);
    validate_path(&path_buf)?;

    info!("Save requested for {}", path_buf.display());

    let backup_path = create_backup(&path_buf)?;

    let json = serde_json::to_string_pretty(&data).map_err(|err| {
        error!(
            "Unable to serialize data for {}: {}",
            path_buf.display(),
            err
        );
        err.to_string()
    })?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(json.as_bytes()).map_err(|err| {
        error!("Failed to recompress {}: {}", path_buf.display(), err);
        err.to_string()
    })?;
    let compressed = encoder.finish().map_err(|err| {
        error!(
            "Failed to finish compression for {}: {}",
            path_buf.display(),
            err
        );
        err.to_string()
    })?;

    log_resource_summaries(&data);

    info!(
        "Save: {} JSON bytes, {} gzip bytes",
        json.as_bytes().len(),
        compressed.len()
    );

    fs::write(&path_buf, compressed).map_err(|err| {
        error!("Unable to write {}: {}", path_buf.display(), err);
        err.to_string()
    })?;

    info!(
        "Save applied to {}, previous save: {}",
        path_buf.display(),
        backup_path.display()
    );

    Ok(backup_path.to_string_lossy().to_string())
}

fn log_resource_summaries(data: &Value) {
    const ARCHER_PREFABS: [&str; 2] = [
        "Prefabs/Characters/Archer",
        "Prefabs/Characters/norselands/Archer_norselands",
    ];
    const WORKER_PREFABS: [&str; 2] = [
        "Prefabs/Characters/Worker",
        "Prefabs/Characters/norselands/Worker_norselands",
    ];
    const FARMER_PREFABS: [&str; 2] = [
        "Prefabs/Characters/Farmer",
        "Prefabs/Characters/norselands/Farmer_norselands",
    ];
    const PIKEMAN_PREFABS: [&str; 3] = [
        "Prefabs/Characters/Pikeman",
        "Prefabs/Characters/norselands/Knight_norselands",
        "Prefabs/Characters/norselands/Pikeman_norselands",
    ];

    if let Some(campaigns) = data.get("campaigns").and_then(|value| value.as_array()) {
        for (campaign_index, campaign) in campaigns.iter().enumerate() {
            if let Some(carry) = campaign.get("carryForward").and_then(|v| v.as_object()) {
                let coins_p1 = carry
                    .get("currencyP1")
                    .and_then(|v| v.get("Coins"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(-1);
                let coins_p2 = carry
                    .get("currencyP2")
                    .and_then(|v| v.get("Coins"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(-1);
                let gems_p1 = carry
                    .get("currencyP1")
                    .and_then(|v| v.get("Gems"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(-1);
                let gems_p2 = carry
                    .get("currencyP2")
                    .and_then(|v| v.get("Gems"))
                    .and_then(|v| v.as_i64())
                    .unwrap_or(-1);

                info!(
                    "Campaign {}: coins P1 = {}, coins P2 = {}, gems P1 = {}, gems P2 = {}",
                    campaign_index, coins_p1, coins_p2, gems_p1, gems_p2
                );
            }

            if let Some(islands) = campaign.get("_islands").and_then(|v| v.as_array()) {
                for (island_index, island) in islands.iter().enumerate() {
                    if let Some(objects) = island.get("objects").and_then(|v| v.as_array()) {
                        let archers = count_prefab(objects, &ARCHER_PREFABS);
                        let workers = count_prefab(objects, &WORKER_PREFABS);
                        let farmers = count_prefab(objects, &FARMER_PREFABS);
                        let pikemen = count_prefab(objects, &PIKEMAN_PREFABS);

                        info!(
                            "Campaign {} island {}: archers={}, workers={}, farmers={}, pikemen={}",
                            campaign_index, island_index, archers, workers, farmers, pikemen
                        );
                    }
                }
            }
        }
    }
}

fn count_prefab(objects: &[Value], prefabs: &[&str]) -> usize {
    objects
        .iter()
        .filter_map(|entry| entry.as_object())
        .filter(|entry| {
            entry
                .get("prefabPath")
                .and_then(|v| v.as_str())
                .map(|path| prefabs.iter().any(|candidate| path.contains(candidate)))
                .unwrap_or(false)
        })
        .count()
}

fn validate_path(path: &Path) -> Result<(), String> {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return Err("Invalid file name".into());
    };

    debug!("Validating file: {}", name);
    if name != SAVE_FILENAME {
        return Err(format!("The selected file ({name}) is not {SAVE_FILENAME}"));
    }

    Ok(())
}

fn create_backup(original: &Path) -> Result<PathBuf, String> {
    let parent = original
        .parent()
        .ok_or_else(|| "Unable to determine the file directory".to_string())?;
    let stem = original
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(SAVE_FILENAME);

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| err.to_string())?
        .as_secs();

    let backup_name = format!("{}_{}.bak", stem, timestamp);
    let backup_path = parent.join(backup_name);

    fs::copy(original, &backup_path).map_err(|err| {
        error!(
            "Unable to create backup {} -> {}: {}",
            original.display(),
            backup_path.display(),
            err
        );
        err.to_string()
    })?;

    info!("Backup copy created: {}", backup_path.display());

    Ok(backup_path)
}

fn default_game_data_directory() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_default();
        return home.join("Library/Application Support/Steam/steamapps/common/Kingdom Two Crowns/KingdomTwoCrowns.app/Contents/Resources/Data");
    }
    #[cfg(target_os = "windows")]
    {
        let program_files = std::env::var_os("PROGRAMFILES(X86)")
            .or_else(|| std::env::var_os("PROGRAMFILES"))
            .map(PathBuf::from)
            .unwrap_or_default();
        return program_files
            .join("Steam/steamapps/common/Kingdom Two Crowns/KingdomTwoCrowns_Data");
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_default();
        home.join(".local/share/Steam/steamapps/common/Kingdom Two Crowns/KingdomTwoCrowns_Data")
    }
}

fn validate_asset_directory(directory: &Path) -> Result<(), String> {
    let resources = directory.join(RESOURCES_ASSETS);
    let shared = directory.join(SHARED_ASSETS);
    let resources_size = fs::metadata(&resources)
        .map_err(|_| {
            format!(
                "Could not find {} in {}",
                RESOURCES_ASSETS,
                directory.display()
            )
        })?
        .len();
    let shared_size = fs::metadata(&shared)
        .map_err(|_| {
            format!(
                "Could not find {} in {}",
                SHARED_ASSETS,
                directory.display()
            )
        })?
        .len();

    if resources_size != EXPECTED_RESOURCES_SIZE || shared_size != EXPECTED_SHARED_SIZE {
        return Err(format!(
            "These assets do not match the supported KTC profile (resources: {resources_size}, shared: {shared_size}). No changes were written."
        ));
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct UnityObject {
    path_id: i64,
    byte_start: usize,
    byte_size: usize,
    class_id: i32,
}

fn read_be_u32(bytes: &[u8], offset: usize) -> Result<u32, String> {
    let value = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| "Unity asset header is truncated".to_string())?;
    Ok(u32::from_be_bytes(value.try_into().unwrap()))
}

fn read_be_u64(bytes: &[u8], offset: usize) -> Result<u64, String> {
    let value = bytes
        .get(offset..offset + 8)
        .ok_or_else(|| "Unity asset header is truncated".to_string())?;
    Ok(u64::from_be_bytes(value.try_into().unwrap()))
}

fn read_le_i16(bytes: &[u8], offset: &mut usize) -> Result<i16, String> {
    let value = bytes
        .get(*offset..*offset + 2)
        .ok_or_else(|| "Unity asset metadata is truncated".to_string())?;
    *offset += 2;
    Ok(i16::from_le_bytes(value.try_into().unwrap()))
}

fn read_le_i32(bytes: &[u8], offset: &mut usize) -> Result<i32, String> {
    let value = bytes
        .get(*offset..*offset + 4)
        .ok_or_else(|| "Unity asset metadata is truncated".to_string())?;
    *offset += 4;
    Ok(i32::from_le_bytes(value.try_into().unwrap()))
}

fn read_le_i64(bytes: &[u8], offset: &mut usize) -> Result<i64, String> {
    let value = bytes
        .get(*offset..*offset + 8)
        .ok_or_else(|| "Unity asset metadata is truncated".to_string())?;
    *offset += 8;
    Ok(i64::from_le_bytes(value.try_into().unwrap()))
}

fn skip_c_string(bytes: &[u8], offset: &mut usize) -> Result<(), String> {
    let remainder = bytes
        .get(*offset..)
        .ok_or_else(|| "Unity asset metadata is truncated".to_string())?;
    let length = remainder
        .iter()
        .position(|byte| *byte == 0)
        .ok_or_else(|| "Unity asset string is not terminated".to_string())?;
    *offset += length + 1;
    Ok(())
}

fn align_four(offset: &mut usize) {
    *offset = (*offset + 3) & !3;
}

fn parse_unity_objects(bytes: &[u8]) -> Result<Vec<UnityObject>, String> {
    if read_be_u32(bytes, 8)? != 22 {
        return Err("Only Unity serialized-file version 22 is supported".into());
    }
    if bytes.get(16).copied() != Some(0) {
        return Err("Big-endian Unity assets are not supported".into());
    }
    let data_offset = usize::try_from(read_be_u64(bytes, 32)?)
        .map_err(|_| "Unity asset data offset is too large".to_string())?;
    let mut cursor = 48;
    skip_c_string(bytes, &mut cursor)?;
    cursor += 4; // target platform
    let type_tree_enabled = *bytes
        .get(cursor)
        .ok_or_else(|| "Unity asset metadata is truncated".to_string())?;
    cursor += 1;
    if type_tree_enabled != 0 {
        return Err("Embedded Unity type trees are not supported by this reader".into());
    }

    let type_count = read_le_i32(bytes, &mut cursor)?;
    if !(0..=10_000).contains(&type_count) {
        return Err("Unity asset contains an invalid type count".into());
    }
    let mut class_ids = Vec::with_capacity(type_count as usize);
    for _ in 0..type_count {
        let class_id = read_le_i32(bytes, &mut cursor)?;
        cursor += 1; // is stripped type
        let _script_type_index = read_le_i16(bytes, &mut cursor)?;
        if class_id == 114 {
            cursor += 16; // script id
        }
        cursor += 16; // old type hash
        if cursor > bytes.len() {
            return Err("Unity serialized type is truncated".into());
        }
        class_ids.push(class_id);
    }

    let object_count = read_le_i32(bytes, &mut cursor)?;
    if !(0..=2_000_000).contains(&object_count) {
        return Err("Unity asset contains an invalid object count".into());
    }
    let mut objects = Vec::with_capacity(object_count as usize);
    for _ in 0..object_count {
        align_four(&mut cursor);
        let path_id = read_le_i64(bytes, &mut cursor)?;
        let relative_start = read_le_i64(bytes, &mut cursor)?;
        let byte_size = read_le_i32(bytes, &mut cursor)?;
        let type_id = read_le_i32(bytes, &mut cursor)?;
        if relative_start < 0 || byte_size < 0 || type_id < 0 {
            return Err("Unity asset contains an invalid object entry".into());
        }
        let class_id = *class_ids
            .get(type_id as usize)
            .ok_or_else(|| "Unity object references an unknown type".to_string())?;
        let byte_start = data_offset
            .checked_add(relative_start as usize)
            .ok_or_else(|| "Unity object position overflowed".to_string())?;
        let byte_size = byte_size as usize;
        if byte_start
            .checked_add(byte_size)
            .filter(|end| *end <= bytes.len())
            .is_none()
        {
            return Err("Unity object points outside the asset file".into());
        }
        objects.push(UnityObject {
            path_id,
            byte_start,
            byte_size,
            class_id,
        });
    }
    Ok(objects)
}

fn object_bytes<'a>(bytes: &'a [u8], object: &UnityObject) -> Result<&'a [u8], String> {
    bytes
        .get(object.byte_start..object.byte_start + object.byte_size)
        .ok_or_else(|| "Unity object points outside the asset file".to_string())
}

fn read_aligned_string(bytes: &[u8], cursor: &mut usize) -> Result<String, String> {
    let length = read_le_i32(bytes, cursor)?;
    if length < 0 {
        return Err("Unity string has a negative length".into());
    }
    let value = bytes
        .get(*cursor..*cursor + length as usize)
        .ok_or_else(|| "Unity string is truncated".to_string())?;
    *cursor += length as usize;
    align_four(cursor);
    String::from_utf8(value.to_vec()).map_err(|_| "Unity string is not UTF-8".to_string())
}

fn mono_script_class_name(bytes: &[u8], object: &UnityObject) -> Result<String, String> {
    let raw = object_bytes(bytes, object)?;
    let mut cursor = 0;
    let _name = read_aligned_string(raw, &mut cursor)?;
    cursor += 4 + 16; // execution order and properties hash
    read_aligned_string(raw, &mut cursor)
}

fn game_object_data(bytes: &[u8], object: &UnityObject) -> Result<(String, Vec<i64>), String> {
    let raw = object_bytes(bytes, object)?;
    let mut cursor = 0;
    let count = read_le_i32(raw, &mut cursor)?;
    if !(0..=10_000).contains(&count) {
        return Err("GameObject contains an invalid component count".into());
    }
    let mut components = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let file_id = read_le_i32(raw, &mut cursor)?;
        let path_id = read_le_i64(raw, &mut cursor)?;
        if file_id == 0 {
            components.push(path_id);
        }
    }
    cursor += 4; // layer
    let name = read_aligned_string(raw, &mut cursor)?;
    Ok((name, components))
}

fn find_mono_script_path(global_managers: &[u8], class_name: &str) -> Result<i64, String> {
    let objects = parse_unity_objects(global_managers)?;
    for object in objects.iter().filter(|object| object.class_id == 115) {
        if mono_script_class_name(global_managers, object)? == class_name {
            return Ok(object.path_id);
        }
    }
    Err(format!("Could not find MonoScript {class_name}"))
}

fn resolve_component_offset(
    bytes: &[u8],
    expected_script_path: i64,
    game_object_name: &str,
    relative_offset: usize,
) -> Result<usize, String> {
    let objects = parse_unity_objects(bytes)?;
    let game_object = objects
        .iter()
        .filter(|object| object.class_id == 1)
        .find_map(|object| {
            game_object_data(bytes, object)
                .ok()
                .filter(|(name, _)| name == game_object_name)
                .map(|data| (object, data.1))
        })
        .ok_or_else(|| format!("Could not find GameObject {game_object_name}"))?;

    for component_path in game_object.1 {
        let Some(component) = objects
            .iter()
            .find(|object| object.path_id == component_path && object.class_id == 114)
        else {
            continue;
        };
        let raw = object_bytes(bytes, component)?;
        if raw.len() < 28 {
            continue;
        }
        let owner_path = i64::from_le_bytes(raw[4..12].try_into().unwrap());
        let script_file = i32::from_le_bytes(raw[16..20].try_into().unwrap());
        let script_path = i64::from_le_bytes(raw[20..28].try_into().unwrap());
        if owner_path == game_object.0.path_id
            && script_file == 1
            && script_path == expected_script_path
        {
            if relative_offset
                .checked_add(4)
                .filter(|end| *end <= component.byte_size)
                .is_none()
            {
                return Err(format!(
                    "Component field +{relative_offset} is outside {game_object_name}'s {}-byte blob",
                    component.byte_size
                ));
            }
            return Ok(component.byte_start + relative_offset);
        }
    }
    Err(format!(
        "Could not find the requested component on {game_object_name}"
    ))
}

fn resolve_builtin_component_offset(
    bytes: &[u8],
    component_class_id: i32,
    game_object_name: &str,
    relative_offset: usize,
) -> Result<usize, String> {
    let objects = parse_unity_objects(bytes)?;
    let game_object = objects
        .iter()
        .filter(|object| object.class_id == 1)
        .find_map(|object| {
            game_object_data(bytes, object)
                .ok()
                .filter(|(name, _)| name == game_object_name)
                .map(|data| (object, data.1))
        })
        .ok_or_else(|| format!("Could not find GameObject {game_object_name}"))?;

    for component_path in game_object.1 {
        let Some(component) = objects.iter().find(|object| {
            object.path_id == component_path && object.class_id == component_class_id
        }) else {
            continue;
        };
        let raw = object_bytes(bytes, component)?;
        if raw.len() < 12 {
            continue;
        }
        let owner_path = i64::from_le_bytes(raw[4..12].try_into().unwrap());
        if owner_path == game_object.0.path_id {
            if relative_offset
                .checked_add(4)
                .filter(|end| *end <= component.byte_size)
                .is_none()
            {
                return Err(format!(
                    "Component field +{relative_offset} is outside {game_object_name}'s {}-byte blob",
                    component.byte_size
                ));
            }
            return Ok(component.byte_start + relative_offset);
        }
    }
    Err(format!(
        "Could not find component class {component_class_id} on {game_object_name}"
    ))
}

fn read_float(bytes: &[u8], offset: usize) -> Result<f32, String> {
    let slice = bytes
        .get(offset..offset + 4)
        .ok_or_else(|| format!("Địa chỉ {offset} nằm ngoài file"))?;
    Ok(f32::from_le_bytes(
        slice
            .try_into()
            .map_err(|_| "Could not read a float value".to_string())?,
    ))
}

fn write_float(bytes: &mut [u8], offset: usize, value: f32) -> Result<(), String> {
    if !value.is_finite() {
        return Err("Value must be a finite number".into());
    }
    let destination = bytes
        .get_mut(offset..offset + 4)
        .ok_or_else(|| format!("Địa chỉ {offset} nằm ngoài file"))?;
    destination.copy_from_slice(&value.to_le_bytes());
    Ok(())
}

fn consistent_value(
    resources: &[u8],
    shared: &[u8],
    targets: &[FloatTarget],
) -> Result<f32, String> {
    let first = targets
        .first()
        .ok_or_else(|| "Profile không có địa chỉ".to_string())?;
    let value = read_float(
        if first.file == RESOURCES_ASSETS {
            resources
        } else {
            shared
        },
        first.offset,
    )?;
    for target in &targets[1..] {
        let candidate = read_float(
            if target.file == RESOURCES_ASSETS {
                resources
            } else {
                shared
            },
            target.offset,
        )?;
        if (candidate - value).abs() > 0.0001 {
            return Err(format!(
                "Variants in {} do not contain consistent values; restore a clean file first",
                target.file
            ));
        }
    }
    Ok(value)
}

fn component_value(
    resources: &[u8],
    shared: &[u8],
    script_path: i64,
    target: &ComponentFloatTarget,
) -> Result<f32, String> {
    let bytes = if target.file == RESOURCES_ASSETS {
        resources
    } else {
        shared
    };
    let offset = resolve_component_offset(
        bytes,
        script_path,
        target.game_object,
        target.relative_offset,
    )?;
    read_float(bytes, offset)
}

fn consistent_component_value(
    resources: &[u8],
    shared: &[u8],
    script_path: i64,
    targets: &[ComponentFloatTarget],
) -> Result<f32, String> {
    let first = targets
        .first()
        .ok_or_else(|| "No Steed targets configured".to_string())?;
    let value = component_value(resources, shared, script_path, first)?;
    for target in &targets[1..] {
        let candidate = component_value(resources, shared, script_path, target)?;
        if (candidate - value).abs() > 0.0001 {
            return Err(format!(
                "Steed variants in {} do not contain consistent values; restore a clean file first",
                target.file
            ));
        }
    }
    Ok(value)
}

fn builtin_component_value(
    resources: &[u8],
    shared: &[u8],
    component_class_id: i32,
    target: &ComponentFloatTarget,
) -> Result<f32, String> {
    let bytes = if target.file == RESOURCES_ASSETS {
        resources
    } else {
        shared
    };
    let offset = resolve_builtin_component_offset(
        bytes,
        component_class_id,
        target.game_object,
        target.relative_offset,
    )?;
    read_float(bytes, offset)
}

fn consistent_builtin_component_value(
    resources: &[u8],
    shared: &[u8],
    component_class_id: i32,
    targets: &[ComponentFloatTarget],
) -> Result<f32, String> {
    let first = targets
        .first()
        .ok_or_else(|| "No component targets configured".to_string())?;
    let value = builtin_component_value(resources, shared, component_class_id, first)?;
    for target in &targets[1..] {
        let candidate = builtin_component_value(resources, shared, component_class_id, target)?;
        if (candidate - value).abs() > 0.0001 {
            return Err(format!(
                "Component variants in {} do not contain consistent values; restore a clean file first",
                target.file
            ));
        }
    }
    Ok(value)
}

fn primary_component_value(
    resources: &[u8],
    shared: &[u8],
    script_path: i64,
    targets: &[ComponentFloatTarget],
) -> Result<f32, String> {
    let target = targets
        .first()
        .ok_or_else(|| "No Steed targets configured".to_string())?;
    component_value(resources, shared, script_path, target)
}

fn read_asset_settings(directory: &Path) -> Result<AssetSettings, String> {
    validate_asset_directory(directory)?;
    let resources = fs::read(directory.join(RESOURCES_ASSETS)).map_err(|err| err.to_string())?;
    let shared = fs::read(directory.join(SHARED_ASSETS)).map_err(|err| err.to_string())?;
    let global_managers = fs::read(directory.join(GLOBAL_MANAGERS_ASSETS))
        .map_err(|err| format!("Could not read {GLOBAL_MANAGERS_ASSETS}: {err}"))?;
    let steed_script_path = find_mono_script_path(&global_managers, "Steed")?;
    let griffin_skill_script_path =
        find_mono_script_path(&global_managers, "PushAttackSteedAbility")?;
    let warhorse_skill_script_path =
        find_mono_script_path(&global_managers, "BuffUnitsSteedAbility")?;
    let archer_script_path = find_mono_script_path(&global_managers, "Archer")?;
    let worker_script_path = find_mono_script_path(&global_managers, "Worker")?;
    Ok(AssetSettings {
        griffin_run_speed: consistent_component_value(
            &resources,
            &shared,
            steed_script_path,
            &GRIFFIN_RUN,
        )?,
        griffin_forest_multiplier: consistent_component_value(
            &resources,
            &shared,
            steed_script_path,
            &GRIFFIN_FOREST,
        )?,
        griffin_run_stamina_rate: consistent_component_value(
            &resources,
            &shared,
            steed_script_path,
            &GRIFFIN_RUN_STAMINA,
        )?,
        griffin_skill_stamina_cost: consistent_component_value(
            &resources,
            &shared,
            griffin_skill_script_path,
            &GRIFFIN_SKILL_COST,
        )?,
        // Older ktc-edit builds only changed the three resources.assets variants.
        // Read the primary variant so users can open those files and use Apply to
        // synchronize the previously omitted sharedassets0.assets prefab.
        horse_run_speed: primary_component_value(
            &resources,
            &shared,
            steed_script_path,
            &HORSE_RUN,
        )?,
        horse_run_stamina_rate: primary_component_value(
            &resources,
            &shared,
            steed_script_path,
            &HORSE_RUN_STAMINA,
        )?,
        warhorse_run_speed: consistent_component_value(
            &resources,
            &shared,
            steed_script_path,
            &WARHORSE_RUN,
        )?,
        warhorse_run_stamina_rate: consistent_component_value(
            &resources,
            &shared,
            steed_script_path,
            &WARHORSE_RUN_STAMINA,
        )?,
        warhorse_skill_stamina_cost: consistent_component_value(
            &resources,
            &shared,
            warhorse_skill_script_path,
            &WARHORSE_SKILL_COST,
        )?,
        warhorse_cooldown: consistent_value(&resources, &shared, &WARHORSE_COOLDOWN)?,
        warhorse_plague_cooldown: consistent_value(&resources, &shared, &WARHORSE_PLAGUE_COOLDOWN)?,
        warhorse_buff_duration: consistent_value(&resources, &shared, &WARHORSE_DURATION)?,
        warhorse_buff_range: consistent_value(&resources, &shared, &WARHORSE_RANGE)?,
        archer_shoot_prep_time: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_SHOOT_PREP,
        )?,
        archer_shoot_cooldown_time: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_SHOOT_COOLDOWN,
        )?,
        archer_shoot_cooldown_with_knight_time: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_KNIGHT_COOLDOWN,
        )?,
        archer_interval_min: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_INTERVAL_MIN,
        )?,
        archer_interval_max: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_INTERVAL_MAX,
        )?,
        archer_formation_interval_min: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_FORMATION_INTERVAL_MIN,
        )?,
        archer_formation_interval_max: consistent_component_value(
            &resources,
            &shared,
            archer_script_path,
            &ARCHER_FORMATION_INTERVAL_MAX,
        )?,
        builder_walk_speed: consistent_component_value(
            &resources,
            &shared,
            worker_script_path,
            &BUILDER_WALK_SPEED,
        )?,
        builder_run_speed: consistent_component_value(
            &resources,
            &shared,
            worker_script_path,
            &BUILDER_RUN_SPEED,
        )?,
        builder_work_time: consistent_component_value(
            &resources,
            &shared,
            worker_script_path,
            &BUILDER_WORK_TIME,
        )?,
        bag_scale: consistent_builtin_component_value(&resources, &shared, 4, &BAG_SCALE)?,
    })
}

#[tauri::command]
fn load_game_assets(data_directory: Option<String>) -> Result<AssetResponse, String> {
    let directory = data_directory
        .map(PathBuf::from)
        .unwrap_or_else(default_game_data_directory);
    let settings = read_asset_settings(&directory)?;
    Ok(AssetResponse {
        data_directory: directory.to_string_lossy().to_string(),
        settings,
    })
}

#[tauri::command]
fn select_game_data_directory() -> Result<AssetResponse, String> {
    let Some(directory) = FileDialog::new()
        .set_title("Select the Kingdom Two Crowns Data folder")
        .pick_folder()
    else {
        return Err("Folder selection was cancelled".into());
    };
    load_game_assets(Some(directory.to_string_lossy().to_string()))
}

fn write_targets(
    resources: &mut [u8],
    shared: &mut [u8],
    targets: &[FloatTarget],
    value: f32,
) -> Result<(), String> {
    for target in targets {
        write_float(
            if target.file == RESOURCES_ASSETS {
                resources
            } else {
                shared
            },
            target.offset,
            value,
        )?;
    }
    Ok(())
}

fn write_component_targets(
    resources: &mut [u8],
    shared: &mut [u8],
    script_path: i64,
    targets: &[ComponentFloatTarget],
    value: f32,
) -> Result<(), String> {
    for target in targets {
        let bytes = if target.file == RESOURCES_ASSETS {
            &mut *resources
        } else {
            &mut *shared
        };
        let offset = resolve_component_offset(
            bytes,
            script_path,
            target.game_object,
            target.relative_offset,
        )?;
        write_float(bytes, offset, value)?;
    }
    Ok(())
}

fn write_builtin_component_targets(
    resources: &mut [u8],
    shared: &mut [u8],
    component_class_id: i32,
    targets: &[ComponentFloatTarget],
    value: f32,
) -> Result<(), String> {
    for target in targets {
        let bytes = if target.file == RESOURCES_ASSETS {
            &mut *resources
        } else {
            &mut *shared
        };
        let offset = resolve_builtin_component_offset(
            bytes,
            component_class_id,
            target.game_object,
            target.relative_offset,
        )?;
        write_float(bytes, offset, value)?;
    }
    Ok(())
}

fn validate_settings(settings: &AssetSettings) -> Result<(), String> {
    let values = [
        settings.griffin_run_speed,
        settings.griffin_forest_multiplier,
        settings.griffin_run_stamina_rate,
        settings.griffin_skill_stamina_cost,
        settings.horse_run_speed,
        settings.horse_run_stamina_rate,
        settings.warhorse_run_speed,
        settings.warhorse_run_stamina_rate,
        settings.warhorse_skill_stamina_cost,
        settings.warhorse_cooldown,
        settings.warhorse_plague_cooldown,
        settings.warhorse_buff_duration,
        settings.warhorse_buff_range,
        settings.archer_shoot_prep_time,
        settings.archer_shoot_cooldown_time,
        settings.archer_shoot_cooldown_with_knight_time,
        settings.archer_interval_min,
        settings.archer_interval_max,
        settings.archer_formation_interval_min,
        settings.archer_formation_interval_max,
        settings.builder_walk_speed,
        settings.builder_run_speed,
        settings.builder_work_time,
        settings.bag_scale,
    ];
    if values
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0 || *value > 1000.0)
    {
        return Err("Every value must be between 0 and 1000".into());
    }
    if settings.archer_interval_min > settings.archer_interval_max {
        return Err("Archer normal interval minimum cannot exceed its maximum".into());
    }
    if settings.archer_formation_interval_min > settings.archer_formation_interval_max {
        return Err("Archer formation interval minimum cannot exceed its maximum".into());
    }
    Ok(())
}

fn atomic_replace(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let temp = path.with_extension("ktcedit.tmp");
    fs::write(&temp, bytes).map_err(|err| format!("Could not write the temporary file: {err}"))?;
    fs::rename(&temp, path).map_err(|err| format!("Could not replace {}: {err}", path.display()))
}

#[tauri::command]
fn apply_game_assets(
    data_directory: String,
    settings: AssetSettings,
) -> Result<AssetApplyResponse, String> {
    validate_settings(&settings)?;
    if game_is_running() {
        return Err("Close Kingdom Two Crowns before applying changes".into());
    }
    let directory = PathBuf::from(data_directory);
    let _current = read_asset_settings(&directory)?;
    let resources_path = directory.join(RESOURCES_ASSETS);
    let shared_path = directory.join(SHARED_ASSETS);
    let mut resources = fs::read(&resources_path).map_err(|err| err.to_string())?;
    let mut shared = fs::read(&shared_path).map_err(|err| err.to_string())?;
    let global_managers = fs::read(directory.join(GLOBAL_MANAGERS_ASSETS))
        .map_err(|err| format!("Could not read {GLOBAL_MANAGERS_ASSETS}: {err}"))?;
    let steed_script_path = find_mono_script_path(&global_managers, "Steed")?;
    let griffin_skill_script_path =
        find_mono_script_path(&global_managers, "PushAttackSteedAbility")?;
    let warhorse_skill_script_path =
        find_mono_script_path(&global_managers, "BuffUnitsSteedAbility")?;
    let archer_script_path = find_mono_script_path(&global_managers, "Archer")?;
    let worker_script_path = find_mono_script_path(&global_managers, "Worker")?;

    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &GRIFFIN_RUN,
        settings.griffin_run_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &GRIFFIN_FOREST,
        settings.griffin_forest_multiplier,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &GRIFFIN_RUN_STAMINA,
        settings.griffin_run_stamina_rate,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        griffin_skill_script_path,
        &GRIFFIN_SKILL_COST,
        settings.griffin_skill_stamina_cost,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &HORSE_RUN,
        settings.horse_run_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &HORSE_RUN_STAMINA,
        settings.horse_run_stamina_rate,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &WARHORSE_RUN,
        settings.warhorse_run_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        steed_script_path,
        &WARHORSE_RUN_STAMINA,
        settings.warhorse_run_stamina_rate,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        warhorse_skill_script_path,
        &WARHORSE_SKILL_COST,
        settings.warhorse_skill_stamina_cost,
    )?;
    write_targets(
        &mut resources,
        &mut shared,
        &WARHORSE_COOLDOWN,
        settings.warhorse_cooldown,
    )?;
    write_targets(
        &mut resources,
        &mut shared,
        &WARHORSE_PLAGUE_COOLDOWN,
        settings.warhorse_plague_cooldown,
    )?;
    write_targets(
        &mut resources,
        &mut shared,
        &WARHORSE_DURATION,
        settings.warhorse_buff_duration,
    )?;
    write_targets(
        &mut resources,
        &mut shared,
        &WARHORSE_RANGE,
        settings.warhorse_buff_range,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_SHOOT_PREP,
        settings.archer_shoot_prep_time,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_SHOOT_COOLDOWN,
        settings.archer_shoot_cooldown_time,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_KNIGHT_COOLDOWN,
        settings.archer_shoot_cooldown_with_knight_time,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_INTERVAL_MIN,
        settings.archer_interval_min,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_INTERVAL_MAX,
        settings.archer_interval_max,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_FORMATION_INTERVAL_MIN,
        settings.archer_formation_interval_min,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        archer_script_path,
        &ARCHER_FORMATION_INTERVAL_MAX,
        settings.archer_formation_interval_max,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        worker_script_path,
        &BUILDER_WALK_SPEED,
        settings.builder_walk_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        worker_script_path,
        &BUILDER_RUN_SPEED,
        settings.builder_run_speed,
    )?;
    write_component_targets(
        &mut resources,
        &mut shared,
        worker_script_path,
        &BUILDER_WORK_TIME,
        settings.builder_work_time,
    )?;
    write_builtin_component_targets(
        &mut resources,
        &mut shared,
        4,
        &BAG_SCALE,
        settings.bag_scale,
    )?;

    let resources_backup = create_backup(&resources_path)?;
    let shared_backup = create_backup(&shared_path)?;
    atomic_replace(&resources_path, &resources)?;
    if let Err(error) = atomic_replace(&shared_path, &shared) {
        let _ = fs::copy(&resources_backup, &resources_path);
        return Err(error);
    }

    let written_resources = fs::read(&resources_path).map_err(|err| err.to_string())?;
    let written_shared = fs::read(&shared_path).map_err(|err| err.to_string())?;
    let verified_horse_run = consistent_component_value(
        &written_resources,
        &written_shared,
        steed_script_path,
        &HORSE_RUN,
    )?;
    let verified_horse_stamina = consistent_component_value(
        &written_resources,
        &written_shared,
        steed_script_path,
        &HORSE_RUN_STAMINA,
    )?;
    if (verified_horse_run - settings.horse_run_speed).abs() > 0.0001
        || (verified_horse_stamina - settings.horse_run_stamina_rate).abs() > 0.0001
    {
        return Err("Regular Horse values were not written consistently".into());
    }

    let verified = read_asset_settings(&directory)?;
    Ok(AssetApplyResponse {
        resources_backup: resources_backup.to_string_lossy().to_string(),
        shared_assets_backup: shared_backup.to_string_lossy().to_string(),
        settings: verified,
    })
}

fn game_is_running() -> bool {
    #[cfg(target_os = "windows")]
    {
        return std::process::Command::new("tasklist")
            .output()
            .map(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .to_lowercase()
                    .contains("kingdomtwocrowns")
            })
            .unwrap_or(false);
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("pgrep")
            .args(["-if", "KingdomTwoCrowns"])
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }
}

#[tauri::command]
fn restore_game_assets(
    data_directory: String,
    resources_backup: String,
    shared_assets_backup: String,
) -> Result<AssetResponse, String> {
    if game_is_running() {
        return Err("Close Kingdom Two Crowns before restoring a backup".into());
    }
    let directory = PathBuf::from(data_directory);
    let resources_backup = PathBuf::from(resources_backup);
    let shared_backup = PathBuf::from(shared_assets_backup);
    let resources_bytes = fs::read(&resources_backup)
        .map_err(|err| format!("Could not read the resources backup: {err}"))?;
    let shared_bytes = fs::read(&shared_backup)
        .map_err(|err| format!("Could not read the sharedassets backup: {err}"))?;
    if resources_bytes.len() as u64 != EXPECTED_RESOURCES_SIZE
        || shared_bytes.len() as u64 != EXPECTED_SHARED_SIZE
    {
        return Err("The backup file sizes are invalid".into());
    }
    let resources_path = directory.join(RESOURCES_ASSETS);
    let shared_path = directory.join(SHARED_ASSETS);
    let _safety_resources = create_backup(&resources_path)?;
    let _safety_shared = create_backup(&shared_path)?;
    atomic_replace(&resources_path, &resources_bytes)?;
    atomic_replace(&shared_path, &shared_bytes)?;
    load_game_assets(Some(directory.to_string_lossy().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::{read::GzDecoder, write::GzEncoder, Compression};
    use serde_json::json;
    use std::{
        fs,
        io::{Read, Write},
        path::{Path, PathBuf},
    };
    use tempfile::tempdir;

    fn write_test_save(dir: &Path, payload: &serde_json::Value) -> PathBuf {
        let path = dir.join(SAVE_FILENAME);
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        let json_bytes = serde_json::to_vec(payload).expect("serialize payload");
        encoder
            .write_all(&json_bytes)
            .expect("write compressed payload");
        let compressed = encoder.finish().expect("finish compression");
        fs::write(&path, compressed).expect("write save file");
        path
    }

    fn read_save(path: &Path) -> serde_json::Value {
        let file = fs::File::open(path).expect("open save file");
        let mut decoder = GzDecoder::new(file);
        let mut json = String::new();
        decoder
            .read_to_string(&mut json)
            .expect("decompress save file");
        serde_json::from_str(&json).expect("parse json")
    }

    #[test]
    fn validate_path_accepts_exact_name() {
        let dir = tempdir().expect("temp dir");
        let path = dir.path().join(SAVE_FILENAME);
        fs::write(&path, b"test").expect("write file");
        assert!(validate_path(&path).is_ok());
    }

    #[test]
    fn validate_path_rejects_other_names() {
        let dir = tempdir().expect("temp dir");
        let path = dir.path().join("global-v35.bak");
        fs::write(&path, b"test").expect("write file");
        assert!(validate_path(&path).is_err());
    }

    #[test]
    fn load_save_file_returns_json_payload() {
        let dir = tempdir().expect("temp dir");
        let expected = json!({
            "serializedSaveDataVersion": 35,
            "campaigns": [
                {
                    "id": 1,
                    "progress": 42
                }
            ]
        });
        let path = write_test_save(dir.path(), &expected);
        let path_string = path.to_string_lossy().to_string();

        let response = load_save_file(path_string.clone()).expect("load save file");

        assert_eq!(response.path, path_string);
        assert_eq!(response.data, expected);
    }

    #[test]
    fn save_save_file_creates_backup_and_updates_contents() {
        let dir = tempdir().expect("temp dir");
        let original = json!({ "value": 10, "flags": [true, false] });
        let path = write_test_save(dir.path(), &original);
        let path_string = path.to_string_lossy().to_string();

        let updated = json!({ "value": 99, "flags": [false], "note": "updated" });
        let backup_path =
            save_save_file(path_string.clone(), updated.clone()).expect("save save file");
        let backup = PathBuf::from(&backup_path);

        assert!(backup.exists(), "backup file should exist");

        let backup_contents = read_save(&backup);
        assert_eq!(backup_contents, original);

        let rewritten = read_save(&path);
        assert_eq!(rewritten, updated);
    }

    #[test]
    fn regular_horse_targets_are_relative_and_include_default_prefab() {
        assert_eq!(HORSE_RUN.len(), 4);
        assert_eq!(HORSE_RUN_STAMINA.len(), 4);
        assert!(HORSE_RUN.iter().any(|target| target.file == SHARED_ASSETS
            && target.game_object == "Horse Regular"
            && target.relative_offset == 196));
        assert!(HORSE_RUN_STAMINA
            .iter()
            .any(|target| target.file == SHARED_ASSETS
                && target.game_object == "Horse Regular"
                && target.relative_offset == 208));
    }

    #[test]
    fn resolves_component_fields_from_installed_assets_when_available() {
        let directory = default_game_data_directory();
        let Ok(resources) = fs::read(directory.join(RESOURCES_ASSETS)) else {
            return;
        };
        let Ok(shared) = fs::read(directory.join(SHARED_ASSETS)) else {
            return;
        };
        let Ok(global_managers) = fs::read(directory.join(GLOBAL_MANAGERS_ASSETS)) else {
            return;
        };
        let script = find_mono_script_path(&global_managers, "Steed").expect("Steed script");
        for target in GRIFFIN_RUN
            .iter()
            .chain(HORSE_RUN.iter())
            .chain(WARHORSE_RUN.iter())
        {
            let bytes = if target.file == RESOURCES_ASSETS {
                &resources
            } else {
                &shared
            };
            let offset =
                resolve_component_offset(bytes, script, target.game_object, target.relative_offset)
                    .expect("dynamic Steed field");
            assert!(read_float(bytes, offset).expect("Steed float").is_finite());
        }
        for (class_name, targets) in [
            ("PushAttackSteedAbility", GRIFFIN_SKILL_COST.as_slice()),
            ("BuffUnitsSteedAbility", WARHORSE_SKILL_COST.as_slice()),
            ("Archer", ARCHER_SHOOT_PREP.as_slice()),
            ("Worker", BUILDER_WALK_SPEED.as_slice()),
        ] {
            let script =
                find_mono_script_path(&global_managers, class_name).expect("ability script");
            for target in targets {
                let bytes = if target.file == RESOURCES_ASSETS {
                    &resources
                } else {
                    &shared
                };
                let offset = resolve_component_offset(
                    bytes,
                    script,
                    target.game_object,
                    target.relative_offset,
                )
                .expect("dynamic ability field");
                assert!(read_float(bytes, offset)
                    .expect("component float")
                    .is_finite());
            }
        }
        for target in &BAG_SCALE {
            let bytes = if target.file == RESOURCES_ASSETS {
                &resources
            } else {
                &shared
            };
            let offset = resolve_builtin_component_offset(
                bytes,
                4,
                target.game_object,
                target.relative_offset,
            )
            .expect("dynamic Transform field");
            assert!(read_float(bytes, offset)
                .expect("Transform float")
                .is_finite());
        }
        read_asset_settings(&directory).expect("installed asset settings should load");
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_save_file,
            save_save_file,
            select_save_file,
            load_game_assets,
            select_game_data_directory,
            apply_game_assets,
            restore_game_assets
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
