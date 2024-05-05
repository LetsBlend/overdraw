use std::ffi::c_void;
use std::mem;
use gl::types::*;

pub struct Mesh {
    mesh_id: GLuint,
    vertex_id: GLuint,
    index_id: GLuint
}

impl Mesh {
    pub fn new() -> Self {
        unsafe {
            let mut mesh_id = 0;
            gl::CreateVertexArrays(1, &mut mesh_id);

            gl::BindVertexArray(mesh_id);

            let vertex_buffer = VertexBuffer::new();
            let index_buffer = IndexBuffer::new();

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<f32>() as GLsizei, 0 as *const c_void);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * mem::size_of::<f32>() as GLsizei, (3 * mem::size_of::<f32>()) as *const c_void);
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);

            Mesh {
                mesh_id,
                vertex_id: vertex_buffer.buffer_id,
                index_id: index_buffer.buffer_id
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.mesh_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.mesh_id);
            gl::DeleteBuffers(1, &mut self.vertex_id);
            gl::DeleteBuffers(1, &mut self.index_id);
        }
    }
}

pub struct VertexBuffer {
    buffer_id: GLuint,
}

impl VertexBuffer {
    pub fn new() -> Self {
        unsafe {
            let vertices: [f32; 20] = [
                1.0, 1.0, 0.0,   1.0, 1.0, // top right
                1.0, -1.0, 0.0,  1.0, 0.0,  // bottom right
                -1.0, -1.0, 0.0, 0.0, 0.0,  // bottom left
                -1.0, 1.0, 0.0,  0.0, 1.0// top left
            ];

            let mut buffer_id = 0;
            gl::CreateBuffers(1, &mut buffer_id);

            gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);
            gl::BufferData(gl::ARRAY_BUFFER, (vertices.len() * mem::size_of::<f32>()) as GLsizeiptr, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);
            VertexBuffer { buffer_id }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.buffer_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

pub struct IndexBuffer {
    buffer_id: GLuint,
}

impl IndexBuffer {
    pub fn new() -> Self {
        unsafe {
            let indices: [i32; 6] = [
                0, 1, 3,   // first triangle
                1, 2, 3    // second triangle
            ];

            let mut buffer_id = 0;
            gl::CreateBuffers(1, &mut buffer_id);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer_id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * mem::size_of::<f32>()) as GLsizeiptr, indices.as_ptr() as *const c_void, gl::STATIC_DRAW);
            IndexBuffer{ buffer_id }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.buffer_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}