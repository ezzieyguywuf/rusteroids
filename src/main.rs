use std::io::{self, Write};
use std::{time};
use crossterm::{self as xtrm, QueueableCommand};

fn main() -> xtrm::Result<()> {
    let mut stdout = io::stdout();

    // enable raw mode
    println!("Hello, world! (top)");
    xtrm::terminal::enable_raw_mode()?;

    // Queue up some things to do
    let (cols, rows) = xtrm::terminal::size()?;
    stdout.queue(xtrm::terminal::EnterAlternateScreen)?
          .queue(xtrm::style::Print("before resizing".to_string()))?
          .queue(xtrm::terminal::SetSize(10, 10))?
          .queue(xtrm::style::Print("after resizing".to_string()))?
          .queue(xtrm::cursor::MoveTo(0, 0))?
          .queue(xtrm::style::Print("After MoveTo".to_string()))?;

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
            stdout.queue(xtrm::cursor::MoveTo(0, 0))?
                  .queue(xtrm::style::Print("...waiting\n"))?;
            stdout.flush()?;
            std::thread::sleep(time::Duration::from_millis(500));
        }
    };

    stdout.queue(xtrm::terminal::SetSize(cols, rows))?
          .queue(xtrm::terminal::LeaveAlternateScreen)?;

    // disable raw mode
    xtrm::terminal::disable_raw_mode()?;
    println!("Hello, world! (bottom)");
    Ok(())
}
