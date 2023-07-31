use winit;

pub struct Window {
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
}

impl Window {

}

pub struct WindowBuilder {
    window_builder: winit::window::WindowBuilder,
    event_loop: winit::event_loop::EventLoop<()>,
}

impl WindowBuilder {
    pub fn new() -> Self {
        // Create the event loop
        let event_loop = winit::event_loop::EventLoop::new();

        // Create the window builder
        let window_builder = winit::window::WindowBuilder::new();

        Self {event_loop, window_builder}
    }
}