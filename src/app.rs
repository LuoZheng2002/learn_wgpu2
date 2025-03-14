use wgpu::util::DeviceExt;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::camera::Camera;
use crate::camera_uniform::CameraUniform;
use crate::input_context::InputContext;
use crate::render_context::RenderContext;
use crate::renderable::RENDERABLES;
use crate::renderables::cube::Cube;
use crate::renderables::polygon::Polygon;
use crate::renderables::skybox::Skybox;
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
        // RENDERABLES.lock().unwrap().push(Box::new(Polygon));
        RENDERABLES.lock().unwrap().push(Box::new(Cube));
        RENDERABLES.lock().unwrap().push(Box::new(Skybox));
    }
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        self.input_context.handle_device_event(&event);
    }
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        self.input_context.handle_window_event(&event);
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                let context = self.get_context_mut();
                context.resize(physical_size);
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

                let window = self.get_context().window.clone();
                self.state.update(&mut self.input_context, window);
                // take out the render context from self
                let render_context = self.render_context.take().unwrap();

                match render_context.render(&self.state) {
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
                self.render_context = Some(render_context);
            }
            _ => (),
        }
    }
}
