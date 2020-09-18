extern crate assimp;
extern crate cgmath;
#[macro_use]
extern crate glium;

use assimp::{Importer, LogStream};
use cgmath::{perspective, Matrix4, Deg, Vector3, Point3};
use glium::{glutin, Surface};
use glium::index::PrimitiveType;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone, Debug)]
    struct Vertex3 {
        position: [f32; 3],
        normal: [f32; 3]
    }
    implement_vertex!(Vertex3, position, normal);

    // Setup logging
    LogStream::set_verbose_logging(true);
    let mut log_stream = LogStream::stdout();
    log_stream.attach();

    // Load shaders
    let program = program!(&display,
        130 => {
            vertex: "
                #version 130

                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;

                in vec3 position;
                in vec3 normal;

                out vec3 v_normal;

                void main() {
                    v_normal = normal;
                    gl_Position = persp_matrix * view_matrix * vec4(position, 1.0);
                }
            ",

            fragment: "
                #version 130

                in vec3 v_normal;
                out vec4 f_color;

                void main() {
                    f_color = vec4(v_normal, 1.0);
                }
            ",
        }
    ).unwrap();

    let mut vertex_buffers = Vec::new();
    let mut index_buffers = Vec::new();

    {
        let mut importer = Importer::new();
        importer.triangulate(true);
        importer.generate_normals(|x| x.enable = true);
        importer.pre_transform_vertices(|x| {
            x.enable = true;
            x.normalize = true
        });
        let scene = importer.read_file("examples/spider.obj").unwrap();

        for mesh in scene.mesh_iter() {
            let verts: Vec<Vertex3> = mesh.vertex_iter().zip(mesh.normal_iter()).map(|(v, n)|
                Vertex3 {
                    position: v.into(),
                    normal: n.into()
                }
            ).collect();

            // Create vertex buffer
            let vb = glium::VertexBuffer::new(&display, &verts);
            vertex_buffers.push(vb.unwrap());

            // Safe to assume all faces are triangles due to import options
            let mut indices = Vec::with_capacity(mesh.num_faces() as usize * 3);
            for face in mesh.face_iter() {
                indices.push(face[0]);
                indices.push(face[1]);
                indices.push(face[2]);
            }

            let ib = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &indices);
            index_buffers.push(ib.unwrap());
        }
    }

    // Setup perspective camera
    let eye = Point3::new(0.0, 3.0, 3.0);
    let pos = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let persp_matrix: [[f32; 4]; 4] = perspective(Deg(60.0), 1.333, 0.1, 1000.0).into();
    let view_matrix: [[f32; 4]; 4] = Matrix4::look_at(eye, pos, up).into();;

    let uniforms = uniform! {
        persp_matrix: persp_matrix,
        view_matrix: view_matrix
    };

    let draw = || {
        let mut target = display.draw();
        target.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        for i in 0..vertex_buffers.len() {
            target.draw(&vertex_buffers[i],
                        &index_buffers[i],
                        &program,
                        &uniforms,
                        &params).unwrap();
        }

        target.finish().unwrap();
    };

    draw();

    // Main loop
    events_loop.run_forever(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => return glutin::ControlFlow::Break,
                glutin::WindowEvent::Resized(..) => draw(),
                _ => (),
            },
            _ => (),
        }
        glutin::ControlFlow::Continue
    });
}
