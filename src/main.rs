#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{hotreload::FreyaCtx, launch::launch_cfg, prelude::{dioxus_hot_reload, Config, LaunchConfig}};
use crate::app_view::app;
pub mod app_view;
pub mod bosa;
pub mod check_pn_type;
pub mod data2excel;
pub mod utils;
pub mod check_version;
pub mod updata;
pub mod error;
const ICON: &[u8] = include_bytes!("../resources/windows/xt.ico");
fn main() {
    dioxus_hot_reload::hot_reload_init!(Config::<FreyaCtx>::default());
    launch_cfg(
        app,
        LaunchConfig::<()>::builder()
            .with_width(1440.0)
            .with_height(900.0)
            .with_decorations(true)
            .with_transparency(false)
            .with_title("数据导出")
            .with_background("rgb(150, 100, 200")
            .with_icon(LaunchConfig::load_icon(ICON))
            .build(),
    );
    // launch(app);
}


