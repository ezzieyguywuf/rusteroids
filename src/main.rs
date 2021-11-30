use std::io::{stdout};
use crossterm as xtrm;

fn main() -> xtrm::Result<()> {
    let mut stdout = stdout();

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
        xtrm::terminal::SetSize(cols, rows), // reset terminal
        xtrm::terminal::LeaveAlternateScreen,
    )?;

    // disable raw mode
    xtrm::terminal::disable_raw_mode()?;
    println!("Hello, world! (bottom)");
    Ok(())
}
