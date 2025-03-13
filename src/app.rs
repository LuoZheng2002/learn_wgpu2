use wgpu::util::DeviceExt;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::camera::Camera;
use crate::camera_uniform::CameraUniform;
use crate::input_context::InputContext;
use crate::render_context::RenderContext;
use crate::renderable::{Polygon, RENDERABLES};
use crate::state::State;

#[derive(Default)]
pub struct App {
    render_context: Option<RenderContext>,
    input_context: InputContext,
    state: State,
}

impl App {
    fn get_context(&self) -> &RenderContext {
        self.render_context.as_ref().unwrap()
    }
    fn get_context_mut(&mut self) -> &mut RenderContext {
        self.render_context.as_mut().unwrap()
    }
}

impl<'a> ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();
        self.render_context = Some(RenderContext::new(window));
        RENDERABLES.lock().unwrap().push(Box::new(Polygon));
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                let state = self.get_context_mut();
                state.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.get_context().window.request_redraw();

                // if !surface_configured {
                //     return;
                // }
                match self.get_context_mut().render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let new_size = self.get_context().size;
                        self.get_context_mut().resize(new_size);
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                        log::error!("OutOfMemory");
                        event_loop.exit();
                    }
                    // This happens when the a frame takes too long to present
                    Err(wgpu::SurfaceError::Timeout) => {
                        log::warn!("Surface timeout")
                    }
                }
            }
            _ => (),
        }
    }
}

