use glium::{Program, Texture2d};
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::{IndexBufferAny};
use glium::program::{ProgramCreationInput};
use glium::vertex::{VertexBufferAny};

pub type Display = GlutinFacade;
pub type Shader = Program;
pub type Texture = Texture2d;
pub type Mesh = (VertexBufferAny, IndexBufferAny);

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
