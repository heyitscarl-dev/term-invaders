use std::io::{stdout, Result, Stdout};

use crossterm::{execute, cursor, terminal::{self, *}};
use crossterm::terminal::ClearType::All as ClearAll;

pub fn prepare() -> Result<Stdout> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, terminal::Clear(ClearAll), cursor::Hide)?;
    Ok(stdout)
}

pub fn cleanup(stdout: &mut Stdout) -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    Ok(())
}
