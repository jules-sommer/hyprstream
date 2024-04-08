use crate::events::{self, Event};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventInterpretError {
  #[error("Failed to interpret event: {0}")]
  InterpretationError(String),
}

pub struct Interpreter<'a> {
  pub raw: &'a str,
}

impl<'b> Interpreter<'b> {
  pub fn new(raw: &'b str) -> Self {
    Interpreter { raw }
  }

  pub fn interpret(&self) -> Result<Event, EventInterpretError> {
    let (event_type, data) = self.raw.split_once(">>").ok_or_else(|| {
      EventInterpretError::InterpretationError(format!("Invalid event format: {}", self.raw))
    })?;

    Ok(Event::from(event_type, data))
  }
}
