use crate::gui::handle_event;
use winit::event_loop::EventLoop;

mod gui;

fn main() {
    let app = gui::window::WinitAppBuilder::with_init(|elwt| {
        let window = gui::window::make_window(elwt, |w| w);

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        (window, surface)
    })
    .with_event_handler(handle_event);

    let event_loop = EventLoop::<()>::with_user_event().build().unwrap();
    let _event_loop_proxy = event_loop.create_proxy();

    std::thread::spawn(move || loop {
        let _ = _event_loop_proxy.send_event(());
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("hello!!!")
    });

    gui::window::init(event_loop, app);
}
