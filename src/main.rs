use core::str;
use gl::types::*;
use glfw::{Action, Context, Key};
use std::ptr;

const VERTEX_SHADER_SOURCE: &str = "#version 330 core
layout (location = 0) in vec3 pos;

void main()
{
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
}\0";

const FRAGMENT_SHADER_SOURCE: &str = "#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.0f, 0.0f, 1.0f);
}\0";

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    // Set version to 3.3 with core profile
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // Initialize Window
    let (mut window, events) = glfw
        .create_window(
            800,
            800,
            "Hello this is microsoft",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol));

    // Set the window as the current context
    window.set_key_polling(true);
    window.make_current();

    // Set the viewport
    unsafe {
        gl::Viewport(0, 0, 800, 800);
    }

    // Declare the vertices
    #[rustfmt::skip]
    let vertices: [f32; 12] = [
        -0.5, -0.5, 0.,
        -0.5, 0.5, 0.,
        0.5, -0.5, 0.,
        0.5, 0.5, 0.,
    ];

    // Declare the indexes that makes up the triangles
    #[rustfmt::skip]
    let indices: [i32; 6] = [
        0, 1, 2,
        1, 2, 3,
    ];

    // Obtain the shader program and vertex array object
    let (shader_program, vao) = unsafe {
        // Create vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        gl::ShaderSource(
            vertex_shader,
            1,
            &(VERTEX_SHADER_SOURCE.as_ptr() as *const i8),
            ptr::null(),
        );
        gl::CompileShader(vertex_shader);

        // Check status of vertex shader
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut info_log: [u8; 256] = [0; 256];
            let mut length = 0;
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                &mut length,
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "VERTEX_SHADER ERROR: {}",
                str::from_utf8(&info_log[..length as usize]).expect("Cannot get error message")
            );
        }

        // Create fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        gl::ShaderSource(
            fragment_shader,
            1,
            &(FRAGMENT_SHADER_SOURCE.as_bytes().as_ptr() as *const i8),
            ptr::null(),
        );
        gl::CompileShader(fragment_shader);

        // Check status of fragment shader
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut info_log: [u8; 256] = [0; 256];
            let mut length = 0;
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                &mut length,
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "FRAGMENT_SHADER ERROR: {}",
                str::from_utf8(&info_log[..length as usize]).expect("Cannot get error message")
            );
        }

        // Create shader program
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check shader program status
        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut info_log: [u8; 256] = [0; 256];
            let mut length = 0;
            gl::GetProgramInfoLog(
                shader_program,
                512,
                &mut length,
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "SHADER_PROGRAM ERROR: {}",
                str::from_utf8(&info_log[..length as usize]).expect("Cannot get error message")
            );
        }

        // Delete unused shader
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        // Create vertex array object
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create vertex buffer object
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&vertices) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Create element buffer object
        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            size_of_val(&indices) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Create vertex attribute pointer
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * size_of_val(&vertices[0]) as i32,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // Unbind vertex array object
        gl::BindVertexArray(0);

        // Return shader program and vertex array object
        (shader_program, vao)
    };

    // Main loop
    while !window.should_close() {
        // Poll events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // Draw to the backbuffer
        unsafe {
            gl::ClearColor(1., 1., 1., 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            gl::BindVertexArray(0);
        }

        // Display to screen
        window.swap_buffers();
    }
}

// Handle events
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
