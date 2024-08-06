use crate::gui::handle_event;
use softbuffer::Surface;
use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};
use winit::{
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

mod gui;
mod internals;

#[derive(Debug)]
pub enum MyUserDefinedEvent {
    Quit,
}

#[derive(Default)]
pub struct Controller {
    pressed: bool,
}

fn main() {
    let event_loop = EventLoop::<MyUserDefinedEvent>::with_user_event()
        .build()
        .unwrap();
    let _event_loop_proxy = event_loop.create_proxy();

    let controller = Arc::new(RwLock::new(Controller::default()));

    let (ro_controller, wo_controller) = (Arc::clone(&controller), Arc::clone(&controller));
    std::thread::spawn(move || loop {
        let _ = _event_loop_proxy.send_event(MyUserDefinedEvent::Quit);
        std::thread::sleep(std::time::Duration::from_secs(1));
        if ro_controller.read().unwrap().pressed {
            println!("hello!!!")
        } else {
            println!("goodbye!!!")
        }
    });

    let app = gui::window::WinitAppBuilder::with_init(initalize).with_event_handler(handle_event);

    gui::window::init(event_loop, app);
}

fn initalize(elwt: &ActiveEventLoop) -> (Rc<Window>, Surface<Rc<Window>, Rc<Window>>) {
    let window = gui::window::make_window(elwt, |w| w);

    let context = softbuffer::Context::new(window.clone()).unwrap();
    let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    (window, surface)
}
