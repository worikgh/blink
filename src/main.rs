use std::thread;
use std::env;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Duration;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::Window;

mod winit_app;

fn main() {
    let mut args = env::args();
    let (r, g, b) = if args.len() == 4 {
	_ = args.next(); // Programme name
	let r = args.next().unwrap().parse::<u8>().expect("Red");
	let g = args.next().unwrap().parse::<u8>().expect("Green");
	let b = args.next().unwrap().parse::<u8>().expect("Blue");
	(r, g, b)
    }else{
	(0, 0, 0)
    };
    let event_loop = EventLoop::new().unwrap();
    let mut app = winit_app::WinitAppBuilder::with_init(|elwt: &ActiveEventLoop| {
        let window = {
            let mut attribs = Window::default_attributes().clone();
            attribs.maximized = true;
            attribs.decorations = false;
            let window = elwt.create_window(attribs);
            Rc::new(window.unwrap())
        };
        let surface = softbuffer::Surface::new(&softbuffer::Context::new(window.clone()).unwrap(), window.clone()).unwrap();
        (window, surface)
    })
    .with_event_handler(move |state, event, elwt| {
        let (window, surface) = state;
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                for index in 0..(width * height) {
                    let red = r as u32;
                    let green = g as u32;
                    let blue = b as u32;

                    buffer[index as usize] = blue | (green << 8) | (red << 16);
                }

                buffer.present().unwrap();
            }
            Event::AboutToWait => {
		thread::sleep(Duration::from_millis(50));
                elwt.exit();
            }
            x => {
                eprintln!("DBG Event: {x:?}");
            }
        }
    });

    event_loop.run_app(&mut app).unwrap();
    eprintln!("Finished");
}
