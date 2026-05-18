use std::collections::HashMap;
use std::{env, fs};

use anyhow::{Context, anyhow};
use directories::BaseDirs;
use serde::Deserialize;
use serde_with::{BoolFromInt, serde_as};

pub type SteamId = String;

pub fn steam_id(username: &str) -> anyhow::Result<SteamId> {
    let base_dirs = BaseDirs::new().context("read base dirs")?;
    let login_users = base_dirs
        .home_dir()
        .join(".local/share/Steam/config/loginusers.vdf");
    let users = fs::read_to_string(&login_users).context("reading loginusers.vdf")?;
    let users: Users = vdf_serde::from_str(&users).context("parsing loginusers.vdf")?;
    for (id, info) in users.0 {
        if info.account_name == username {
            return Ok(id);
        }
    }
    Err(anyhow!("Username is not found!"))
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "users")]
pub struct Users(HashMap<SteamId, User>);

#[serde_as]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub account_name: String,
    pub persona_name: String,
    #[serde_as(as = "BoolFromInt")]
    pub remember_password: bool,
    #[serde_as(as = "BoolFromInt")]
    pub wants_offline_mode: bool,
    #[serde_as(as = "BoolFromInt")]
    pub skip_offline_mode_warning: bool,
    #[serde_as(as = "BoolFromInt")]
    pub allow_auto_login: bool,
    #[serde_as(as = "BoolFromInt")]
    pub most_recent: bool,
    pub timestamp: u64,
}
#[derive(Debug, Clone)]
pub struct Envars {
    pub user: String,
    pub app_id: String,
    pub compat_data_path: String,
    pub compat_install_path: String,
}

impl Envars {
    pub fn parse() -> anyhow::Result<Self> {
        return Ok(Self {
            user: envar("SteamUser")?,
            app_id: envar("SteamAppId")?,
            compat_data_path: envar("STEAM_COMPAT_DATA_PATH")?,
            compat_install_path: envar("STEAM_COMPAT_INSTALL_PATH")?,
        });
        fn envar(name: &str) -> anyhow::Result<String> {
            env::var(name).with_context(|| format!("reading {name} envar"))
        }
    }
}
