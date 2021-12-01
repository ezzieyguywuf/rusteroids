use glfw::{self, Context};
use std::{io, io::Write, cell};

fn error_callback(err: glfw::Error, desc: String, error_count: &cell::Cell<usize>) {
    io::stderr().write_all(
        format!("GLFW error {} - {}: {}\n", error_count.get(), err, desc)
        .to_string().as_bytes()).unwrap();
    error_count.set(error_count.get() + 1);
}

fn main() -> io::Result<()> {
    let err = Some(glfw::Callback{
        f: error_callback as fn(glfw::Error, String, &cell::Cell<usize>),
        data: cell::Cell::new(0)
    });
    let mut glfw_ = glfw::init(err).unwrap();

    // glfw_.window_hint(glfw::WindowHint::ContextVersion(40000, 3000)); // Ridiculous!
    let (mut window, events) =
        glfw_
        .create_window(300, 300, "Hello rusteroids!", glfw::WindowMode::Windowed)
        .expect("Failed to create glfw window");

    window.make_current();
    window.set_key_polling(true);

    while !window.should_close() {
        window.swap_buffers();

        glfw_.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                },
                _ => {},
            }
        }
    }

    Ok(())
}
