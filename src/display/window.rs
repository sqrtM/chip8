use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 64 * 10;
const HEIGHT: usize = 32 * 10;

pub fn init() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window =
        Window::new("WOWO", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 999;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
