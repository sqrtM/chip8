pub mod window;

use softbuffer::Surface;
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::dpi::PhysicalSize;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::{Key, NamedKey};
use winit::window::Window;

pub fn handle_event<E>(
    state: &mut (Rc<Window>, Surface<Rc<Window>, Rc<Window>>),
    event: Event<E>,
    elwt: &ActiveEventLoop,
) {
    let (window, surface) = state;
    elwt.set_control_flow(ControlFlow::Wait);

    match event {
        Event::WindowEvent {
            window_id,
            event: WindowEvent::RedrawRequested,
        } if window_id == window.id() => {
            if let (Some(width), Some(height)) = {
                let size = PhysicalSize {
                    width: 6400,
                    height: 3200,
                };
                (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
            } {
                surface.resize(width, height).unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                for y in 0..3200 {
                    for x in 0..6400 {
                        let red = x % 255;
                        let green = y % 255;
                        let blue = (x * y) % 255;
                        let index = y as usize * width.get() as usize + x as usize;
                        buffer[index] = blue | (green << 8) | (red << 16);
                    }
                }

                buffer.present().unwrap();
            }
        }
        Event::WindowEvent {
            event:
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                    ..
                },
            window_id,
        } if window_id == window.id() => {
            elwt.exit();
        }
        _ => {}
    }
}
