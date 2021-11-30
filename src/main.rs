use std::io::{stdout};
use crossterm as xtrm;

fn main() -> xtrm::Result<()> {
    // enable raw mode
    xtrm::terminal::enable_raw_mode()?;

    println!("Hello, world! (top)");
    println!("rawModeEnabled = {}", xtrm::terminal::is_raw_mode_enabled()?);
    let (cols, rows) = xtrm::terminal::size()?;
    // Resize terminal and scrollup
    xtrm::execute!(
        stdout(),
        xtrm::terminal::SetSize(10, 10),
        xtrm::terminal::ScrollUp(5)
    )?;

    // Be a good citizen and clean up
    xtrm::execute!(stdout(), xtrm::terminal::SetSize(cols, rows))?;
    println!("Hello, world! (bottom)");

    // disable raw mode
    xtrm::terminal::disable_raw_mode()?;
    println!("rawModeEnabled = {}", xtrm::terminal::is_raw_mode_enabled()?);
    Ok(())
}
