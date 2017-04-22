use glium::{Program, Texture2d};
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::{IndexBufferAny, IndexBuffer, PrimitiveType};
use glium::program::{ProgramCreationInput};
use glium::vertex::{VertexBufferAny, VertexBuffer};

pub type Display = GlutinFacade;
pub type Shader = Program;
pub type Texture = Texture2d;
pub type Mesh = (VertexBufferAny, IndexBufferAny);


#[derive(Copy, Clone)]
struct Vertex
{
    position: [f32; 2]
}
implement_vertex!(Vertex, position);


pub fn load_shader<V, F>(display: &Display, vertex_source: V, fragment_source: F) -> Shader
    where V: AsRef<str>, F: AsRef<str>
{
    Shader::new(display, ProgramCreationInput::SourceCode
    {
        vertex_shader: vertex_source.as_ref(),
        fragment_shader: fragment_source.as_ref(),
        outputs_srgb: true,
        geometry_shader: None,
        tessellation_control_shader: None,
        tessellation_evaluation_shader: None,
        transform_feedback_varyings: None,
        uses_point_size: false
    }).expect("Failed to load shaders!")
}

pub fn quad_mesh(display: &Display) -> Mesh
{
    let vertices = [
        Vertex { position: [-0.5, -0.5] },
        Vertex { position: [ 0.5, -0.5] },
        Vertex { position: [ 0.5,  0.5] },
        Vertex { position: [-0.5,  0.5] }
    ];
    let indices = [0, 1, 2, 0, 2, 3_u16];
    let vertex_buffer = VertexBuffer::new(display, &vertices).expect("Failed to build vertex buffer");
    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).expect("Failed to build index buffer");
    (vertex_buffer.into(), index_buffer.into())
}
