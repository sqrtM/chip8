pub mod window;

use softbuffer::Surface;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::keyboard::{Key, NamedKey};
use winit::window::Window;

#[derive(Default)]
pub struct Controller {
    pub pressing: Vec<Key>,
}

pub trait UserEvent {
    fn transform(&self, b: &mut [u32]) -> ();
}

pub fn handle_event<E>(
    state: &mut (Rc<Window>, Surface<Rc<Window>, Rc<Window>>),
    event: Event<E>,
    elwt: &ActiveEventLoop,
    cont: Arc<RwLock<Controller>>,
) where
    E: UserEvent,
{
    let (window, surface) = state;
    elwt.set_control_flow(ControlFlow::Wait);

    match event {
        Event::WindowEvent {
            window_id,
            event: WindowEvent::RedrawRequested,
        } if window_id == window.id() => {
            if let (Some(width), Some(height)) = {
                let size = PhysicalSize {
                    width: 1280,
                    height: 640,
                };
                (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
            } {
                surface.resize(width, height).unwrap();
                let mut buffer = surface.buffer_mut().unwrap();
                buffer.fill(0);
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
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            logical_key: key,
                            state,
                            ..
                        },
                    ..
                },
            window_id,
        } if window_id == window.id() => match cont.try_write() {
            Ok(mut v) => {
                match state {
                    ElementState::Pressed => {
                        if !v.pressing.contains(&key) {
                            v.pressing.push(key)
                        }
                    }
                    ElementState::Released => {
                        if let Some(k) = v.pressing.iter().position(|x| x == &key) {
                            v.pressing.remove(k);
                        }
                    }
                };
            }
            Err(e) => println!("{}", e),
        },
        Event::UserEvent(e) => {
            let mut buffer = surface.buffer_mut().unwrap();
            e.transform(buffer.as_mut());
            buffer.present().unwrap();
        }
        _ => {}
    }
}
