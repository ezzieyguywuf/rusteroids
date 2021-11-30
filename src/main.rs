use std::io::{self, Write};
use std::{thread, time};
use crossterm as xtrm;

fn main() -> xtrm::Result<()> {
    let mut stdout = io::stdout();

    // enable raw mode
    println!("Hello, world! (top)");
    xtrm::terminal::enable_raw_mode()?;

    // Queue up some things to do
    let (cols, rows) = xtrm::terminal::size()?;
    xtrm::queue!(
        stdout,
        xtrm::terminal::EnterAlternateScreen,
        xtrm::style::Print("before resizing".to_string()),
        xtrm::terminal::SetSize(10, 10),
        xtrm::style::Print("after resizing".to_string()),
        xtrm::cursor::MoveTo(0, 0),
        xtrm::style::Print("After MoveTo".to_string()),
        // xtrm::terminal::ScrollDown(5),
        // xtrm::style::Print("<-- final location".to_string()),
    )?;

    loop {
        if xtrm::event::poll(time::Duration::from_millis(500))? {
            match xtrm::event::read()? {
                xtrm::event::Event::Key(event) => {
                    if event.code == xtrm::event::KeyCode::Char('q') {
                        break;
                    }
                },
                xtrm::event::Event::Mouse(_) => (),
                xtrm::event::Event::Resize(_, _) => (),
            }
        } else {
            xtrm::queue!(
                stdout,
                xtrm::cursor::MoveTo(0, 0),
                xtrm::style::Print("...waiting\n"),
            )?;
            stdout.flush()?;
            std::thread::sleep(time::Duration::from_millis(500));
        }
    };

    xtrm::queue!(
        stdout,
        xtrm::terminal::SetSize(cols, rows), // reset terminal
        xtrm::terminal::LeaveAlternateScreen,
    )?;

    // disable raw mode
    xtrm::terminal::disable_raw_mode()?;
    println!("Hello, world! (bottom)");
    Ok(())
}
