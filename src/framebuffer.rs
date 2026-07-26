use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Vec<Color>,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            color_buffer: vec![Color::BLACK; (width * height) as usize],
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn point(&mut self, x: u32, y: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = (y * self.width + x) as usize;
        self.color_buffer[index] = self.current_color;
    }

    pub fn get_color(&self, x: u32, y: u32) -> Color {
        if x >= self.width || y >= self.height {
            return self.background_color;
        }

        let index = (y * self.width + x) as usize;
        self.color_buffer[index]
    }

    pub fn clear(&mut self) {
        self.color_buffer.fill(self.background_color);
    }
}