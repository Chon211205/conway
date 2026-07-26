use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

const ALIVE_COLOR: Color = Color::GREEN;
const DEAD_COLOR: Color = Color::BLACK;

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

            let neighbor_x = (x as i32 + offset_x)
                .rem_euclid(framebuffer.width as i32);

            let neighbor_y = (y as i32 + offset_y)
                .rem_euclid(framebuffer.height as i32);

            let color = framebuffer.get_color(
                neighbor_x as u32,
                neighbor_y as u32,
            );

            if color == ALIVE_COLOR {
                alive_neighbors += 1;
            }
        }
    }

    alive_neighbors
}

pub fn add_block(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
) {
    let pattern = [
        (0, 0), (1, 0),
        (0, 1), (1, 1),
    ];

    place_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn add_beehive(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
) {
    let pattern = [
                (1, 0), (2, 0),
        (0, 1),                 (3, 1),
                (1, 2), (2, 2),
    ];

    place_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn add_loaf(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
) {
    let pattern = [
                (1, 0), (2, 0),
        (0, 1),                 (3, 1),
                (1, 2),         (3, 2),
                        (2, 3),
    ];

    place_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn add_boat(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
) {
    let pattern = [
        (0, 0), (1, 0),
        (0, 1),         (2, 1),
                (1, 2),
    ];

    place_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn add_tub(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
) {
    let pattern = [
                (1, 0),
        (0, 1),         (2, 1),
                (1, 2),
    ];

    place_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn add_blinker(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
) {
    let pattern = [
        (0, 0),
        (0, 1),
        (0, 2),
    ];

    place_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn add_toad(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
) {
    let pattern = [
                (1, 0), (2, 0), (3, 0),
        (0, 1), (1, 1), (2, 1),
    ];

    place_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn add_beacon(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
) {
    let pattern = [
        (0, 0), (1, 0),
        (0, 1), (1, 1),

                        (2, 2), (3, 2),
                        (2, 3), (3, 3),
    ];

    place_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn add_glider(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
) {
    let pattern = [
                (1, 0),
                        (2, 1),
        (0, 2), (1, 2), (2, 2),
    ];

    place_pattern(framebuffer, start_x, start_y, &pattern);
}

pub fn add_lwss(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
) {
    let pattern = [
        (1, 0),                         (4, 0),
        (0, 1),
        (0, 2),                         (4, 2),
        (0, 3), (1, 3), (2, 3), (3, 3),
    ];

    place_pattern(framebuffer, start_x, start_y, &pattern);
}

fn place_pattern(
    framebuffer: &mut Framebuffer,
    start_x: u32,
    start_y: u32,
    pattern: &[(u32, u32)],
) {
    framebuffer.set_current_color(ALIVE_COLOR);

    for &(offset_x, offset_y) in pattern {
        let x = start_x + offset_x;
        let y = start_y + offset_y;

        if x < framebuffer.width && y < framebuffer.height {
            framebuffer.point(x, y);
        }
    }
}

pub fn load_initial_patterns(framebuffer: &mut Framebuffer) {
    framebuffer.clear();

    add_block(framebuffer, 5, 6);
    add_beehive(framebuffer, 15, 5);
    add_boat(framebuffer, 27, 7);
    add_tub(framebuffer, 38, 5);
    add_loaf(framebuffer, 49, 6);
    add_blinker(framebuffer, 62, 5);
    add_toad(framebuffer, 72, 7);
    add_glider(framebuffer, 88, 5);

    add_glider(framebuffer, 8, 18);
    add_beacon(framebuffer, 20, 18);
    add_block(framebuffer, 33, 20);
    add_beehive(framebuffer, 44, 18);
    add_blinker(framebuffer, 57, 20);
    add_boat(framebuffer, 67, 18);
    add_tub(framebuffer, 78, 20);
    add_glider(framebuffer, 89, 18);

    add_toad(framebuffer, 5, 32);
    add_boat(framebuffer, 17, 31);
    add_glider(framebuffer, 28, 33);
    add_loaf(framebuffer, 39, 31);
    add_beacon(framebuffer, 51, 32);
    add_block(framebuffer, 64, 33);
    add_beehive(framebuffer, 75, 31);
    add_blinker(framebuffer, 90, 32);

    add_tub(framebuffer, 8, 45);
    add_blinker(framebuffer, 18, 44);
    add_beehive(framebuffer, 29, 46);
    add_glider(framebuffer, 42, 44);
    add_toad(framebuffer, 53, 45);
    add_boat(framebuffer, 66, 44);
    add_beacon(framebuffer, 77, 45);
    add_glider(framebuffer, 91, 46);

    add_block(framebuffer, 5, 58);
    add_loaf(framebuffer, 16, 57);
    add_glider(framebuffer, 29, 59);
    add_tub(framebuffer, 40, 58);
    add_blinker(framebuffer, 51, 57);
    add_beehive(framebuffer, 62, 59);
    add_boat(framebuffer, 75, 57);
    add_lwss(framebuffer, 86, 58);

    add_glider(framebuffer, 8, 72);
    add_beacon(framebuffer, 20, 70);
    add_toad(framebuffer, 34, 72);
    add_block(framebuffer, 47, 70);
    add_boat(framebuffer, 58, 73);
    add_loaf(framebuffer, 69, 70);
    add_blinker(framebuffer, 82, 72);
    add_glider(framebuffer, 92, 70);

    add_tub(framebuffer, 5, 86);
    add_beehive(framebuffer, 16, 84);
    add_glider(framebuffer, 29, 86);
    add_block(framebuffer, 41, 85);
    add_beacon(framebuffer, 52, 84);
    add_boat(framebuffer, 65, 86);
    add_toad(framebuffer, 76, 84);
    add_lwss(framebuffer, 88, 84);
}