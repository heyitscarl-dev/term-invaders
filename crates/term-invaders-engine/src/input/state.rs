// In the future, this module will contain a 
// wrapper data structure around a HashSet / -Map,
// providing utility functions for input handling.

use std::collections::HashSet;
use crossterm::event::KeyCode;

pub type Inputs = HashSet<KeyCode>;
