use glium::{Surface, implement_vertex, uniform};

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new().build().expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        color: [f32; 3],
    }
    implement_vertex!(Vertex, position, color);
    
    let shape = vec![
        Vertex { position: [ -0.5,  -0.5  ], color: [1.0, 0.0, 0.0] },
        Vertex { position: [  0.0,   0.5  ], color: [0.0, 1.0, 0.0] },
        Vertex { position: [  0.5,  -0.25 ], color: [0.0, 0.0, 1.0] }
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        out vec3 vertex_color;

        uniform mat4 matrix;

        void main() {
            vertex_color = color;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        
        in vec3 vertex_color;
        out vec4 color;

        void main() {
            color = vec4(vertex_color, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
    frame.finish().unwrap();

    let mut t: f32 = 0.0;
    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                winit::event::WindowEvent::RedrawRequested => {
                    t += 0.02;

                    let x = t.sin() * 0.5;

                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);
                    let uniforms = uniform! {
                        matrix: [
                            [t.cos(), t.sin(), 0.0, 0.0],
                            [-t.sin(), t.cos(), 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [ x,  0.0, 0.0, 1.0f32],
                        ]
                    };
                    target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
                    target.finish().unwrap();
                },
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });
}
