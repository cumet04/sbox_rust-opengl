extern crate glfw;
use self::glfw::{Action, Context, Key};

use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: glfw::Glfw,
    window: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        // glfw: initialize and configure
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        // glfw window creation
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // gl: load all OpenGL function pointers
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Window {
            glfw: glfw,
            window: window,
            events: events,
        }
    }

    pub fn render_loop<F>(&mut self, f: F)
    where
        F: Fn(),
    {
        while !self.window.should_close() {
            // events
            for (_, event) in glfw::flush_messages(&self.events) {
                match event {
                    glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                        gl::Viewport(0, 0, width, height)
                    },
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    }
                    _ => {}
                }
            }

            f();

            // glfw: swap buffers and poll IO events
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }
}
