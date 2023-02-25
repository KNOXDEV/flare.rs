use std::ops::Range;
use wgpu::*;

pub struct RenderItem<'a> {
    pub pipeline: &'a RenderPipeline,
    pub vertex_buffers: Vec<&'a Buffer>,
    pub draw_technique: DrawTechnique<'a>,
    pub instances: Range<u32>,
}

pub enum DrawTechnique<'a> {
    VertexOnly {
        vertices: Range<u32>,
    },
    Indexed {
        index_buffer: &'a Buffer,
        indices: Range<u32>,
    },
}