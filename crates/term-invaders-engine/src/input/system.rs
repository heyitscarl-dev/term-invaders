use std::{collections::HashSet, io::Result, time::Duration};

use crossterm::event::{self, Event};

use crate::input::state::Inputs;

pub fn input_system() -> Result<Inputs> {
    let mut pressed = HashSet::new();

    while event::poll(Duration::ZERO)? {
        if let Event::Key(event) = event::read()? {
            pressed.insert(event.code);
        }
    }

    Ok(pressed)
}
