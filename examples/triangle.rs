#[macro_use(gfx_vertex, gfx_parameters)]
extern crate gfx;
extern crate gfx_device_gl;
extern crate window;
extern crate input;
extern crate shader_version;
extern crate glutin;
extern crate glutin_window;
extern crate gfx_window_shared;

use std::sync::{ Arc, RwLock };
use window::{ Window, WindowSettings };
use shader_version::OpenGL;
use glutin_window::GlutinWindow;
use gfx::traits::FactoryExt;
use gfx::{ batch, Stream, ClearData };
use gfx_window_shared::init_sync;
use input::{ Input, Button, Key };


gfx_vertex!( Vertex {
    a_Pos@ pos: [f32; 2],
    a_Color@ color: [f32; 3],
});


fn main() {
    const GLVERSION: OpenGL = OpenGL::_2_1;
    let settings = WindowSettings::new("shared glutin window", (640, 480));
    let window = Arc::new(RwLock::new(GlutinWindow::new(GLVERSION, settings)));
    let (mut stream, mut device, mut factory) = init_sync(window.clone());

    let batch = {
        let vertex_data = [
            Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
            Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
            Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] },
        ];
        let mesh = factory.create_mesh(&vertex_data);
        let program = {
            let vs = gfx::ShaderSource {
                glsl_120: Some(include_bytes!("triangle_120.glslv")),
                glsl_150: Some(include_bytes!("triangle_150.glslv")),
                .. gfx::ShaderSource::empty()
            };
            let fs = gfx::ShaderSource {
                glsl_120: Some(include_bytes!("triangle_120.glslf")),
                glsl_150: Some(include_bytes!("triangle_150.glslf")),
                .. gfx::ShaderSource::empty()
            };
            factory.link_program_source(vs, fs).unwrap()
        };
        batch::Full::new(mesh, program, None).unwrap()
    };

    'main: while !window.read().unwrap().should_close() {
        // quit when Esc is pressed.
        while let Some(event) = Window::poll_event(&mut *window.write().unwrap()) {
            match event {
                Input::Press(Button::Keyboard(Key::Escape)) => break 'main,
                _ => (),
            }
        }

        stream.clear(gfx::ClearData {
            color: [0.3, 0.3, 0.3, 1.0],
            depth: 1.0,
            stencil: 0,
        });
        stream.draw(&batch).unwrap();
        stream.present(&mut device);
    }
}
