use crate::pipelines::rectangle::RectangleInstance;
use crate::pipelines::{Color, ScreenPosition};
use crate::renderer::device_context::DeviceContext;
use crate::renderer::window_context::WindowContext;
use crate::renderer::Renderer;
use std::time::{SystemTime, UNIX_EPOCH};
use wgpu::*;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub struct Application {
    renderer: Renderer,
}

impl Application {
    pub async fn new(event_loop: &EventLoop<()>) -> Self {
        // apparently this is necessary to get nice errors while using wgpu
        env_logger::init();
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let window_context = WindowContext::new(&instance, event_loop).await;
        let device_context = DeviceContext::new(&instance, &window_context.surface).await;
        let renderer = Renderer::new(window_context, device_context);
        renderer.initialize_surface();

        Self { renderer }
    }

    fn update(&mut self) {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let x = (time as f64 / 1000.0).sin();
        let y = (time as f64 / 1000.0).cos();

        let rects = [RectangleInstance {
            position: ScreenPosition(x as f32 * 0.5, y as f32 * 0.5),
            size: ScreenPosition(0.1, 0.1),
            color: Color(0.5, 0.7, 0.8),
        }];

        self.renderer.set_rectangles(&rects);
    }

    pub fn run_event_loop(mut self, event_loop: EventLoop<()>) -> ! {
        event_loop.run(move |event, _, control_flow| match event {
            Event::RedrawRequested(window_id) if window_id == self.renderer.window_id() => {
                self.update();
                match self.renderer.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(SurfaceError::Lost) => self.renderer.initialize_surface(),
                    // The system is out of memory, we should probably quit
                    Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually request it.
                self.renderer.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.renderer.window_id() => {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        self.renderer.configure_surface(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        self.renderer.configure_surface(**new_inner_size);
                    }
                    // exit when ESC is pressed
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            }
            _ => {}
        })
    }
}
