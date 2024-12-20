use gl::types::*;
use glfw::{Action, Context, Key};
use std::ffi::c_void;
use std::path::Path;
use std::ptr;
use std::time;
use utils::Shader;
extern crate image;

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

    // Allow resize of window
    window.set_size_callback(change_window_size);

    // Set the viewport
    unsafe {
        gl::Viewport(0, 0, WIDTH as GLint, HEIGHT as GLint);
    }

    // Declare the vertices as the whole screen
    #[rustfmt::skip]
    let vertices= [
        //Positions //Textures
        -0.5, -0.5, 0.0, 0.0,
        -0.5,  0.5, 0.0, 1.0,
         0.5, -0.5, 1.0, 0.0,
        -0.5,  0.5, 0.0, 1.0,
         0.5, -0.5, 1.0, 0.0,
         0.5,  0.5, 1.0, 1.0f32,
    ];

    // Obtain the shader program and vertex array object
    let (shader_program, vao, texture1, texture2) = unsafe {
        // Create shader from file
        let shader_class = Shader::from_file(Path::new("vertex.glsl"), Path::new("fragment.glsl"))
            .expect("Cannot create shader class");

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
        // This one is for position
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            4 * size_of_val(&vertices[0]) as i32,
            ptr::null::<c_void>(),
        );
        gl::EnableVertexAttribArray(0);

        // This one is for texture
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            4 * size_of_val(&vertices[0]) as i32,
            (2 * size_of_val(&vertices[0])) as *const c_void,
            // ptr::null::<c_void>(),
        );
        gl::EnableVertexAttribArray(1);

        // Create the container texture
        let img = image::open(Path::new("./container.jpg")).expect("Cannot load texture image.");
        let data = img
            .as_flat_samples_u8()
            .expect("Cannot flatten texture image.");

        let mut texture1 = 0;

        gl::GenTextures(1, &mut texture1);

        // Active texture is 0 by default
        gl::ActiveTexture(gl::TEXTURE0);

        gl::BindTexture(gl::TEXTURE_2D, texture1);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            data.samples.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        // Create the moai texture
        let img = image::open(Path::new("./moai.png")).expect("Cannot load texture image.");
        let data = img
            .as_flat_samples_u8()
            .expect("Cannot flatten texture image.");

        let mut texture2 = 0;
        gl::GenTextures(1, &mut texture2);

        gl::ActiveTexture(gl::TEXTURE1);

        gl::BindTexture(gl::TEXTURE_2D, texture2);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.samples.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, 0);

        // Set texture options
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );

        // Unbind vertex array object
        gl::BindVertexArray(0);
        gl::ActiveTexture(gl::TEXTURE0);
        // Return shader program and vertex array object

        (shader_class.get(), vao, texture1, texture2)
    };

    let start_time = time::Instant::now();
    let time_uniform =
        unsafe { gl::GetUniformLocation(shader_program, "transparency\0".as_ptr() as *const i8) };

    unsafe {
        gl::UseProgram(shader_program);
        gl::Uniform1i(
            gl::GetUniformLocation(shader_program, "texture1\0".as_ptr() as *const i8),
            0,
        );
        gl::Uniform1i(
            gl::GetUniformLocation(shader_program, "texture2\0".as_ptr() as *const i8),
            1,
        );
        gl::UseProgram(0);
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

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);
            gl::BindVertexArray(vao);
            gl::UseProgram(shader_program);

            gl::Uniform1f(time_uniform, start_time.elapsed().as_secs_f32());

            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
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

// Change window size
fn change_window_size(_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}
