use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

const ALIVE_COLOR: Color = Color::GREEN;
const DEAD_COLOR: Color = Color::RED;

pub fn render(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width;
    let height = framebuffer.height;


    let mut next_generation =
        vec![DEAD_COLOR; (width * height) as usize];

    for y in 0..height {
        for x in 0..width {
            let current_color = framebuffer.get_color(x, y);
            let is_alive = current_color == ALIVE_COLOR;

            let alive_neighbors =
                count_alive_neighbors(framebuffer, x, y);

            let next_color = match (is_alive, alive_neighbors) {
                (true, 2) | (true, 3) => ALIVE_COLOR,

                (false, 3) => ALIVE_COLOR,

                _ => DEAD_COLOR,
            };

            let index = (y * width + x) as usize;
            next_generation[index] = next_color;
        }
    }


    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize;
            let color = next_generation[index];

            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

pub fn add_glider(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
) {
    let pattern = [
        (1, 0),
        (2, 1),
        (0, 2),
        (1, 2),
        (2, 2),
    ];

    framebuffer.set_current_color(ALIVE_COLOR);

    for (offset_x, offset_y) in pattern {
        framebuffer.point(
            start_x + offset_x,
            start_y + offset_y,
        );
    }
}

fn count_alive_neighbors(
    framebuffer: &Framebuffer,
    x: u32,
    y: u32,
) -> u8 {
    let mut alive_neighbors = 0;

    for offset_y in -1..=1 {
        for offset_x in -1..=1 {
            if offset_x == 0 && offset_y == 0 {
                continue;
            }

            let neighbor_x = x as i32 + offset_x;
            let neighbor_y = y as i32 + offset_y;

            if neighbor_x < 0
                || neighbor_y < 0
                || neighbor_x >= framebuffer.width as i32
                || neighbor_y >= framebuffer.height as i32
            {
                continue;
            }

            let neighbor_color = framebuffer.get_color(
                neighbor_x as u32,
                neighbor_y as u32,
            );

            if neighbor_color == ALIVE_COLOR {
                alive_neighbors += 1;
            }
        }
    }

    alive_neighbors
}