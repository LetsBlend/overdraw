use std::env::var;
use std::ffi::CString;
use std::str::Utf8Error;
use gl::types::*;

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Shader
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

trait Shader {
    fn compile(shader: &str, shader_type: GLenum) -> Result<GLuint, String> {
        unsafe {
            // Create Shader
            let c_str: *const i8 = shader.as_ptr() as *const i8;
            let shader_id = gl::CreateShader(shader_type);
            gl::ShaderSource(shader_id, 1, &c_str, std::ptr::null());
            gl::CompileShader(shader_id);

            // Validate Shader
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = vec![0; len as usize];
                gl::GetShaderInfoLog(shader_id, len, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);

                let message = String::from_utf8(info_log).unwrap_or_else(|err| format!("[ERROR::SHADER::COMPILATION_FAILED]: {}", err));
                return Err(message);
            }
            Ok(shader_id)
        }
    }

    fn link(program_id: GLuint) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(program_id);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = vec![0; len as usize];
                gl::GetProgramInfoLog(program_id, len, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);

                let message = String::from_utf8(info_log).unwrap_or_else(|err| format!("[ERROR::SHADER::COMPILATION_FAILED]: {}", err));
                return Err(message);
            }
            Ok(())
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Standard Shader
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct StandardShader {
    program_id: GLuint
}

impl Shader for StandardShader {}
impl StandardShader {
    pub fn new(vertex_shader: &str, pixel_shader: &str) -> Self {
        unsafe {
            let program_id = gl::CreateProgram();

            let vertex_id = Self::compile(vertex_shader, gl::VERTEX_SHADER).expect("[ERROR::SHADER::COMPILATION_FAILED]");
            gl::AttachShader(program_id, vertex_id);

            let pixel_id = Self::compile(pixel_shader, gl::FRAGMENT_SHADER).expect("[ERROR::SHADER::COMPILATION_FAILED]");

            gl::AttachShader(program_id, pixel_id);

            Self::link(program_id).expect("[ERROR::SHADER::LINKING_FAILED]");

            gl::DeleteShader(vertex_id);
            gl::DeleteShader(pixel_id);

            StandardShader { program_id }
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
    pub fn set_int(&self, value: i32, variable: &str) {
        unsafe {
            gl::UseProgram(self.program_id);
            let c_variable = CString::new(variable).unwrap();
            let location = gl::GetUniformLocation(self.program_id, c_variable.as_ptr() as *const GLchar);
            gl::Uniform1i(location, value);
            gl::UseProgram(0);
        }
    }
    pub fn set_vector2(&self, value: &glm::Vec2, variable: &str) {
        unsafe {
            gl::UseProgram(self.program_id);
            let c_variable = CString::new(variable).unwrap();
            let location = gl::GetUniformLocation(self.program_id, c_variable.as_ptr() as *const GLchar);
            gl::Uniform2f(location, value.x, value.y);
            gl::UseProgram(0);
        }
    }
    pub fn set_ivector2(&self, value: &glm::Vec2, variable: &str) {
        unsafe {
            gl::UseProgram(self.program_id);
            let c_variable = CString::new(variable).unwrap();
            let location = gl::GetUniformLocation(self.program_id, c_variable.as_ptr() as *const GLchar);
            gl::Uniform2i(location, value.x as GLint, value.y as GLint);
            gl::UseProgram(0);
        }
    }
}

impl Drop for StandardShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Compute Shader
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ComputeShader {
    program_id: GLuint
}

impl Shader for ComputeShader {}
impl ComputeShader {
    pub fn new(shader: &str) -> Self{
        unsafe {
            let program_id = gl::CreateProgram();

            let shader_id = Self::compile(shader, gl::COMPUTE_SHADER).expect("[ERROR::SHADER::COMPILATION_FAILED]");
            gl::AttachShader(program_id, shader_id);

            Self::link(program_id).expect("[ERROR::SHADER::LINKING_FAILED]");

            gl::DeleteShader(shader_id);

            ComputeShader{ program_id }
        }
    }

    pub fn dispatch(&self, x: u32, y: u32, z: u32) {
        unsafe {
            gl::DispatchCompute(x as GLuint, y as GLuint, z as GLuint);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn wait(&self) {
        unsafe {
            gl::MemoryBarrier(gl::ALL_BARRIER_BITS);
        }
    }

    pub fn wait_for(&self, barrier: GLenum) {
        unsafe  {
            gl::MemoryBarrier(barrier);
        }
    }
    pub fn set_int(&self, value: i32, variable: &str) {
        unsafe {
            gl::UseProgram(self.program_id);
            let c_variable = CString::new(variable).unwrap();
            let location = gl::GetUniformLocation(self.program_id, c_variable.as_ptr() as *const GLchar);
            gl::Uniform1i(location, value);
            gl::UseProgram(0);
        }
    }
    pub fn set_vector2(&self, value: &glm::Vec2, variable: &str) {
        unsafe {
            gl::UseProgram(self.program_id);
            let c_variable = CString::new(variable).unwrap();
            let location = gl::GetUniformLocation(self.program_id, c_variable.as_ptr() as *const GLchar);
            gl::Uniform2f(location, value.x, value.y);
            gl::UseProgram(0);
        }
    }
    pub fn set_vector4(&self, value: &glm::Vec4, variable: &str) {
        unsafe {
            gl::UseProgram(self.program_id);
            let c_string = CString::new(variable).unwrap();
            let location = gl::GetUniformLocation(self.program_id, c_string.as_ptr() as *const GLchar);
            gl::Uniform4f(location, value.x, value.y, value.z, value.w);
            gl::UseProgram(0);
        }
    }
    pub fn set_ivector2(&self, value: &glm::Vec2, variable: &str) {
        unsafe {
            gl::UseProgram(self.program_id);
            let c_variable = CString::new(variable).unwrap();
            let location = gl::GetUniformLocation(self.program_id, c_variable.as_ptr() as *const GLchar);
            gl::Uniform2i(location, value.x as GLint, value.y as GLint);
            gl::UseProgram(0);
        }
    }
}

impl Drop for ComputeShader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}











