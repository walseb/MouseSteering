use crate::config::ControlConfig;
pub struct Processor;
impl Processor {
    pub fn normalize(value: i32, min: i32, max: i32) -> f32 {
        2.0 * ((value as f32 - min as f32) / (max as f32 - min as f32)) - 1.0
    }

    pub fn process(value: f32, config: &ControlConfig) -> f32 {
        if config.precise_input {
            if config.edge_scaling {
                if config.snap_input {
                    Self::apply_snap(value / config.scaling_threshold, config.snap_threshold, config.snap_threshold)
                } else {
                    value / config.scaling_threshold
                }
            } else {
                if config.snap_input {
                    Self::apply_snap(value, config.snap_threshold, config.snap_threshold)
                } else {
                    value
                }
            }
        } else {
            if config.snap_input {
                Self::apply_snap(value, 1.0, config.snap_threshold)
            } else {
                Self::apply_snap(value, 1.0, 0.0)
            }
        }
    }

    pub fn to_thumb_val(value: f32) -> i16 {
        (value * i16::MAX as f32) as i16
    }

    pub fn to_trigger_val(value: f32) -> u8 {
        (value * 255.0) as u8
    }

    fn apply_snap(value: f32, threshold: f32, center_threshold: f32) -> f32 {
        let mut result = value;

        if value.abs() >= 1.0 - threshold {
            result = result.signum();
        }
        if value.abs() <= center_threshold {
            result = 0.0
        }

        result.min(1.0).max(-1.0)
    }
}
