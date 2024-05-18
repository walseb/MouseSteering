mod config;
mod emulator;
mod utils;
mod visualizer;

use crate::emulator::Emulator;
use crate::utils::{get_thumbstick_x, get_trigger, normalize_value};
use crate::visualizer::Visualizer;
use config::Config;
use mouse_position::mouse_position::Mouse;
use std::{thread, time};

fn main() {
    let mut emulator: Emulator = Emulator::new();
    let mut visualizer: Visualizer = Visualizer::new(240, 240);
    let config: Config = Config::new();

    loop {
        let mouse_pos = Mouse::get_mouse_position();

        match mouse_pos {
            Mouse::Position { x, y } => {
                if config.steering_enabled {
                    let normalized_value =
                        normalize_value(x as f32, 0.0, config.screen_width as f32);

                    let (thumbstick_x, value) = get_thumbstick_x(normalized_value, &config);

                    visualizer.update_steer(value);
                    emulator.gamepad.thumb_lx = thumbstick_x;
                }

                if config.throttle_enabled {
                    let normalized_value =
                        normalize_value(y as f32, 0.0, config.screen_height as f32);

                    let (left_trigger, right_trigger, left_value, right_value) = get_trigger(normalized_value, &config);

                    visualizer.update_triggers(left_value, right_value);
                    emulator.gamepad.left_trigger = left_trigger;
                    emulator.gamepad.right_trigger = right_trigger;
                }

                emulator.update();
            }
            Mouse::Error => {
                println!("Failed to get mouse position!");
            }
        }

        visualizer.update();

        if !visualizer.is_open() {
            break;
        }

        thread::sleep(time::Duration::from_millis(1));
    }
}
