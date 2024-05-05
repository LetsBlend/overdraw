//#![windows_subsystem = "windows"]
// #![cfg(not(debug_assertions), windows_subsystem = "windows")]

// include!(concat!(env!("OUT_DIR"), "/custom_bindings.rs"));

mod input;
mod open_gl;
mod window;
mod shader_code;
mod gui;
mod history;

use std::str::from_utf8;
use shader_code::*;

use std::time;
use device_query::Keycode;
use input::Input;

use window::Win;

use open_gl::*;
use glm::*;
use gui::*;

use texture::*;
use shaders::*;
use buffers::Mesh;
use crate::history::History;

fn main() {
    let mut input = Input::new();

    let mut window = Win::new();
    viewport(0, 0, window.get_width(), window.get_height());

    let mut gui = Gui::new(&mut window.p_window);

    let mesh = Mesh::new();
    let shader = StandardShader::new(VERTEX_SHADER, PIXEL_SHADER);

    let mut history = History::new();
    history.push_back(Texture2D::new(window.get_width(), window.get_height()));
    let compute_shader = ComputeShader::new(COMPUTE_SHADER);

    let mut brush_size = 5;
    let mut prev_brush_size = 5;
    let mut prev_cursor_pos = input.get_cursor_pos();

    let mut color_picked = vec4(1.0, 1.0, 1.0, 1.0);
    let mut is_hovered = false;
    // Only here because bug in input library
    let mut first_click = false;
    while window.open() {
        window.poll_events(&mut input, &mut gui.imgui, &mut gui.imgui_glfw);
        input.poll();

        if !window.edit(&input) {
            std::thread::sleep(time::Duration::from_millis(100));
            continue;
        }

        // Undo
        if input.get_key(Keycode::LControl) {
            if input.get_key_down(Keycode::Z) {
                history.undo();
            }
        }

        // Redo
        if input.get_key(Keycode::LControl) {
            if input.get_key_down(Keycode::Y) {
                history.redo();
            }
        }

        // Compute
        let mut cursor_coords = vec2(input.get_cursor_pos().x, window.get_size().y - input.get_cursor_pos().y);
        let prev_cursor_coords = vec2(input.get_prev_cursor_pos().x, window.get_size().y - input.get_prev_cursor_pos().y);

        if !is_hovered {
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // This is only here because there is a bug in the input library with the first click, and I am too lazy to properly fix it
            if !first_click && !input.get_key(Keycode::LAlt) && (input.get_button(1) || input.get_button(2)) {
                history.pop_undos();
                history.push_back(history.get_current().clone());
                first_click = true;
            }
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////

            if !input.get_key(Keycode::LAlt) && (input.get_button_down(1) || input.get_button_down(2)) {
                history.pop_undos();
                history.push_back(history.get_current().clone());
            } else if !input.get_key(Keycode::LAlt) && (input.get_button(1) || input.get_button(2)) {
                let color = if input.get_button(2) {
                    vec4(0.0, 0.0, 0.0, 0.0)
                } else {
                    color_picked
                };
                paint(&compute_shader, history.get_current(), color, cursor_coords, prev_cursor_coords, brush_size);
            } else if input.get_key(Keycode::LAlt) {
                /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
                // This is only here because there is a bug in the input library with the first click, and I am too lazy to properly fix it
                if !first_click && input.get_button(1) {
                    prev_cursor_pos = input.get_cursor_pos();
                    prev_brush_size = brush_size;
                    first_click = true;
                }
                /////////////////////////////////////////////////////////////////////////////////////////////////////////////////

                if input.get_button_down(1) {
                    prev_cursor_pos = input.get_cursor_pos();
                    prev_brush_size = brush_size;
                }
                if input.get_button(1) {
                    brush_size = abs(input.get_cursor_pos().x + 1.0 * prev_brush_size as f32 - prev_cursor_pos.x) as i32;
                    cursor_coords = vec2(prev_cursor_pos.x, window.get_size().y - prev_cursor_pos.y);
                }
                if input.get_button_down(2) {
                    history.get_current().clear();
                }
            }
        }

        // Render to screen
        shader.set_ivector2(&cursor_coords, "cursor_pos");
        shader.set_ivector2(&window.get_size(), "screen_size");
        shader.set_int(brush_size, "brush_size");

        mesh.bind();
        history.get_current().bind(gl::TEXTURE0);
        shader.bind();
        draw_indexed(6);
        shader.unbind();
        history.get_current().unbind(gl::TEXTURE0);
        mesh.unbind();

        // Gui
        is_hovered = gui.show_gui(&mut window.p_window, &mut color_picked, &mut first_click, &mut brush_size, &mut history);
        if input.get_button(2) {
            is_hovered = false;
        }

        window.swap_buffers();
        clear(0.0, 0.0, 0.0, 0.0);
    }
}

fn paint(compute_shader: &ComputeShader, texture: &Texture2D, color: Vec4, cursor_coords: Vec2, prev_cursor_coords: Vec2, brush_size: i32) {
    compute_shader.set_ivector2(&cursor_coords, "cursor_pos");
    compute_shader.set_ivector2(&prev_cursor_coords, "prev_cursor_pos");
    compute_shader.set_int(brush_size, "brush_size");
    compute_shader.set_vector4(&color, "color");

    let size = clamp(abs(cursor_coords - prev_cursor_coords) + 2.0 * brush_size as f32, vec2(0.0, 0.0), vec2(texture.get_width() as f32, texture.get_height() as f32));
    texture.bind_image(gl::TEXTURE0);
    compute_shader.bind();
    compute_shader.dispatch(ceil(size.x / 10.0) as u32, ceil(size.y / 10.0) as u32, 1);
    compute_shader.wait();
    compute_shader.unbind();
    texture.unbind_image(gl::TEXTURE0);
}