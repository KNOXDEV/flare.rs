use crate::renderer::render_item::{DrawTechnique, RenderItem};
use wgpu::util::DeviceExt;
use wgpu::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex(f32, f32);

const VERTEX_LAYOUT: VertexBufferLayout = VertexBufferLayout {
    array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
    step_mode: VertexStepMode::Vertex,
    attributes: &vertex_attr_array![0 => Float32x3],
};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct RectangleInstance {
    position: [f32; 2],
    size: [f32; 2],
    color: [f32; 3],
}

const INSTANCE_VERTEX_LAYOUT: VertexBufferLayout = VertexBufferLayout {
    array_stride: std::mem::size_of::<RectangleInstance>() as BufferAddress,
    step_mode: VertexStepMode::Instance,
    attributes: &vertex_attr_array![1 => Float32x2, 2 => Float32x2, 3 => Float32x3],
};

const RECTANGLE_VERTS: &[Vertex] = &[
    Vertex(1.0, 1.0),
    Vertex(-1.0, 1.0),
    Vertex(-1.0, -1.0),
    Vertex(1.0, -1.0),
];

const RECTANGLE_INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

const RECTANGLE_INSTANCES: &[RectangleInstance] = &[
    RectangleInstance {
        position: [0.0, 0.0],
        size: [0.1, 0.1],
        color: [0.5, 0.7, 0.8],
    },
    RectangleInstance {
        position: [0.0, 0.0],
        size: [0.05, 0.05],
        color: [0.8, 0.5, 0.5],
    },
];

pub struct RectangleRenderer {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    render_pipeline: RenderPipeline,
    instance_buffer: Option<Buffer>,
}

impl RectangleRenderer {
    pub fn new(device: &Device, texture_format: TextureFormat) -> Self {
        let vertex_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Rectangle Vertex Buffer"),
            contents: bytemuck::cast_slice(RECTANGLE_VERTS),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Rectangle Index Buffer"),
            contents: bytemuck::cast_slice(RECTANGLE_INDICES),
            usage: BufferUsages::INDEX,
        });

        let shader = device.create_shader_module(include_wgsl!("rectangle.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[VERTEX_LAYOUT, INSTANCE_VERTEX_LAYOUT],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: texture_format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer: None,
        }
    }

    pub fn get_items(&mut self, device: &Device) -> Vec<RenderItem> {
        let instance_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Rectangle Vertex Buffer"),
            contents: bytemuck::cast_slice(RECTANGLE_INSTANCES),
            usage: BufferUsages::VERTEX,
        });

        self.instance_buffer = Option::from(instance_buffer);

        let item = RenderItem {
            pipeline: &self.render_pipeline,
            vertex_buffers: vec![&self.vertex_buffer, self.instance_buffer.as_ref().unwrap()],
            draw_technique: DrawTechnique::Indexed {
                index_buffer: &self.index_buffer,
                indices: 0..RECTANGLE_INDICES.len() as u32,
            },
            instances: 0..RECTANGLE_INSTANCES.len() as u32,
        };

        vec![item]
    }
}
