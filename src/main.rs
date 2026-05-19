use std::fs;
use std::process::Command;

use anyhow::{Context as _, bail};
use clap::Parser as _;
use directories::BaseDirs;

use sgco::cli::CliArgs;
use sgco::overlays::{DEFAULT_OVERLAYS, Overlay, replace_placeholders};
use sgco::steam::{Envars, steam_id};

fn main() {
    run().unwrap();
}

fn run() -> anyhow::Result<()> {
    let cli_args = CliArgs::try_parse()?;
    if cli_args.cmd.is_empty() {
        bail!("Game launch command is not provided!");
    }
    let envars = Envars::parse()?;
    let steam_id = steam_id(&envars.user)?;
    let overlay = match cli_args.r#override {
        Some(o) => {
            let (app_id, overlay) = Overlay::parse(&o)
                .with_context(|| format!("Failed to parse overlay string: {o}"))?;
            if app_id != envars.app_id {
                bail!("SteamAppId mismatch: envar={} override={}", envars.app_id, app_id);
            }
            overlay
        }
        None => DEFAULT_OVERLAYS
            .get(&envars.app_id)
            .cloned()
            .with_context(|| format!("No config found for SteamAppId={}!", envars.app_id))?,
    };
    let app_cfg_dir = replace_placeholders(&envars, &steam_id, &overlay);
    let overlay_cfg_dir = BaseDirs::new()
        .context("reading base dirs")?
        .config_dir()
        .join("sgco")
        .join(format!("{} - {}", envars.app_id, overlay.app_name));
    let mut bwrap = Command::new("bwrap");
    bwrap.arg("--dev-bind").arg("/").arg("/");
    let mut new_cfgs = Vec::new();
    for file in &overlay.cfg_files {
        let app_cfg = app_cfg_dir.join(file);
        let overlay_cfg = overlay_cfg_dir.join(file);
        if let Some(parent) = overlay_cfg.parent() {
            fs::create_dir_all(parent).context("create overlay dirs")?;
        }
        match (app_cfg.exists(), overlay_cfg.exists()) {
            (_, true) => {
                // We have the overlay file
            }
            (true, false) => {
                // There is existing config file and we don't have its overlay
                fs::copy(&app_cfg, &overlay_cfg).context("copy initial overlay file")?;
            }
            (false, false) => {
                // No config file or its overlay file, skip binding and copy to new overlay later
                new_cfgs.push(file.as_str());
                continue;
            }
        }
        bwrap.arg("--bind").arg(overlay_cfg).arg(app_cfg);
    }
    bwrap.arg("--").args(cli_args.cmd);
    let _ = bwrap
        .spawn()
        .context("spawn child process")?
        .wait()
        .context("wait for child process to exit")?;
    for file in new_cfgs {
        let app_cfg = app_cfg_dir.join(file);
        if app_cfg.exists() {
            fs::copy(&app_cfg, overlay_cfg_dir.join(file))
                .context("copy newly created config file")?;
        }
    }
    Ok(())
}
