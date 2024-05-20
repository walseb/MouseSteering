#![windows_subsystem = "windows"]

mod config;
mod emulator;
mod gui;
mod processor;

use crate::gui::Visualizer;
use config::Config;
use emulator::Emulator;
use mouse_position::mouse_position::Mouse;
use processor::Processor;
use std::{thread, time::Duration};

fn main() {
    let mut emulator = Emulator::new();
    let mut visualizer = Visualizer::new();

    let config = Config::new();
    let steering_config = &config.steering_config;
    let throttle_config = &config.throttle_config;

    loop {
        let mouse_position = Mouse::get_mouse_position();

        match mouse_position {
            Mouse::Position { x, y } => {
                if steering_config.enabled {
                    let normal_value = Processor::normalize(x, 0, config.screen_width);

                    let horizontal_value = Processor::process(normal_value, &steering_config);

                    emulator.gamepad.thumb_lx = Processor::to_thumb_val(horizontal_value);

                    let (lval, rval) = if horizontal_value <= 0.0 {
                        (horizontal_value.abs(), 0.0)
                    } else {
                        (0.0, horizontal_value.abs())
                    };

                    visualizer.update(None, Some(lval), None, Some(rval));
                }

                if throttle_config.enabled {
                    let normal_value = Processor::normalize(y, 0, config.screen_height);

                    let vertical_value = Processor::process(normal_value, &throttle_config);

                    let (left_trigger, right_trigger) = if vertical_value <= 0.0 {
                        (0.0, vertical_value.abs())
                    } else {
                        (vertical_value.abs(), 0.0)
                    };

                    emulator.gamepad.left_trigger = Processor::to_trigger_val(left_trigger);
                    emulator.gamepad.right_trigger = Processor::to_trigger_val(right_trigger);

                    visualizer.update(Some(right_trigger), None, Some(left_trigger), None);
                }
            }
            Mouse::Error => {}
        }

        emulator.emulate();
        visualizer.draw();

        if visualizer.should_close() {
            break;
        }

        thread::sleep(Duration::from_millis(1));
    }
}
