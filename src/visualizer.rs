use minifb::{Window, WindowOptions};

pub struct Visualizer {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

pub fn from_rgb(red: u32, green: u32, blue: u32) -> u32 {
    (red << 16) | (green << 8) | blue
}

fn map_value(value: f32, min: f32, max: f32) -> f32 {
    min + value * (max - min)
}

fn get_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn get_arrow_pattern(orientation: usize) -> Vec<(i16, i16)> {
    let mut pattern: Vec<(i16, i16)> = Vec::new();

    if orientation == 0 {
        // Left
        for dy in 0..8 {
            for dx in 0..3 {
                pattern.push((dx + dy - 6, -dy));
            }
        }
        for dy in 1..8 {
            for dx in 0..3 {
                pattern.push((dx + dy - 6, dy));
            }
        }
    } else if orientation == 1 {
        // Top
        for dx in 0..8 {
            for dy in 0..3 {
                pattern.push((dx, dx - dy - 3));
            }
        }
        for dx in 1..8 {
            for dy in 0..3 {
                pattern.push((-dx, dx - dy - 3));
            }
        }
    } else if orientation == 2 {
        // Right
        for dy in 0..8 {
            for dx in 0..3 {
                pattern.push((dx - dy + 3, -dy));
            }
        }
        for dy in 1..8 {
            for dx in 0..3 {
                pattern.push((dx - dy + 3, dy));
            }
        }
    } else if orientation == 4 {
        // Bottom
        for dx in 0..8 {
            for dy in 0..3 {
                pattern.push((dx, dy - dx + 3));
            }
        }
        for dx in 1..8 {
            for dy in 0..3 {
                pattern.push((-dx, dy - dx + 3));
            }
        }
    }

    pattern
}

impl Visualizer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer: Vec<u32> = vec![0; width * height];
        let mut window = Window::new(
            "Mouse-Steering by RasyaJusticio",
            width,
            height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.topmost(true);
        window.set_target_fps(60);

        Visualizer {
            window,
            buffer,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
        self.clear_buffer();
    }

    pub fn update_steer(&mut self, value: f32) {
        let container_size = 64;

        let left_x = 12;
        let right_x = (12 * 3) + (container_size * 2);
        let start_y = (12 * 2) + container_size;

        let container_color = from_rgb(55, 55, 55);
        let border_color = from_rgb(95, 95, 95);

        // Left container
        self.draw_rect(
            left_x,
            start_y,
            container_size,
            container_size,
            container_color,
        );
        // Right container
        self.draw_rect(
            right_x,
            start_y,
            container_size,
            container_size,
            container_color,
        );

        let fill_color = from_rgb(55, 155, 255);
        if value <= 0.0 {
            // Left container
            let mapped_value = map_value(value.abs(), container_size as f32, 0.0);

            self.draw_rect(
                left_x + mapped_value as usize,
                start_y,
                container_size - mapped_value as usize,
                container_size,
                fill_color,
            );
        } else {
            // Right container
            let mapped_value = map_value(value, 0.0, container_size as f32);

            self.draw_rect(
                right_x,
                start_y,
                mapped_value as usize,
                container_size,
                fill_color,
            );
        }

        // Left border
        self.draw_border(
            left_x,
            start_y,
            container_size,
            container_size,
            border_color,
        );
        // Right border
        self.draw_border(
            right_x,
            start_y,
            container_size,
            container_size,
            border_color,
        );

        // Left arrow
        self.draw_pattern(
            left_x + (container_size / 2),
            start_y + (container_size / 2),
            get_arrow_pattern(0),
        );
        // Right arrow
        self.draw_pattern(
            right_x + (container_size / 2),
            start_y + (container_size / 2),
            get_arrow_pattern(2),
        );
    }

    pub fn update_triggers(&mut self, left_trigger: f32, right_trigger: f32) {
        let container_size = 64;

        let start_x = (12 * 2) + container_size;
        let top_y = 12;
        let bottom_y = (12 * 2) + container_size;

        let container_color = from_rgb(55, 55, 55);
        let border_color = from_rgb(95, 95, 95);

        // Top container
        self.draw_rect(
            start_x,
            top_y,
            container_size,
            container_size,
            container_color,
        );
        // Bottom container
        self.draw_rect(
            start_x,
            bottom_y,
            container_size,
            container_size,
            container_color,
        );

        let mapped_top = map_value(right_trigger, container_size as f32, 0.0);
        let mapped_bottom = map_value(left_trigger, 0.0, container_size as f32);

        let top_color = from_rgb(0, 215, 0);
        let bottom_color = from_rgb(255, 0, 0);

        // Top fill
        self.draw_rect(
            start_x,
            top_y + mapped_top as usize,
            container_size,
            container_size - mapped_top as usize,
            top_color,
        );
        // Bottom fill
        self.draw_rect(
            start_x,
            bottom_y,
            container_size,
            mapped_bottom as usize,
            bottom_color,
        );

        // Top border
        self.draw_border(start_x, top_y, container_size, container_size, border_color);
        // Bottom border
        self.draw_border(
            start_x,
            bottom_y,
            container_size,
            container_size,
            border_color,
        );

        // Top arrow
        self.draw_pattern(
            start_x + (container_size / 2),
            top_y + (container_size / 2),
            get_arrow_pattern(1),
        );
        // Bottom arrow
        self.draw_pattern(
            start_x + (container_size / 2),
            bottom_y + (container_size / 2),
            get_arrow_pattern(4),
        );
    }

    pub fn is_open(&mut self) -> bool {
        self.window.is_open()
    }

    fn clear_buffer(&mut self) {
        self.buffer.iter_mut().for_each(|p| *p = 0);
    }

    fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for k in 0..height {
            for j in 0..width {
                self.draw(x + j, y + k, color);
            }
        }
    }

    fn draw_border(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for k in 0..height {
            for j in 0..width {
                if j == 0 || k == 0 || j == width - 1 || k == height - 1 {
                    self.draw(x + j, y + k, color);
                }
            }
        }
    }

    fn draw_pattern(&mut self, x: usize, y: usize, pattern: Vec<(i16, i16)>) {
        let color = from_rgb(255, 2555, 255);
        for &(dx, dy) in &pattern {
            self.draw(
                x.wrapping_add(dx as usize),
                y.wrapping_add(dy as usize),
                color,
            );
        }
    }

    fn draw(&mut self, x: usize, y: usize, color: u32) {
        let index = get_index(x, y, self.width);
        self.buffer[index] = color;
    }
}
