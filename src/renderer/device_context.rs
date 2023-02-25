use wgpu::*;

pub struct DeviceContext {
    pub(crate) adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

impl DeviceContext {
    pub async fn new(instance: &Instance, surface: &Surface) -> Self {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("failed to find a compatible adapter");

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::empty(),
                    limits: Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .expect("failed to find a compatible device");

        Self {
            adapter,
            device,
            queue,
        }
    }

    pub fn create_command_encoder(&self, label: &str) -> CommandEncoder {
        self.device
            .create_command_encoder(&CommandEncoderDescriptor { label: Some(label) })
    }
}