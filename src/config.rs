use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Error as IoError;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    screen_width: Option<i32>,
    screen_height: Option<i32>,

    steering_enabled: Option<bool>,
    steering_smooth: Option<bool>,
    steering_side_snap: Option<bool>,
    steering_side_threshold: Option<f32>,
    steering_center_snap: Option<bool>,
    steering_center_threshold: Option<f32>,

    throttle_enabled: Option<bool>,
    throttle_smooth: Option<bool>,
    throttle_side_snap: Option<bool>,
    throttle_side_threshold: Option<f32>,
    throttle_center_snap: Option<bool>,
    throttle_center_threshold: Option<f32>,
}

#[derive(Debug)]
pub struct Config {
    pub screen_width: i32,
    pub screen_height: i32,

    pub steering_enabled: bool,
    pub steering_smooth: bool,
    pub steering_side_snap: bool,
    pub steering_side_threshold: f32,
    pub steering_center_snap: bool,
    pub steering_center_threshold: f32,

    pub throttle_enabled: bool,
    pub throttle_smooth: bool,
    pub throttle_side_snap: bool,
    pub throttle_side_threshold: f32,
    pub throttle_center_snap: bool,
    pub throttle_center_threshold: f32,
}

impl ConfigToml {
    fn get_defaults() -> ConfigToml {
        ConfigToml {
            screen_width: Some(1280),
            screen_height: Some(720),

            steering_enabled: Some(true),
            steering_smooth: Some(true),
            steering_side_snap: Some(true),
            steering_side_threshold: Some(0.1),
            steering_center_snap: Some(true),
            steering_center_threshold: Some(0.1),

            throttle_enabled: Some(true),
            throttle_smooth: Some(true),
            throttle_side_snap: Some(true),
            throttle_side_threshold: Some(0.1),
            throttle_center_snap: Some(true),
            throttle_center_threshold: Some(0.1),
        }
    }

    fn merge(config: ConfigToml) -> ConfigToml {
        let default = ConfigToml::get_defaults();

        ConfigToml {
            screen_width: config.screen_width.or(default.screen_width),
            screen_height: config.screen_height.or(default.screen_height),

            steering_enabled: config.steering_enabled.or(default.steering_enabled),
            steering_smooth: config.steering_smooth.or(default.steering_smooth),
            steering_side_snap: config.steering_side_snap.or(default.steering_side_snap),
            steering_side_threshold: config
                .steering_side_threshold
                .or(default.steering_side_threshold),
            steering_center_snap: config.steering_center_snap.or(default.steering_center_snap),
            steering_center_threshold: config
                .steering_center_threshold
                .or(default.steering_center_threshold),

            throttle_enabled: config.throttle_enabled.or(default.throttle_enabled),
            throttle_smooth: config.throttle_smooth.or(default.throttle_smooth),
            throttle_side_snap: config.throttle_side_snap.or(default.throttle_side_snap),
            throttle_side_threshold: config
                .throttle_side_threshold
                .or(default.throttle_side_threshold),
            throttle_center_snap: config.throttle_center_snap.or(default.throttle_center_snap),
            throttle_center_threshold: config
                .throttle_center_threshold
                .or(default.throttle_center_threshold),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let filepaths: [&str; 2] = ["./config.toml", "./Config.toml"];

        let mut content: String = String::new();
        for filepath in filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let user_config: ConfigToml =
            toml::from_str(&content).unwrap_or_else(|_| ConfigToml::get_defaults());
        let config_toml: ConfigToml = ConfigToml::merge(user_config);

        Config {
            screen_width: config_toml.screen_width.unwrap(),
            screen_height: config_toml.screen_height.unwrap(),

            steering_enabled: config_toml.steering_enabled.unwrap(),
            steering_smooth: config_toml.steering_smooth.unwrap(),
            steering_side_snap: config_toml.steering_side_snap.unwrap(),
            steering_side_threshold: config_toml.steering_side_threshold.unwrap(),
            steering_center_snap: config_toml.steering_center_snap.unwrap(),
            steering_center_threshold: config_toml.steering_center_threshold.unwrap(),

            throttle_enabled: config_toml.throttle_enabled.unwrap(),
            throttle_smooth: config_toml.throttle_smooth.unwrap(),
            throttle_side_snap: config_toml.throttle_side_snap.unwrap(),
            throttle_side_threshold: config_toml.throttle_side_threshold.unwrap(),
            throttle_center_snap: config_toml.throttle_center_snap.unwrap(),
            throttle_center_threshold: config_toml.throttle_center_threshold.unwrap(),
        }
    }
}
