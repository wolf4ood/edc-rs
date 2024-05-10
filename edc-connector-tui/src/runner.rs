use std::time::Duration;

use crossterm::event;
use ratatui::{backend::Backend, Terminal};

use crate::components::{Component, ComponentMsg, GlobalMsg};

pub struct Runner<C: Component> {
    tick_rate: Duration,
    component: C,
}

impl<C: Component + Send> Runner<C> {
    pub fn new(tick_rate: Duration, component: C) -> Self {
        Self {
            tick_rate,
            component,
        }
    }

    pub async fn run(&mut self, mut terminal: Terminal<impl Backend>) -> anyhow::Result<()> {
        terminal.clear()?;

        loop {
            terminal.draw(|frame| self.component.view(frame, frame.size()))?;
            if event::poll(self.tick_rate)? {
                let evt = event::read()?;

                if let Some(msg) = self.component.handle_event(evt)? {
                    let should_quit = matches!(msg, ComponentMsg::Global(GlobalMsg::Quit));
                    self.component.update(msg).await?;
                    if should_quit {
                        break;
                    }
                }
            };
        }

        Ok(())
    }
}
