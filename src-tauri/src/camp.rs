//! Build-specific ARM64 guard, identical to the separately tested camp patch.
use super::*;
const HOOK: usize = 0x660dbc;
const CAVE: usize = 0x44c6d00;
const CONTINUE: usize = 0x660dc0;
const FADE: usize = 0x661a88;

#[derive(Serialize)]
pub struct Response {
    enabled: bool,
    backup: Option<String>,
}

fn branch(a: usize, z: usize, link: bool) -> u32 {
    (if link { 0x94000000 } else { 0x14000000 }) | (((z as i64 - a as i64) / 4) as u32 & 0x3ffffff)
}
fn code() -> Vec<u8> {
    [
        0xf9402008_u32,
        0xb40002e8,
        0x1000000a,
        0xd28da10b,
        0xf2a0898b,
        0xcb0b014a,
        0xd2825a0b,
        0xf2a00e0b,
        0x8b0b014a,
        0xf9400909,
        0xeb0a013f,
        0x540001c0,
        0xf9403d08,
        0xb4000168,
        0xb940190c,
        0x91008108,
        0x3400010c,
        0xf8408509,
        0xb4000089,
        0xf9400929,
        0xeb0a013f,
        0x54000080,
        0x5100058c,
        0x17fffff9,
        branch(CAVE + 0x60, FADE, true),
        branch(CAVE + 0x64, CONTINUE, false),
    ]
    .into_iter()
    .flat_map(u32::to_le_bytes)
    .collect()
}
fn word(b: &[u8], p: usize) -> Result<u32, String> {
    Ok(u32::from_le_bytes(
        b.get(p..p + 4)
            .ok_or("Truncated camp code")?
            .try_into()
            .unwrap(),
    ))
}
fn locate(b: &[u8]) -> Result<usize, String> {
    let off = coin_bag::locate(b)?;
    // The exact loop, callback construction and delegate layout inspected for this build.
    for (a, w) in [
        (0x660dac, 0x394162a8),
        (0x660db0, 0x3707fb68),
        (0x660db4, 0x2f00e400),
        (0x660db8, 0xaa1503e0),
        (CONTINUE, 0x17ffffd7),
        (0x701058, 0xf9400102),
        (0x70105c, 0xaa1303e1),
        (0x701064, 0x9487e085),
        (0x701074, 0x948c4a42),
        (0x28f9290, 0xf9400448),
        (0x28f9294, 0xf9000808),
    ] {
        if word(b, off + a)? != w {
            return Err("Unsupported Beggar Camp native code".into());
        }
    }
    if b.get(off + 0x4906b90..off + 0x4906b98) != Some(&0x60000977_u64.to_le_bytes()) {
        return Err("Unsupported camp callback metadata reference".into());
    }
    // coin_bag::locate verifies all sections end before its earlier cave.
    // Check our later guard also remains inside the executable __TEXT file range.
    let mut p = off + 32;
    let mut fits = false;
    for _ in 0..word(b, off + 16)? {
        let cmd = word(b, p)?;
        let size = word(b, p + 4)? as usize;
        if size < 8 {
            return Err("Invalid Mach-O command".into());
        }
        if cmd == 25 && b.get(p + 8..p + 14) == Some(b"__TEXT") {
            let fs = u64::from_le_bytes(
                b.get(p + 48..p + 56)
                    .ok_or("Truncated segment")?
                    .try_into()
                    .unwrap(),
            );
            fits = fs >= (CAVE + 104) as u64 && word(b, p + 60)? & 4 != 0;
        }
        p = p.checked_add(size).ok_or("Invalid Mach-O layout")?;
    }
    if !fits {
        return Err("Camp guard does not fit executable padding".into());
    }
    Ok(off)
}
fn read(b: &[u8]) -> Result<bool, String> {
    let off = locate(b)?;
    let guard = b
        .get(off + CAVE..off + CAVE + 104)
        .ok_or("Truncated guard")?;
    if word(b, off + HOOK)? == branch(HOOK, FADE, true) && guard.iter().all(|x| *x == 0) {
        return Ok(false);
    }
    if word(b, off + HOOK)? == branch(HOOK, CAVE, false) && guard == code() {
        return Ok(true);
    }
    Err("Unrecognized camp patch or occupied code cave".into())
}
fn patch(b: &mut [u8], enabled: bool) -> Result<(), String> {
    read(b)?;
    let off = locate(b)?;
    let hook = if enabled {
        branch(HOOK, CAVE, false)
    } else {
        branch(HOOK, FADE, true)
    };
    b[off + HOOK..off + HOOK + 4].copy_from_slice(&hook.to_le_bytes());
    b[off + CAVE..off + CAVE + 104].copy_from_slice(&if enabled { code() } else { vec![0; 104] });
    Ok(())
}
fn check_metadata(directory: &Path) -> Result<(), String> {
    let b = fs::read(directory.join("il2cpp_data/Metadata/global-metadata.dat"))
        .map_err(|e| e.to_string())?;
    let strings = word(&b, 24)? as usize;
    let methods = word(&b, 48)? as usize;
    let name = strings
        .checked_add(word(&b, methods + 1211 * 36)? as usize)
        .ok_or("Invalid metadata")?;
    if b.get(name..name + 17) != Some(b"SpawnCitizenHouse") {
        return Err("Unsupported camp callback metadata".into());
    }
    Ok(())
}
#[tauri::command]
pub fn load_camp_preservation(data_directory: String) -> Result<Response, String> {
    if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        return Err("Apple Silicon Macs only".into());
    }
    let dir = Path::new(&data_directory);
    let p = game_assembly_path(dir)?;
    check_metadata(dir)?;
    Ok(Response {
        enabled: read(&fs::read(p).map_err(|e| e.to_string())?)?,
        backup: None,
    })
}
#[tauri::command]
pub fn apply_camp_preservation(data_directory: String, enabled: bool) -> Result<Response, String> {
    use std::process::Command;
    if !cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        return Err("Apple Silicon Macs only".into());
    }
    if game_is_running() {
        return Err("Close Kingdom Two Crowns before applying changes".into());
    }
    let dir = Path::new(&data_directory);
    let p = game_assembly_path(dir)?;
    check_metadata(dir)?;
    let mut b = fs::read(&p).map_err(|e| e.to_string())?;
    if read(&b)? == enabled {
        return Ok(Response {
            enabled,
            backup: None,
        });
    }
    patch(&mut b, enabled)?;
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_nanos();
    let backup = p.with_file_name(format!("{GAME_ASSEMBLY}.ktcedit.{stamp}.bak"));
    let temp = p.with_file_name(format!("{GAME_ASSEMBLY}.ktcedit.{stamp}.tmp"));
    fs::copy(&p, &backup).map_err(|e| e.to_string())?;
    let result = (|| -> Result<(), String> {
        fs::write(&temp, &b).map_err(|e| e.to_string())?;
        fs::set_permissions(
            &temp,
            fs::metadata(&p).map_err(|e| e.to_string())?.permissions(),
        )
        .map_err(|e| e.to_string())?;
        for args in [vec!["--force", "--sign", "-"], vec!["--verify", "--strict"]] {
            let r = Command::new("codesign")
                .args(args)
                .arg(&temp)
                .output()
                .map_err(|e| e.to_string())?;
            if !r.status.success() {
                return Err(format!(
                    "Code signing failed: {}",
                    String::from_utf8_lossy(&r.stderr)
                ));
            }
        }
        if read(&fs::read(&temp).map_err(|e| e.to_string())?)? != enabled {
            return Err("Camp patch verification failed".into());
        }
        fs::rename(&temp, &p).map_err(|e| e.to_string())?;
        Ok(())
    })();
    let _ = fs::remove_file(&temp);
    result.map_err(|e| format!("{e}. Original file unchanged. Backup: {}", backup.display()))?;
    Ok(Response {
        enabled,
        backup: Some(backup.to_string_lossy().into()),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip_and_preserve_coin_patch() {
        let p=Path::new("/Users/quangvictornguyen/Library/Application Support/Steam/steamapps/common/Kingdom Two Crowns/KingdomTwoCrowns.app/Contents/Frameworks/GameAssembly.dylib");
        if !p.exists() {
            return;
        }
        let mut b = fs::read(p).unwrap();
        let before = b.clone();
        let state = read(&b).unwrap();
        let off = locate(&b).unwrap();
        patch(&mut b, !state).unwrap();
        assert_eq!(read(&b).unwrap(), !state);
        assert_eq!(
            &b[off + 0x44c6c90..off + 0x44c6ca8],
            &before[off + 0x44c6c90..off + 0x44c6ca8]
        );
        assert!(b.iter().zip(&before).enumerate().all(|(i, (a, z))| a == z
            || (off + HOOK..off + HOOK + 4).contains(&i)
            || (off + CAVE..off + CAVE + 104).contains(&i)));
        patch(&mut b, state).unwrap();
        assert_eq!(b, before);
        b[off + CONTINUE] ^= 1;
        assert!(read(&b).is_err());
    }
    #[test]
    fn matches_gameplay_tested_artifact() {
        let p =
            Path::new("/Users/quangvictornguyen/Documents/Codex/2026-09-19/l/camp-patch/guard.bin");
        if p.exists() {
            assert_eq!(code(), fs::read(p).unwrap());
        }
    }
}
