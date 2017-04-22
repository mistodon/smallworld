use glium::{Program, Texture2d};
use glium::backend::glutin_backend::GlutinFacade;
use glium::index::{IndexBufferAny, IndexBuffer, PrimitiveType};
use glium::program::{ProgramCreationInput};
use glium::texture::{RawImage2d, MipmapsOption};
use glium::vertex::{VertexBufferAny, VertexBuffer};
use image::{load_from_memory_with_format, ImageFormat};

pub type Display = GlutinFacade;
pub type Shader = Program;
pub type Texture = Texture2d;
pub type Mesh = (VertexBufferAny, IndexBufferAny);

pub struct TextureAtlas
{
    pub texture: Texture,
    pub width_tiles: u32,
    pub height_tiles: u32
}

impl TextureAtlas
{
    pub fn get_uv_offset_scale(&self, tx: u32, ty: u32) -> ([f32; 2], [f32; 2])
    {
        let u = (tx as f32) / (self.width_tiles as f32);
        let v = (ty as f32) / (self.height_tiles as f32);
        let su = 1.0 / (self.width_tiles as f32);
        let sv = 1.0 / (self.height_tiles as f32);
        ([u, v], [su, sv])
    }
}

#[derive(Copy, Clone)]
struct Vertex
{
    offset: [f32; 2],
    uv: [f32; 2]
}
implement_vertex!(Vertex, offset, uv);


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
        Vertex { offset: [-0.5, -0.5], uv: [0.0, 0.0] },
        Vertex { offset: [ 0.5, -0.5], uv: [1.0, 0.0] },
        Vertex { offset: [ 0.5,  0.5], uv: [1.0, 1.0] },
        Vertex { offset: [-0.5,  0.5], uv: [0.0, 1.0] }
    ];
    let indices = [0, 1, 2, 0, 2, 3_u16];
    let vertex_buffer = VertexBuffer::new(display, &vertices).expect("Failed to build vertex buffer");
    let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices).expect("Failed to build index buffer");
    (vertex_buffer.into(), index_buffer.into())
}

pub fn load_texture(display: &Display, bytes: &[u8]) -> (Texture, (u32, u32))
{
    let image = load_from_memory_with_format(bytes, ImageFormat::PNG).expect("Failed to decode image").to_rgba();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let texture = Texture2d::with_mipmaps(display, image, MipmapsOption::NoMipmap).expect("Failed to load texture");
    (texture, image_dimensions)
}

pub fn load_texture_atlas(display: &Display, bytes: &[u8], tile_size: u32) -> TextureAtlas
{
    let (texture, dimensions) = load_texture(display, bytes);
    let (w, h) = dimensions;
    let (tw, th) = (w / tile_size, h / tile_size);
    TextureAtlas { texture: texture, width_tiles: tw, height_tiles: th }
}

pub fn calculate_projection(resolution: (u32, u32), tile_size: u32) -> [f32; 2]
{
    let (w, h) = resolution;
    let (w, h) = (w as f32, h as f32);
    let tile_size = tile_size as f32;
    let projection = [2.0 * tile_size / w, 2.0 * tile_size / h];
    projection
}
