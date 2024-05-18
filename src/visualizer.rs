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

fn map_steer_value(value: f32, min: f32, max: f32) -> f32 {
    let normalized_value = (value + 1.0) / 2.0;
    min + normalized_value * (max - min)
}

fn map_trig_value(value: f32, min: f32, max: f32) -> f32 {
    min + value * (max - min)
}

impl Visualizer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer: Vec<u32> = vec![0; width * height];
        let mut window = Window::new("Visualizer", width, height, WindowOptions::default())
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
        let container_width = 120;
        let container_height = 16;

        let start_x = (self.width / 2) - (container_width / 2);
        let start_y = (self.height / 2) - (container_height / 2) + (container_height * 2);

        let container_color = from_rgb(55, 55, 55);
        self.draw_rect(
            start_x,
            start_y,
            container_width,
            container_height,
            container_color,
        );

        let middle_color = from_rgb(85, 85, 85);
        let middle_value = map_steer_value(0.0, start_x as f32, (start_x + container_width - 5) as f32);
        self.draw_rect(
            middle_value as usize - 2,
            start_y,
            9,
            container_height,
            middle_color,
        );

        let mapped_value = map_steer_value(
            value,
            start_x as f32,
            (start_x + container_width - 5) as f32,
        );

        let line_color = from_rgb(255, 155, 0);
        self.draw_rect(
            mapped_value as usize,
            start_y,
            5,
            container_height,
            line_color,
        );
    }

    pub fn update_triggers(&mut self, left_trigger: f32, right_trigger: f32) {
        self.update_trigger(left_trigger, false);
        self.update_trigger(right_trigger, true);
    }

    pub fn update_trigger(&mut self, value: f32, is_throttle: bool) {
        let container_width = 120;
        let container_height = 16;

        let start_x = (self.width / 2) - (container_width / 2);
        let start_y = if is_throttle {
            (self.height / 2) - (container_height / 2) - (container_height * 2)
        } else {
            (self.height / 2) - (container_height / 2)
        };

        // Draw Container
        let container_color = from_rgb(55, 55, 55);
        self.draw_rect(
            start_x,
            start_y,
            container_width,
            container_height,
            container_color,
        );

        let mapped_value = map_trig_value(
            value,
            0.0,
            container_width as f32,
        );

        // Draw Values
        let value_color = if is_throttle {
            from_rgb(0, 255, 0)
        } else {
            from_rgb(255, 0, 0)
        };
        self.draw_rect(start_x, start_y, mapped_value as usize, container_height, value_color);
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

    fn draw(&mut self, x: usize, y: usize, color: u32) {
        let index = y * self.width + x;
        self.buffer[index] = color;
    }
}
