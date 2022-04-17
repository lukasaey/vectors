mod scaler;

use raylib::prelude::*;
use std::f32::consts::PI;

const WIDTH: i32 = 1280;
const HEIGHT: i32 = 720;

const SCROLL_STEP_DOWN: f32 = 1.1;
const SCROLL_STEP_UP: f32 = 0.9;

fn main() {
    let (mut rl, thread) = raylib::init().size(WIDTH, HEIGHT).title("vectors").build();

    let mut left_holding_x: i32 = 0;
    let mut left_holding_y: i32 = 0;
    let mut left_holding = false;

    let mut last_right_x: i32 = 0;
    let mut last_right_y: i32 = 0;

    let mut vectors: Vec<(Vector2, Vector2)> = Vec::new();

    let mut scaler = scaler::Scaler {
        scale: 1.0,
        x_offset: 0.0,
        y_offset: 0.0,
    };

    while !rl.window_should_close() {
        let mouse_x = rl.get_mouse_x();
        let mouse_y = rl.get_mouse_y();

        if rl.is_mouse_button_down(MouseButton::MOUSE_MIDDLE_BUTTON) {
            scaler.x_offset -= (mouse_x - last_right_x) as f32 * scaler.scale;
            scaler.y_offset -= (mouse_y - last_right_y) as f32 * scaler.scale;
        } else if rl.is_key_pressed(KeyboardKey::KEY_R) {
            vectors.pop();
        }

        let scroll = rl.get_mouse_wheel_move();

        if scroll != 0.0 {
            let (x_before_scale, y_before_scale) = scaler.to_world(mouse_x, mouse_y);

            if scroll > 0.0 {
                let new_scale = scaler.scale * SCROLL_STEP_UP;
                if new_scale > 0.0 {
                    scaler.scale = new_scale;
                }
            } else {
                let new_scale = scaler.scale * SCROLL_STEP_DOWN;
                if new_scale > 0.0 {
                    scaler.scale = new_scale;
                }
            }

            let (x_after_scale, y_after_scale) = scaler.to_world(mouse_x, mouse_y);

            scaler.x_offset += x_before_scale - x_after_scale;
            scaler.y_offset += y_before_scale - y_after_scale;
        }
        last_right_x = mouse_x;
        last_right_y = mouse_y;

        let mouse_left_down = rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);

        let mut d = rl.begin_drawing(&thread);

        if mouse_left_down && !left_holding {
            // just started holding, setting starting point
            left_holding_x = mouse_x;
            left_holding_y = mouse_y;
            left_holding = true;
        } else if mouse_left_down && left_holding {
            // in the process of holding, draw vector from start to mouse
            draw_arrow(
                &mut d,
                &Vector2::new(
                    left_holding_x as f32,
                    left_holding_y as f32,
                ),
                &Vector2::new(
                    mouse_x as f32,
                    mouse_y as f32,
                ),
                7.3,
                Color::RED,
            )
        } else if !mouse_left_down && left_holding {
            // finished dragging, add the vector to vectors
            vectors.push((
                scaler.to_world_v(Vector2 {
                    x: left_holding_x as f32,
                    y: left_holding_y as f32,
                }),
                scaler.to_world_v(Vector2 {
                    x: mouse_x as f32,
                    y: mouse_y as f32,
                }),
            ));
            left_holding = false;
        }

        d.clear_background(Color::BLACK);
        for vec in &vectors {
            draw_arrow(
                &mut d,
                &scaler.to_screen_v(vec.0),
                &scaler.to_screen_v(vec.1),
                7.3,
                Color::BLUE,
            );
        }
    }
}

fn draw_arrow(d: &mut RaylibDrawHandle<'_>, p0: &Vector2, p1: &Vector2, radius: f32, color: Color) {
    d.draw_line_v(p0, p1, color);

    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;

    let length = (dx * dx + dy * dy).sqrt();

    let vx = dy / length;
    let vy = dx / length;

    let angle = -vy.atan2(vx) * 180.0 / PI;

    d.draw_poly(p1, 3, radius, angle, color);
}
