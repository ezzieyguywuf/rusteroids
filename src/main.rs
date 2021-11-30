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
    let mut x = 0;
    let mut y = 0;
    stdout.queue(xtrm::terminal::EnterAlternateScreen)?
          .queue(xtrm::cursor::Hide)?
          .queue(xtrm::style::Print("before resizing".to_string()))?
          .queue(xtrm::terminal::SetSize(1000, 1000))?
          .queue(xtrm::style::Print("after resizing".to_string()))?
          .queue(xtrm::cursor::MoveTo(0, 1))?
          .queue(xtrm::style::Print("After MoveTo".to_string()))?;

    loop {
        if xtrm::event::poll(time::Duration::from_millis(1))? {
            match xtrm::event::read()? {
                xtrm::event::Event::Key(event) => {
                    if event.code == xtrm::event::KeyCode::Char('q') {
                        break;
                    } else if event.code == xtrm::event::KeyCode::Up {
                        stdout.queue(xtrm::cursor::MoveTo(x, y))?.queue(xtrm::style::Print(" "))?;
                        y -= 1;
                    } else if event.code == xtrm::event::KeyCode::Down {
                        stdout.queue(xtrm::cursor::MoveTo(x, y))?.queue(xtrm::style::Print(" "))?;
                        y += 1;
                    } else if event.code == xtrm::event::KeyCode::Left {
                        stdout.queue(xtrm::cursor::MoveTo(x, y))?.queue(xtrm::style::Print(" "))?;
                        x -= 1;
                    } else if event.code == xtrm::event::KeyCode::Right {
                        stdout.queue(xtrm::cursor::MoveTo(x, y))?.queue(xtrm::style::Print(" "))?;
                        x += 1;
                    }
                },
                xtrm::event::Event::Mouse(_) => (),
                xtrm::event::Event::Resize(_, _) => (),
            }
        } else {
            stdout.queue(xtrm::cursor::MoveTo(0, 2))?
                  .queue(xtrm::style::Print("...waiting\n"))?;
        }
        stdout.queue(xtrm::cursor::MoveTo(x, y))?
              .queue(xtrm::style::Print("."))?;
        stdout.flush()?;
        std::thread::sleep(time::Duration::from_millis(1));
    };

    stdout.queue(xtrm::terminal::SetSize(cols, rows))?
          .queue(xtrm::cursor::Show)?
          .queue(xtrm::terminal::LeaveAlternateScreen)?;

    // disable raw mode
    xtrm::terminal::disable_raw_mode()?;
    println!("Hello, world! (bottom)");
    Ok(())
}
