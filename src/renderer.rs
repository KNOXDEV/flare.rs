use crate::context::{DeviceContext, WindowContext};
use crate::pipelines::rectangle::RectangleRenderer;
use std::ops::Range;
use wgpu::*;
use winit::dpi::PhysicalSize;
use winit::window::WindowId;

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

pub struct Renderer {
    window_context: WindowContext,
    device_context: DeviceContext,
    surface_configuration: SurfaceConfiguration,
    rectangle_renderer: RectangleRenderer,
}

impl Renderer {
    pub fn new(window_context: WindowContext, device_context: DeviceContext) -> Self {
        let surface_configuration = window_context.surface_configuration(&device_context);

        let rectangle_renderer =
            RectangleRenderer::new(&device_context.device, surface_configuration.format);

        Self {
            window_context,
            device_context,
            surface_configuration,
            rectangle_renderer,
        }
    }

    pub fn configure_surface(&mut self, size: PhysicalSize<u32>) {
        self.surface_configuration.width = size.width;
        self.surface_configuration.height = size.height;
        self.initialize_surface();
    }

    pub fn initialize_surface(&self) {
        self.window_context
            .surface
            .configure(&self.device_context.device, &self.surface_configuration);
    }

    pub fn request_redraw(&self) {
        self.window_context.window.request_redraw();
    }

    pub fn window_id(&self) -> WindowId {
        self.window_context.window.id()
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let (output, view) = self.window_context.create_surface_texture()?;
        let mut encoder = self
            .device_context
            .create_command_encoder("Command Encoder");

        let render_items = self
            .rectangle_renderer
            .get_items(&self.device_context.device);

        {
            // we only have to create a new render pass between items if the framebuffer /
            // texture view changes, which should not happen in our application
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.3,
                            g: 0.4,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_items.into_iter().for_each(|item| {
                render_pass.set_pipeline(item.pipeline);
                item.vertex_buffers
                    .iter()
                    .enumerate()
                    .for_each(|(i, vertex_buffer)| {
                        render_pass.set_vertex_buffer(i as u32, vertex_buffer.slice(..));
                    });

                match item.draw_technique {
                    DrawTechnique::VertexOnly { vertices } => {
                        render_pass.draw(vertices, item.instances);
                    }
                    DrawTechnique::Indexed {
                        index_buffer,
                        indices,
                    } => {
                        render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
                        render_pass.draw_indexed(indices, 0, item.instances);
                    }
                }
            });
        }

        // submit will accept anything that implements IntoIter
        self.device_context
            .queue
            .submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
