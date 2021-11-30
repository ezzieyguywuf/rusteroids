use std::io::{stdout};
use crossterm as xtrm;

fn main() -> xtrm::Result<()> {
    println!("Hello, world! (top)");
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
    Ok(())
}
