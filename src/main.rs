#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use beryllium::*;
use ogl33::*;
use rand::prelude::*;

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    {
        sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
            .unwrap();
    }

    let win_args = video::CreateWinArgs {
        title: "Your Mother",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: true,
        resizable: true,
    };

    let win = sdl
        .create_gl_window(win_args)
        .expect("couldn't make a window and context");

    unsafe {
        load_gl_with(|f_name| win.get_proc_address(f_name.cast()));

        let mut vao = 0;
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);

        let mut vbo = 0;
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        glBindBuffer(GL_ARRAY_BUFFER, vbo);
    }

    type Vertex = [f32; 3];
    const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];

    unsafe {
        glBufferData(
            GL_ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            GL_STATIC_DRAW,
        );
    }

    let mut rng = thread_rng();
    let mut c = 0;
    let mut r: GLfloat = 0.0;
    let mut g: GLfloat = 0.0;
    let mut b: GLfloat = 0.0;
    'main_loop: loop {
        // handle this frame's event
        while let Some(event) = sdl.poll_events() {
            match event {
                (events::Event::Quit, _) => break 'main_loop,
                _ => (),
            }
        }
        if c % 10 == 0 {
            r = rng.gen();
            g = rng.gen();
            b = rng.gen();
        }
        unsafe {
            glClearColor(r, g, b, 1.0);
            glClear(GL_COLOR_BUFFER_BIT);
        }
        c = c + 1;
        win.swap_window();
    }
    println!("Hello, world!");
    println!("Bruh");
}
