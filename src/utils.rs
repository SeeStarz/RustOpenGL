use gl::types::*;
use std::{ffi::CStr, path::Path, str};

pub struct Shader {
    shader_program_id: u32,
}

impl Shader {
    pub fn get(&self) -> u32 {
        self.shader_program_id
    }

    pub fn from_cstr(
        vertex_source: &CStr,
        fragment_source: &CStr,
    ) -> Result<Shader, Box<dyn std::error::Error>> {
        Shader::new(vertex_source, fragment_source)
    }

    pub fn from_file(
        vertex_source_path: &Path,
        fragment_source_path: &Path,
    ) -> Result<Shader, Box<dyn std::error::Error>> {
        let vertex_string = std::fs::read_to_string(vertex_source_path)?;
        let vertex_source = std::ffi::CString::new(vertex_string)?;

        let fragment_string = std::fs::read_to_string(fragment_source_path)?;
        let fragment_source = std::ffi::CString::new(fragment_string)?;

        Shader::new(&vertex_source, &fragment_source)
    }

    fn new(
        vertex_source: &CStr,
        fragment_source: &CStr,
    ) -> Result<Shader, Box<dyn std::error::Error>> {
        let vertex_shader = unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &vertex_source.as_ptr(),
                std::ptr::null::<i32>(),
            );
            gl::CompileShader(vertex_shader);

            let mut success = gl::FALSE as i32;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut info_log: [u8; 512] = [0; 512];
                let mut length = 0;
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    &mut length,
                    info_log.as_mut_ptr() as *mut GLchar,
                );

                let message = str::from_utf8(&info_log[..length as usize])
                    .expect("Cannot retrieve error message");
                return Err(format!("VERTEX SHADER ERROR: {message}").into());
            }

            vertex_shader
        };

        let fragment_shader = unsafe {
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &fragment_source.as_ptr(),
                std::ptr::null::<i32>(),
            );
            gl::CompileShader(fragment_shader);

            let mut success = gl::FALSE as i32;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut info_log: [u8; 512] = [0; 512];
                let mut length = 0;
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    &mut length,
                    info_log.as_mut_ptr() as *mut GLchar,
                );

                let message = str::from_utf8(&info_log[..length as usize])
                    .expect("Cannot retrieve error message");
                return Err(format!("FRAGMENT SHADER ERROR: {message}").into());
            }

            fragment_shader
        };

        unsafe {
            let shader_program_id = gl::CreateProgram();
            gl::AttachShader(shader_program_id, vertex_shader);
            gl::AttachShader(shader_program_id, fragment_shader);
            gl::LinkProgram(shader_program_id);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut info_log: [u8; 512] = [0; 512];
                let mut length = 0;
                gl::GetProgramInfoLog(
                    shader_program_id,
                    512,
                    &mut length,
                    info_log.as_mut_ptr() as *mut GLchar,
                );

                let message = str::from_utf8(&info_log[..length as usize])
                    .expect("Cannot retrieve error message");
                return Err(format!("VERTEX SHADER ERROR: {message}").into());
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            Ok(Shader { shader_program_id })
        }
    }
}
