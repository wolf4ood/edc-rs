use std::{collections::VecDeque, time::Duration};

use crossterm::event;
use ratatui::{backend::Backend, Terminal};

use crate::components::{Component, ComponentEvent, ComponentMsg, GlobalMsg};

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

        let mut should_quit = false;
        loop {
            if should_quit {
                break;
            }
            terminal.draw(|frame| C::view(&mut self.model, frame, frame.size()))?;

            if event::poll(self.tick_rate)? {
                let evt = event::read()?;
                let mut msgs = C::handle_event(&self.model, ComponentEvent::Event(evt))?
                    .into_iter()
                    .collect::<VecDeque<_>>();

                while let Some(msg) = msgs.pop_front() {
                    should_quit = matches!(msg, ComponentMsg::Global(GlobalMsg::Quit));

                    let ret = C::update(&mut self.model, msg).await?;

                    for m in ret.msgs {
                        msgs.push_back(m);
                    }

                    for c in ret.cmds {
                        for m in c.await.unwrap() {
                            msgs.push_back(m);
                        }
                    }
                }
            };
        }

        Ok(())
    }
}
