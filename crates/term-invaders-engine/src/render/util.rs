use std::io::{stdout, Result, Stdout};

use crossterm::{cursor, execute, terminal::{self, disable_raw_mode, enable_raw_mode, ClearType::All as ClearAll}};

// TODO(#2) - wrap preapare/cleanup in Guard

/// Prepare the terminal for raw rendering.
/// This _needs_ to be called before executing 
/// the rendering system.

pub fn prepare() -> Result<Stdout> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, terminal::Clear(ClearAll), cursor::Hide)?;
    Ok(stdout)
}

/// Revert the terminal to its original state, 
/// before calling [`prepare`]. Should be called 
/// before the application quits.

pub fn cleanup(stdout: &mut Stdout) -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    Ok(())
}
