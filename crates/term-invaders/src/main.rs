use std::io::Result;
use crossterm::event::KeyCode;
use term_invaders_engine::input::{input_system, Inputs};

fn main() -> Result<()> {
    let mut stdout = term_invaders_engine::render::prepare()?;
    
    // Enter Loop 

    loop {
        // Gather Input
        let inputs: Inputs = input_system()?;

        // Apply Input
        if inputs.contains(&KeyCode::Char('q')) {
            break;
        }
    }
    
    // Cleanup
    
    term_invaders_engine::render::cleanup(&mut stdout)?;
    Ok(())
}
