use gl::types::*;
use glfw::{Action, Context, Key};
use std::ffi::c_void;
use std::ptr;
use utils::Shader;

pub mod utils;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

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
            WIDTH,
            HEIGHT,
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
        gl::Viewport(0, 0, WIDTH as GLint, HEIGHT as GLint);
    }

    // Declare the vertices as the whole screen
    #[rustfmt::skip]
    let vertices: [f32; 12] = [
        -1.0, -1.0,
        -1.0, 1.0,
        1.0, -1.0,
        -1.0, 1.0,
        1.0, -1.0,
        1.0, 1.0,
    ];

    // Obtain the shader program and vertex array object
    let (shader_program, vao) = unsafe {
        // Create shader from file
        let shader_class =
            Shader::new("vertex.glsl", "fragment.glsl").expect("Cannot create shader class");

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
            (size_of_val(&vertices)) as isize,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Create vertex attribute pointer
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            2 * size_of_val(&vertices[0]) as i32,
            ptr::null::<c_void>(),
        );
        gl::EnableVertexAttribArray(0);

        // Unbind vertex array object
        gl::BindVertexArray(0);
        // Return shader program and vertex array object

        (shader_class.get(), vao)
    };

    // Pass window size
    unsafe {
        let window_size_location =
            gl::GetUniformLocation(shader_program, "windowSize\0".as_ptr() as *const i8);
        gl::UseProgram(shader_program);
        gl::Uniform2f(window_size_location, WIDTH as f32, HEIGHT as f32);
        gl::UseProgram(0);
    }

    // Start time counter
    let time_start = std::time::Instant::now();

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
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            let value_location =
                gl::GetUniformLocation(shader_program, "time\0".as_ptr() as *const i8);
            let time_elapsed = time_start.elapsed().as_millis() as f32 / 1000.0;
            gl::Uniform1f(value_location, time_elapsed);

            gl::BindVertexArray(0);
            gl::UseProgram(0);
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
