use gl::types::*;
use std::str;

pub struct Shader {
    shader_program: u32,
}

impl Shader {
    pub fn get(&self) -> u32 {
        self.shader_program
    }

    pub fn new(
        vertex_shader_path: &str,
        fragment_shader_path: &str,
    ) -> Result<Shader, Box<dyn std::error::Error>> {
        let vertex_shader_source = std::fs::read_to_string(vertex_shader_path)?;
        let vertex_shader_source = std::ffi::CString::new(vertex_shader_source)?;

        let fragment_shader_source = std::fs::read_to_string(fragment_shader_path)?;
        let fragment_shader_source = std::ffi::CString::new(fragment_shader_source)?;

        let vertex_shader = unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &vertex_shader_source.as_ptr(),
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
                &fragment_shader_source.as_ptr(),
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
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut info_log: [u8; 512] = [0; 512];
                let mut length = 0;
                gl::GetProgramInfoLog(
                    shader_program,
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
            Ok(Shader { shader_program })
        }
    }
}
