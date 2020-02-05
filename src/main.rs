use glad_gl::gl;
use glfw::Context;

mod field;
mod figures;
mod game;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw
        .create_window(640, 500, "RGBetris", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_resizable(false);
    window.set_key_polling(true);
    window.set_size_polling(true);
    window.set_sticky_keys(true);
    window.make_current();

    gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

    unsafe {
        gl::Ortho(0.0, 640.0, 0.0, 500.0, -1.0, 1.0);
    }

    let mut game = game::Game::new();
    let mut delta_time = std::time::Duration::new(0, 0);

    while !game.is_want_exit() && !window.should_close() {
        let frame_start = std::time::SystemTime::now();

        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Size(x, y) => {
                    unsafe {
                        gl::LoadIdentity();
                        gl::Viewport(0, 0, x, y);
                        gl::Ortho(0.0, x as f64, 0.0, y as f64, -1.0, 1.0);
                    }
                }

                _ => { game.handle_event(event); }
            }
        }

        game.tick(delta_time);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::GL_COLOR_BUFFER_BIT);
        }

        game.draw(&window);

        window.swap_buffers();
        delta_time = frame_start.elapsed().unwrap();
    }
}
