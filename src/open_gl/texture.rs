use gl::TEXTURE_2D;
use gl::types::*;

pub struct Texture2D {
    texture_id: GLuint,
    width: u32,
    height: u32
}

impl Texture2D {
    pub fn new(width: u32, height: u32) -> Self {
        unsafe {
            let mut texture_id = 0;
            gl::GenTextures(1, &mut texture_id);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as GLint, width as GLsizei, height as GLsizei, 0, gl::RGBA, gl::FLOAT, std::ptr::null());
            gl::BindTexture(gl::TEXTURE_2D, 0);

            Texture2D { texture_id, width, height }
        }
    }

    // use for sampler2D
    pub fn bind(&self, unit: GLenum) {
        unsafe {
            gl::ActiveTexture(unit);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn unbind(&self, unit: GLenum) {
        unsafe {
            gl::ActiveTexture(unit);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    // use for image2D
    pub fn bind_image(&self, unit: GLenum) {
        unsafe {
            gl::ActiveTexture(unit);
            gl::BindImageTexture(0, self.texture_id, 0, gl::FALSE, 0, gl::READ_WRITE, gl::RGBA32F);
        }
    }

    pub fn unbind_image(&self, unit: GLenum) {
        unsafe {
            gl::ActiveTexture(unit);
            gl::BindImageTexture(0, 0, 0, gl::FALSE, 0, gl::READ_WRITE, gl::RGBA32F);
        }
    }

    pub fn clone(&self) -> Self {
        unsafe {
            let dest = Texture2D::new(self.width, self.height);
            gl::CopyImageSubData(self.texture_id, gl::TEXTURE_2D, 0, 0, 0, 0, dest.texture_id, gl::TEXTURE_2D, 0, 0, 0, 0, dest.width as GLsizei, dest.height as GLsizei, 1);
            dest
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::ClearTexImage(self.texture_id, 0, gl::RGBA, gl::FLOAT, std::ptr::null());
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.texture_id);
        }
    }
}