mod framebuffer;
mod game_of_life;

use framebuffer::Framebuffer;
use game_of_life::{add_glider, render};

use raylib::ffi;
use raylib::prelude::*;

const FRAMEBUFFER_WIDTH: u32 = 100;
const FRAMEBUFFER_HEIGHT: u32 = 100;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;

fn framebuffer_to_bytes(framebuffer: &Framebuffer) -> Vec<u8> {
    let mut bytes =
        Vec::with_capacity(framebuffer.color_buffer.len() * 4);

    for color in &framebuffer.color_buffer {
        bytes.push(color.r);
        bytes.push(color.g);
        bytes.push(color.b);
        bytes.push(color.a);
    }

    bytes
}

fn create_image(width: i32, height: i32, color: Color) -> Image {
    let raw_image = unsafe {
        ffi::GenImageColor(
            width,
            height,
            color.into(),
        )
    };

    unsafe { Image::from_raw(raw_image) }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life")
        .resizable()
        .build();

    rl.set_target_fps(10);

    let mut framebuffer =
        Framebuffer::new(FRAMEBUFFER_WIDTH, FRAMEBUFFER_HEIGHT);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.clear();

    add_glider(&mut framebuffer, 10, 10);
    add_glider(&mut framebuffer, 30, 20);
    add_glider(&mut framebuffer, 60, 50);

    let image = create_image(
        FRAMEBUFFER_WIDTH as i32,
        FRAMEBUFFER_HEIGHT as i32,
        Color::BLACK,
    );

    let mut texture = rl
        .load_texture_from_image(&thread, &image)
        .expect("No se pudo crear la textura");

    while !rl.window_should_close() {
        render(&mut framebuffer);

        let pixel_data = framebuffer_to_bytes(&framebuffer);

        texture
            .update_texture(&pixel_data)
            .expect("No se pudo actualizar la textura");

        let screen_width = rl.get_screen_width();
        let screen_height = rl.get_screen_height();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        d.draw_texture_pro(
            &texture,
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: FRAMEBUFFER_WIDTH as f32,
                height: FRAMEBUFFER_HEIGHT as f32,
            },
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: screen_width as f32,
                height: screen_height as f32,
            },
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );
    }
}