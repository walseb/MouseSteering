use crate::config::Config;

pub fn normalize_value(val: f32, min: f32, max: f32) -> f32 {
    2.0 * ((val - min) / (max - min)) - 1.0
}

pub fn apply_snap(value: f32, threshold: f32, center_snap: bool, center_threshold: f32) -> f32 {
    let mut result = value;

    if result.abs() >= 1.0 - threshold {
        result = result.signum();
    } else if center_snap && result.abs() <= center_threshold {
        result = 0.0;
    } else if !center_snap && result.abs() <= 0.02 {
        result = 0.0;
    }

    result.min(1.0).max(-1.0)
}

pub fn get_thumbstick_x(value: f32, config: &Config) -> (i16, f32) {
    let smoothed_value = if config.steering_smooth {
        apply_snap(
            value,
            config.steering_side_threshold,
            config.steering_center_snap,
            config.steering_center_threshold,
        )
    } else {
        if value <= 0.0 {
            if config.steering_center_snap {
                if value >= -config.steering_center_threshold {
                    0.0
                } else {
                    -1.0
                }
            } else {
                if value >= -0.02 {
                    0.0
                } else {
                    -1.0
                }
            }
        } else {
            if config.steering_center_snap {
                if value <= config.steering_center_threshold {
                    0.0
                } else {
                    1.0
                }
            } else {
                if value <= 0.02 {
                    0.0
                } else {
                    1.0
                }
            }
        }
    };

    ((smoothed_value * i16::MAX as f32) as i16, smoothed_value)
}

pub fn get_trigger(value: f32, config: &Config) -> (u8, u8, f32, f32) {
    let smoothed_value = if config.throttle_smooth {
        apply_snap(
            value,
            config.throttle_side_threshold,
            config.throttle_center_snap,
            config.steering_center_threshold,
        )
    } else {
        if value <= 0.0 {
            if config.throttle_center_snap {
                if value.abs() <= config.throttle_center_threshold {
                    0.0
                } else {
                    -1.0
                }
            } else {
                if value.abs() <= 0.02 {
                    0.0
                } else {
                    -1.0
                }
            }
        } else {
            if config.throttle_center_snap {
                if value.abs() <= config.throttle_center_threshold {
                    0.0
                } else {
                    1.0
                }
            } else {
                if value.abs() <= 0.02 {
                    0.0
                } else {
                    1.0
                }
            }
        }
    };

    if value <= 0.0 {
        (0, (smoothed_value.abs() * 255.0) as u8, 0.0, smoothed_value.abs())
    } else {
        ((smoothed_value.abs() * 255.0) as u8, 0, smoothed_value.abs(), 0.0)
    }
}
