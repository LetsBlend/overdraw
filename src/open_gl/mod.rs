pub mod shaders;
pub mod buffers;
pub mod texture;

use gl::types::*;

pub fn viewport(pos_x: u32, pos_y: u32, width: u32, height: u32) {
    unsafe {
        gl::Viewport(pos_x as GLint, pos_y as GLint, width as GLsizei, height as GLsizei);
    }
}

pub fn clear(r: f32, b: f32, g: f32, a: f32) {
    unsafe {
        gl::ClearColor(r as GLfloat, g as GLfloat, b as GLfloat, a as GLfloat);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

pub fn draw(count: i32) {
    unsafe {
        gl::DrawArrays(gl::TRIANGLES, 0, count);
    }
}

pub fn draw_indexed(count: i32) {
    unsafe {
        gl::DrawElements(gl::TRIANGLES, count, gl::UNSIGNED_INT, std::ptr::null());
    }
}