//! Simple winit application.
use winit::raw_window_handle::{HasDisplayHandle, WaylandDisplayHandle};
use winit::event_loop::EventLoop;
use std::error::Error;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use winit::event_loop::ControlFlow;

#[derive(Default)]
struct App {
    window: Option<Window>,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.
		if let Some(rd_window) = self.window.as_ref(){
		    if let Ok(h) = rd_window.display_handle(){
			let rh = h.as_raw();
			match rh {
			    winit::raw_window_handle::RawDisplayHandle::Wayland(WaylandDisplayHandle{display: wh, ..}) => {
				let t = wh.as_ref();
				
			    },
			    _ => panic!("Not implemented"),
			}
		    }
		}
		    
                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();
    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    // event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    // event_loop.set_control_flow(ControlFlow::WaitUntil();

    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
