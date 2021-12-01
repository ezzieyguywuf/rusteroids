use glfw;
use std::{io, io::Write, cell};

fn error_callback(err: glfw::Error, desc: String, error_count: &cell::Cell<usize>) {
    io::stderr().write_all(format!("GLFW error {} - {}: {}\n", error_count.get(), err, desc).to_string().as_bytes()).unwrap();
    error_count.set(error_count.get() + 1);
}

fn main() {
    let err = Some(glfw::Callback{
        f: error_callback as fn(glfw::Error, String, &cell::Cell<usize>),
        data: cell::Cell::new(0)
    });
    let mut glfw_ = glfw::init(err).unwrap();

    glfw_.window_hint(glfw::WindowHint::ContextVersion(40000, 3000)); // Ridiculous!
    let _ = glfw_.create_window(300, 300, "Hey this won't work.", glfw::WindowMode::Windowed);
    let _ = glfw_.create_window(300, 300, "Nope, not working.", glfw::WindowMode::Windowed);
    let _ = glfw_.create_window(300, 300, "stop it! :(", glfw::WindowMode::Windowed);
}
