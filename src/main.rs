mod scaler;
mod world_vector;

use crate::world_vector::WorldVector;
use raylib::prelude::*;
use std::f32::consts::PI;

const SCROLL_STEP_DOWN: f32 = 1.1;
const SCROLL_STEP_UP: f32 = 0.9;
const ARROW_SIZE: f32 = 7.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .resizable()
        .vsync()
        .title("vectors")
        .build();

    // where the user started holding the left mouse button
    let mut left_holding_mouse_pos = Vector2::new(0.0, 0.0);
    // where the user started holding the middle mouse button
    let mut last_mid_mouse_click = Vector2::new(0.0, 0.0);
    // is left mouse button down
    let mut left_holding = false;
    // are the vectors being simulated (are they being moved according to velocity)
    let mut simulating = false;

    let mut vectors: Vec<WorldVector> = Vec::new();

    // in charge of conversion to and from world and screen coordinates
    let mut scaler = scaler::Scaler {
        scale: 1.0,
        offset: Vector2::new(0.0, 0.0),
    };

    // main loop
    while !rl.window_should_close() {
        let mouse_pos = Vector2::new(rl.get_mouse_x() as f32, rl.get_mouse_y() as f32);

        // while middle mouse is down, pan around
        if rl.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON) {
            // not using to_world because the offset would mess it up
            scaler.offset -= (mouse_pos - last_mid_mouse_click) * scaler.scale;
        } else if rl.is_key_pressed(KeyboardKey::KEY_R) {
            vectors.pop();
        } else if rl.is_key_pressed(KeyboardKey::KEY_C) {
            vectors.clear();
        } else if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            simulating = !simulating;
        }

        let scroll = rl.get_mouse_wheel_move();

        if scroll != 0.0 {
            let before_scale = scaler.to_world(&mouse_pos);

            let step = if scroll > 0.0 {
                SCROLL_STEP_UP
            } else {
                SCROLL_STEP_DOWN
            };

            let new_scale = scaler.scale * step;
            if new_scale > 0.0 {
                scaler.scale = new_scale;
            }

            let after_scale = scaler.to_world(&mouse_pos);

            scaler.offset += before_scale - after_scale;
        }

        last_mid_mouse_click = mouse_pos;

        let mouse_left_down = rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);

        if simulating {
            // update all vector positions
            for v in vectors.iter_mut() {
                v.update(&rl.get_frame_time());
            }
        }

        let mut d = rl.begin_drawing(&thread);

        if mouse_left_down && !left_holding {
            // just started holding, setting starting point
            left_holding_mouse_pos = mouse_pos;
            left_holding = true;
        } else if mouse_left_down && left_holding {
            // in the process of holding, draw vector from start to mouse
            draw_arrow(&mut d, &left_holding_mouse_pos, &mouse_pos, ARROW_SIZE, Color::RED)
        } else if !mouse_left_down && left_holding {
            // finished dragging, add the vector to vectors
            vectors.push(WorldVector::new(
                scaler.to_world(&left_holding_mouse_pos),
                scaler.to_world(&mouse_pos) - scaler.to_world(&left_holding_mouse_pos),
            ));
            left_holding = false;
        }

        d.clear_background(Color::BLACK);
        for vec in &vectors {
            draw_arrow_v(&mut d, &scaler.to_screen_v(vec), ARROW_SIZE, Color::BLUE);
        }
    }
}

fn draw_arrow_v(d: &mut RaylibDrawHandle, v: &WorldVector, radius: f32, color: Color) {
    let p0 = v.root;
    let p1 = v.root + v.velocity;
    draw_arrow(d, &p0, &p1, radius, color);
}

fn draw_arrow(d: &mut RaylibDrawHandle, p0: &Vector2, p1: &Vector2, radius: f32, color: Color) {
    d.draw_line_v(p0, p1, color);

    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;

    let length = (dx * dx + dy * dy).sqrt();

    let vx = dy / length;
    let vy = dx / length;

    let angle = -vy.atan2(vx) * 180.0 / PI;

    d.draw_poly(p1, 3, radius, angle, color);
}
