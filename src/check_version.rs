use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug, Serialize)]
pub struct AppInfos {
    pub name: String,
    pub version: String,
}

pub async fn check_version() -> Option<AppInfos> {
    let config_text = fs::read_to_string("//192.168.0.250/soft/data2excel/version.toml");
    match config_text {
        Ok(res) => {
            let c: AppInfos = { toml::from_str(&res).expect("LogRocket: error reading stream") };
            Some(c)
        }
        Err(_) => {
            let config_text = fs::read_to_string("//192.168.0.250/soft/data2excel/version.toml");
            match config_text {
                Ok(res) => {
                    let c: AppInfos =
                        { toml::from_str(&res).expect("LogRocket: error reading stream") };
                    Some(c)
                }
                Err(_) => None,
            }
        }
    }
}
