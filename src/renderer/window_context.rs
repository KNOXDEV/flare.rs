use wgpu::*;
use winit::event_loop::EventLoop;
use winit::window::Window;
use crate::renderer::device_context::DeviceContext;

pub struct WindowContext {
    pub window: Window,
    pub surface: Surface,
}

impl WindowContext {
    pub async fn new(instance: &Instance, event_loop: &EventLoop<()>) -> Self {
        let window = Window::new(event_loop).expect("failed to create a new window");

        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        Self { window, surface }
    }

    pub fn create_surface_texture(&self) -> Result<(SurfaceTexture, TextureView), SurfaceError> {
        let surface_texture = self.surface.get_current_texture()?;
        let view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        Ok((surface_texture, view))
    }

    pub fn surface_configuration(&self, device_context: &DeviceContext) -> SurfaceConfiguration {
        let surface_caps = self.surface.get_capabilities(&device_context.adapter);
        let size = self.window.inner_size();

        // find an sRGB format
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.describe().srgb)
            .unwrap_or(surface_caps.formats[0]);

        SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        }
    }
}