use crate::application::Application;
use winit::event_loop::EventLoop;

mod application;
mod context;
mod pipelines;
mod renderer;

fn main() {
    let event_loop = EventLoop::new();
    let application = pollster::block_on(Application::new(&event_loop));

    // calling this function will never return
    application.run_event_loop(event_loop);
}
