mod scaler;

use raylib::prelude::*;
use std::f32::consts::PI;

const SCROLL_STEP_DOWN: f32 = 1.1;
const SCROLL_STEP_UP: f32 = 0.9;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .resizable()
        .title("vectors")
        .build();

    let mut left_holding_mouse_pos = Vector2::new(0.0, 0.0);
    let mut left_holding = false;

    let mut last_mid_mouse_click = Vector2::new(0.0, 0.0);

    let mut vectors: Vec<(Vector2, Vector2)> = Vec::new();

    let mut scaler = scaler::Scaler {
        scale: 1.0,
        offset: Vector2::new(0.0, 0.0),
    };

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mouse_pos = Vector2::new(rl.get_mouse_x() as f32, rl.get_mouse_y() as f32);

        if rl.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON) {
            scaler.offset -= (mouse_pos - last_mid_mouse_click) * scaler.scale;
        } else if rl.is_key_pressed(KeyboardKey::KEY_R) {
            vectors.pop();
        }

        let scroll = rl.get_mouse_wheel_move();

        if scroll != 0.0 {
            let before_scale = scaler.to_world(mouse_pos);

            let step = if scroll > 0.0 {
                SCROLL_STEP_UP
            } else {
                SCROLL_STEP_DOWN
            };

            let new_scale = scaler.scale * step;
            if new_scale > 0.0 {
                scaler.scale = new_scale;
            }

            let after_scale = scaler.to_world(mouse_pos);

            scaler.offset += before_scale - after_scale;
        }

        last_mid_mouse_click = mouse_pos;

        let mouse_left_down = rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);

        let mut d = rl.begin_drawing(&thread);

        if mouse_left_down && !left_holding {
            // just started holding, setting starting point
            left_holding_mouse_pos = mouse_pos;
            left_holding = true;
        } else if mouse_left_down && left_holding {
            // in the process of holding, draw vector from start to mouse
            draw_arrow(&mut d, &left_holding_mouse_pos, &mouse_pos, 7.3, Color::RED)
        } else if !mouse_left_down && left_holding {
            // finished dragging, add the vector to vectors
            vectors.push((
                scaler.to_world(left_holding_mouse_pos),
                scaler.to_world(mouse_pos),
            ));
            left_holding = false;
        }

        d.clear_background(Color::BLACK);
        for vec in &vectors {
            draw_arrow(
                &mut d,
                &scaler.to_screen(vec.0),
                &scaler.to_screen(vec.1),
                7.3,
                Color::BLUE,
            );
        }
    }
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
