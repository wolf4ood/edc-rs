use std::time::Duration;
mod model;
mod msg;
mod update;
mod view;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Frame, Terminal};

use self::{model::{Model, RunningState}, msg::Message};

pub struct App {
    tick_rate: Duration,
    model: Model,
}

impl App {
    pub fn new() -> Self {
        Self {
            model: Model::default(),
            tick_rate: Duration::from_millis(250),
        }
    }

    pub async fn run(&mut self, mut terminal: Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.clear()?;

        while self.model.running_state() != &RunningState::Done {
            self.draw(&mut terminal)?;
            let msg = self.handle_event()?;
            if let Some(m) = msg {
                self.update(m);
            }
        }

        Ok(())
    }

    fn ui(&mut self, frame: &mut Frame) {
        self.view(frame);
    }

    pub fn handle_event(&self) -> std::io::Result<Option<Message>> {
        if event::poll(self.tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    return Ok(self.handle_key(key));
                }
            }
        }
        Ok(None)
    }

    fn handle_key(&self, key: event::KeyEvent) -> Option<Message> {
        match key.code {
            KeyCode::Char('j') => Some(Message::Increment),
            KeyCode::Char('k') => Some(Message::Decrement),
            KeyCode::Char('q') => Some(Message::Quit),
            _ => None,
        }
    }

    pub fn draw(&mut self, terminal: &mut Terminal<impl Backend>) -> std::io::Result<()> {
        terminal.draw(|frame| self.ui(frame))?;

        Ok(())
    }
}
