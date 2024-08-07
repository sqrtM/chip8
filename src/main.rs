use crate::gui::{handle_event, Controller};
use internals::DisplayCommand;
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

fn main() {
    let event_loop = EventLoop::<DisplayCommand>::with_user_event()
        .build()
        .unwrap();
    let _event_loop_proxy = event_loop.create_proxy();

    let controller = Arc::new(RwLock::new(Controller::default()));
    let (ro_controller, wo_controller) = (Arc::clone(&controller), Arc::clone(&controller));

    std::thread::spawn(move || loop {
        //let _ = _event_loop_proxy.send_event(DisplayCommand::Quit);
        std::thread::sleep(std::time::Duration::from_secs(1));
        match ro_controller.try_read() {
            Ok(v) => println!("{:?}", v.pressing),
            Err(e) => println!("{}", e),
        }
    });

    let app = gui::window::WinitAppBuilder::with_init(initalize)
        .with_event_handler(handle_event, wo_controller);

    gui::window::init(event_loop, app);
}

fn initalize(elwt: &ActiveEventLoop) -> (Rc<Window>, Surface<Rc<Window>, Rc<Window>>) {
    let window = gui::window::make_window(elwt, |w| w);

    let context = softbuffer::Context::new(window.clone()).unwrap();
    let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    (window, surface)
}
