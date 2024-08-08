use crate::gui::{handle_event, Controller};
use internals::{Chip8, DisplayCommand};
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

    std::thread::spawn(move || {
        let mut chip8 = Chip8::new(ro_controller);
        chip8.memory.load("./data/2-ibm-logo.ch8");
        //chip8.memory.load("./data/3-corax+.ch8");

        loop {
            let inst = internals::parse_opcode(
                ((chip8.memory.0[chip8.registers.pc as usize] as u16) << 8)
                    | chip8.memory.0[chip8.registers.pc as usize + 1] as u16,
            );
            println!(
                "{:?} ; {:2X}-{:2X} ; {:X?}",
                inst,
                (chip8.memory.0[chip8.registers.pc as usize] << 4) as u16,
                chip8.memory.0[chip8.registers.pc as usize + 1] as u16,
                chip8.registers.pc
            );
            match chip8.run_instruction(inst) {
                Ok(s) => match s {
                    internals::InstructionResult::Success => (),
                    internals::InstructionResult::Display(d) => {
                        match _event_loop_proxy.send_event(d) {
                            Ok(()) => (),
                            Err(..) => println!("ERR: Event loop Closed !"),
                        }
                    }
                },
                Err(e) => println!("{:?}", e),
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
            match chip8.controller.try_read() {
                Ok(v) => println!("{:?}", v.pressing),
                Err(e) => println!("{}", e),
            }
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
