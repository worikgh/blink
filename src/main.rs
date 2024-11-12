use std::error::Error;
use pixels::{ Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Create an event loop
    let event_loop = EventLoop::new();

    // Create a window
    let window = WindowBuilder::new()
        .with_title("Color Screen")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)?;

    // Create a pixel buffer
    let width = window.inner_size().width;
    let height = window.inner_size().height;
    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(width, height, surface_texture)?;

    // Set the color you want (RGBA format)
    let color = [0, 128, 255, 255]; // A nice shade of blue

    // Fill the pixel buffer with the chosen color
    let frame = pixels.get_frame();
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&color);
    }

    // Event loop to handle window events
    event_loop.run_return(|event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::RedrawRequested(_) => {
                // Render the pixel buffer
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    });

    Ok(())
}
