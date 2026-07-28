use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::LazyLock;

use crate::steam::{Envars, SteamId};

pub static DEFAULT_OVERLAYS: LazyLock<HashMap<String, Overlay>> = LazyLock::new(|| {
    let overlays = [
        "292030|The Witcher 3|%STEAMUSER%/Documents/The Witcher 3|dx12user.settings,user.settings",
        "499450|The Witcher 3|%STEAMUSER%/Documents/The Witcher 3|user.settings",
        "814380|Sekiro|%STEAMUSER%/AppData/Roaming/Sekiro|GraphicsConfig.xml",
        "524220|NieR Automata|%STEAMUSER%/Documents/My Games/NieR_Automata|SystemData.dat",
        "1113560|NieR Replicant|%STEAMUSER%/Documents/My Games/NieR Replicant ver.1.22474487139/Steam/%STEAMID%|drawing_settings.ini",
        "1151640|Horizon Zero Dawn|%STEAMUSER%/Documents/Horizon Zero Dawn/Saved Game/profile|graphicsconfig.ini",
        "1245620|ELDEN RING|%STEAMUSER%/AppData/Roaming/EldenRing|GraphicsConfig.xml",
        "1903340|Expedition 33|%STEAMUSER%/AppData/Local/Sandfall/Saved/SaveGames/%STEAMID%|SharedGameUserSettings.sav",
    ];
    overlays
        .into_iter()
        .map(Overlay::parse)
        .collect::<Option<_>>()
        .expect("some")
});

/// Replace placeholders in the config directory path to make an absolute path.
pub fn replace_placeholders(envars: &Envars, steam_id: &SteamId, overlay: &Overlay) -> PathBuf {
    let replacements = [
        ("%STEAMID%", steam_id),
        ("%STEAMUSER%", &format!("{}/pfx/drive_c/users/steamuser", envars.compat_data_path)),
        ("%STEAM_COMPAT_DATA_PATH%", &envars.compat_data_path),
        ("%STEAM_COMPAT_INSTALL_PATH%", &envars.compat_install_path),
    ];
    replacements
        .iter()
        .fold(overlay.cfg_dir.clone(), |acc, (from, to)| acc.replace(from, to))
        .into()
}

#[derive(Debug, Clone)]
pub struct Overlay {
    pub app_name: String,
    pub cfg_dir: String,
    pub cfg_files: Vec<String>,
}

impl Overlay {
    pub fn parse(s: &str) -> Option<(String, Self)> {
        let mut s = s.split('|');
        let app_id = s.next()?.to_owned();
        let overlay = Self {
            app_name: s.next()?.to_owned(),
            cfg_dir: s.next()?.to_owned(),
            cfg_files: s.next()?.split(',').map(ToOwned::to_owned).collect(),
        };
        Some((app_id, overlay))
    }
}
