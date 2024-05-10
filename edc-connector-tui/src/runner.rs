use std::time::Duration;

use crossterm::event;
use ratatui::{backend::Backend, Terminal};

use crate::components::{Component, ComponentMsg, GlobalMsg};

pub struct Runner<C: Component> {
    tick_rate: Duration,
    model: C::Model,
}

impl<C: Component + Send> Runner<C> {
    pub fn new(tick_rate: Duration, model: C::Model) -> Self {
        Self { tick_rate, model }
    }

    pub async fn run(&mut self, mut terminal: Terminal<impl Backend>) -> anyhow::Result<()> {
        terminal.clear()?;

        loop {
            terminal.draw(|frame| C::view(&mut self.model, frame, frame.size()))?;

            if event::poll(self.tick_rate)? {
                let evt = event::read()?;

                if let Some(msg) = C::handle_event(&self.model, evt)? {
                    let should_quit = matches!(msg, ComponentMsg::Global(GlobalMsg::Quit));
                    C::update(&mut self.model, msg).await?;
                    if should_quit {
                        break;
                    }
                }
            };
        }

        Ok(())
    }
}
