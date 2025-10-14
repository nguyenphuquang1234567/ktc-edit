use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use log::{debug, error, info};
use rfd::FileDialog;
use serde::Serialize;
use serde_json::Value;

const SAVE_FILENAME: &str = "global-v35";

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
        error!("Impossible d'ouvrir {}: {}", path_buf.display(), err);
        err.to_string()
    })?;
    let mut decoder = GzDecoder::new(file);
    let mut json = String::new();
    decoder.read_to_string(&mut json).map_err(|err| {
        error!(
            "Erreur lors de la décompression de {}: {}",
            path_buf.display(),
            err
        );
        err.to_string()
    })?;
    debug!(
        "Décompression terminée ({} octets JSON)",
        json.as_bytes().len()
    );
    let data: Value = serde_json::from_str(&json).map_err(|err| {
        error!("JSON invalide dans {}: {}", path_buf.display(), err);
        err.to_string()
    })?;
    if let Some(obj) = data.as_object() {
        info!("Fichier chargé avec {} clés de premier niveau", obj.len());
    }

    Ok(LoadResponse { path, data })
}

#[tauri::command]
fn select_save_file() -> Result<LoadResponse, String> {
    let Some(selected) = FileDialog::new()
        .set_title("Choisir global-v35")
        .pick_file()
    else {
        return Err("Sélection annulée".into());
    };

    let path = selected.to_string_lossy().to_string();
    info!("Fichier sélectionné: {}", path);
    load_save_file(path)
}

#[tauri::command]
fn save_save_file(path: String, data: Value) -> Result<String, String> {
    let path_buf = PathBuf::from(&path);
    validate_path(&path_buf)?;

    info!("Demande de sauvegarde pour {}", path_buf.display());

    let backup_path = create_backup(&path_buf)?;

    let json = serde_json::to_string_pretty(&data).map_err(|err| {
        error!(
            "Impossible de sérialiser les données pour {}: {}",
            path_buf.display(),
            err
        );
        err.to_string()
    })?;
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(json.as_bytes()).map_err(|err| {
        error!(
            "Erreur lors de la recompression de {}: {}",
            path_buf.display(),
            err
        );
        err.to_string()
    })?;
    let compressed = encoder.finish().map_err(|err| {
        error!(
            "Erreur lors de la finalisation de la compression pour {}: {}",
            path_buf.display(),
            err
        );
        err.to_string()
    })?;

    log_resource_summaries(&data);

    info!(
        "Sauvegarde: JSON {} octets, gzip {} octets",
        json.as_bytes().len(),
        compressed.len()
    );

    fs::write(&path_buf, compressed).map_err(|err| {
        error!("Impossible d'écrire {}: {}", path_buf.display(), err);
        err.to_string()
    })?;

    info!(
        "Sauvegarde appliquée sur {}, sauvegarde précédente: {}",
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
                    "Campagne {}: coins P1 = {}, coins P2 = {}, gems P1 = {}, gems P2 = {}",
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
                            "Campagne {} île {}: archers={}, ouvriers={}, fermiers={}, piquiers={}",
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
        return Err("Nom de fichier invalide".into());
    };

    debug!("Validation du fichier: {}", name);
    if name != SAVE_FILENAME {
        return Err(format!(
            "Le fichier sélectionné ({name}) n'est pas {SAVE_FILENAME}"
        ));
    }

    Ok(())
}

fn create_backup(original: &Path) -> Result<PathBuf, String> {
    let parent = original
        .parent()
        .ok_or_else(|| "Impossible de déterminer le dossier du fichier".to_string())?;
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
            "Impossible de créer la sauvegarde {} -> {}: {}",
            original.display(),
            backup_path.display(),
            err
        );
        err.to_string()
    })?;

    info!("Copie de sauvegarde créée: {}", backup_path.display());

    Ok(backup_path)
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
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_save_file,
            save_save_file,
            select_save_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
