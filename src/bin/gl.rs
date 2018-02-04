#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use glium::index::PrimitiveType;

fn main() {
  let mut events_loop = glutin::EventsLoop::new();
  let window = glutin::WindowBuilder::new()
    .with_dimensions(1024, 768)
    .with_title("Hello world");
  let context = glutin::ContextBuilder::new();
  let display = glium::Display::new(window, context, &events_loop).unwrap();

  // building the vertex buffer, which contains all the vertices that we will draw
  let vertex_buffer = {
    #[derive(Copy, Clone)]
    struct Vertex {
      position: [f32; 2],
      color: [f32; 3],
    }

    implement_vertex!(Vertex, position, color);

    glium::VertexBuffer::new(
      &display,
      &[
        Vertex {
          position: [-0.5, -0.5],
          color: [0.0, 1.0, 1.0],
        },
        Vertex {
          position: [0.0, 0.5],
          color: [0.0, 0.0, 1.0],
        },
        Vertex {
          position: [0.5, -0.5],
          color: [0.0, 0.0, 1.0],
        },
      ],
    ).unwrap()
  };

  // building the index buffer
  let index_buffer =
    glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap();

  // compiling shaders and linking them together
  let program = program!(&display,
    140 => {
      vertex: "
        #version 140
        uniform mat4 matrix;
        in vec2 position;
        in vec3 color;
        out vec3 vColor;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0) * matrix;
            vColor = color;
        }
      ",

      fragment: "
        #version 140
        in vec3 vColor;
        out vec4 f_color;
        void main() {
            f_color = vec4(vColor, 1.0);
        }
      "
    },

    110 => {
      vertex: "
        #version 110
        uniform mat4 matrix;
        attribute vec2 position;
        attribute vec3 color;
        varying vec3 vColor;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0) * matrix;
            vColor = color;
        }
      ",

      fragment: "
        #version 110
        varying vec3 vColor;
        void main() {
            gl_FragColor = vec4(vColor, 1.0);
        }
      ",
    },

    100 => {
      vertex: "
        #version 100
        uniform lowp mat4 matrix;
        attribute lowp vec2 position;
        attribute lowp vec3 color;
        varying lowp vec3 vColor;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0) * matrix;
            vColor = color;
        }
      ",

      fragment: "
        #version 100
        varying lowp vec3 vColor;
        void main() {
            gl_FragColor = vec4(vColor, 1.0);
        }
      ",
    },
  ).unwrap();

  let draw = |t : f32| {
    // building the uniforms
    let uniforms = uniform! {
        matrix: [
          [t.cos(), -t.sin(), 0.0, 0.0],
          [t.sin(), t.cos(), 0.0, 0.0],
          [0.0, 0.0, 1.0, 0.0],
          [0.0, 0.0, 0.0, 1.0f32]
        ]
    };

    // drawing a frame
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 0.0);
    target
      .draw(
        &vertex_buffer,
        &index_buffer,
        &program,
        &uniforms,
        &Default::default(),
      )
      .unwrap();
    target.finish().unwrap();
  };

  let mut t : f32 = 0f32;
  events_loop.run_forever(|event| {
    t += 0.00001 % 2.0 * 3.14159f32;
    match event {
      glutin::Event::WindowEvent { event, .. } => match event {
        // Break from the main loop when the window is closed.
        glutin::WindowEvent::Closed => return glutin::ControlFlow::Break,
        // Redraw the triangle when the window is resized.
        glutin::WindowEvent::Resized(..) => draw(t),
        _ => draw(t),
      },
      _ => (),
    }
    glutin::ControlFlow::Continue
  });
}
