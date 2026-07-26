mod framebuffer;
mod game_of_life;

use framebuffer::Framebuffer;
use game_of_life::{load_initial_patterns, render};

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

fn create_image(
    width: i32,
    height: i32,
    color: Color,
) -> Image {
    let raw_image = unsafe {
        ffi::GenImageColor(
            width,
            height,
            color.into(),
        )
    };

    unsafe {
        Image::from_raw(raw_image)
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Conway's Game of Life")
        .resizable()
        .build();

    rl.set_target_fps(10);

    let mut framebuffer =
        Framebuffer::new(
            FRAMEBUFFER_WIDTH,
            FRAMEBUFFER_HEIGHT,
        );

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.set_current_color(Color::GREEN);
    framebuffer.clear();

    load_initial_patterns(&mut framebuffer);

    let image = create_image(
        FRAMEBUFFER_WIDTH as i32,
        FRAMEBUFFER_HEIGHT as i32,
        Color::BLACK,
    );

    let mut texture = rl
        .load_texture_from_image(&thread, &image)
        .expect("No se pudo crear la textura");

    let mut paused = false;

    while !rl.window_should_close() {

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_R) {
            load_initial_patterns(&mut framebuffer);
        }

        let next_generation =
            rl.is_key_pressed(KeyboardKey::KEY_N);

        if !paused || next_generation {
            render(&mut framebuffer);
        }

        let pixel_data =
            framebuffer_to_bytes(&framebuffer);

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

        let status = if paused {
            "PAUSADO"
        } else {
            "EJECUTANDO"
        };

        d.draw_rectangle(
            0,
            0,
            screen_width,
            55,
            Color::new(0, 0, 0, 180),
        );

        d.draw_text(
            status,
            10,
            8,
            20,
            Color::WHITE,
        );

        d.draw_text(
            "ESPACIO: pausa | N: siguiente | R: reiniciar",
            10,
            32,
            16,
            Color::WHITE,
        );
    }
}